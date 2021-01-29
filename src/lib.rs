use mesdoc::interface::{
	BoxDynElement, BoxDynNode, BoxDynText, BoxDynUncareNode, Elements, IAttrValue, IDocumentTrait,
	IElementTrait, IEnumTyped, IErrorHandle, INodeTrait, INodeType, ITextTrait, IUncareNodeTrait,
	InsertPosition, MaybeDoc, MaybeElement, Texts,
};
use mesdoc::{self, utils::retain_by_index};
use rphtml::{
	config::{ParseOptions, RenderOptions},
	entity::{encode, EncodeType::NamedOrDecimal, EntitySet::SpecialChars},
	parser::{allow_insert, Attr, AttrData, CodePosAt, Doc, Node, NodeType, RefNode, RootNode},
};
use std::error::Error;
use std::rc::Rc;
use std::{any::Any, cell::RefCell};
/// type implement INodeTrait with Node
struct Dom {
	node: Rc<RefCell<Node>>,
}
impl Dom {
	fn validate_dom_change(&self, node: &BoxDynElement, method: &str) {
		// test if current node is element node
		let my_node_type = self.node.borrow().node_type;
		if my_node_type != NodeType::Tag {
			panic!("Can't {} for a {:?} type", method, my_node_type);
		}
		// document
		if let INodeType::Document = node.node_type() {
			panic!("Can't {} of a document type", method);
		}
		// test if same node
		if self.is(&node) {
			panic!("Can't {} of self.", method);
		}
		// test if the node is self's parent node
		let mut cur = self.cloned();
		while let Some(parent) = &cur.parent() {
			if parent.is(&node) {
				panic!("Can't {} of self's parent", method);
			}
			cur = parent.cloned();
		}
	}
}

fn to_static_str(orig: String) -> &'static str {
	Box::leak(orig.into_boxed_str())
}

// get current node's index and do with the sibling nodes
fn get_index_then_do(cur: &RefNode, siblings: &[RefNode]) -> Option<usize> {
	let mut find_index: Option<usize> = None;
	for (index, node) in siblings.iter().enumerate() {
		if Node::is_same(cur, node) {
			find_index = Some(index);
			break;
		}
	}
	find_index
}

impl INodeTrait for Dom {
	/// impl `to_node`
	fn to_node(self: Box<Self>) -> Box<dyn Any> {
		self
	}

	/// impl `clone_node`
	fn clone_node<'b>(&self) -> BoxDynNode<'b> {
		Box::new(Dom {
			node: self.node.clone(),
		})
	}

	/// impl `typed`
	fn typed<'b>(self: Box<Self>) -> IEnumTyped<'b> {
		match self.node_type() {
			INodeType::Element | INodeType::DocumentFragement => {
				IEnumTyped::Element(self as BoxDynElement)
			}
			INodeType::Text => IEnumTyped::Text(self as BoxDynText),
			_ => IEnumTyped::UncareNode(self as BoxDynUncareNode),
		}
	}

	/// impl `node_type`
	fn node_type(&self) -> INodeType {
		let node_type = self.node.borrow().node_type;
		match node_type {
			NodeType::AbstractRoot => {
				let (is_document, _) = self.node.borrow().is_document();
				if is_document {
					INodeType::Document
				} else {
					INodeType::DocumentFragement
				}
			}
			NodeType::Comment => INodeType::Comment,
			NodeType::Text | NodeType::SpacesBetweenTag => INodeType::Text,
			NodeType::Tag => INodeType::Element,
			NodeType::XMLCDATA => INodeType::XMLCDATA,
			NodeType::HTMLDOCTYPE => INodeType::HTMLDOCTYPE,
			_ => INodeType::Other,
		}
	}
	/// impl `parent`
	fn parent<'b>(&self) -> MaybeElement<'b> {
		if let Some(parent) = &self.node.borrow().parent {
			if let Some(node) = parent.upgrade() {
				let cur = Dom { node };
				return Some(Box::new(cur));
			}
		}
		None
	}

	/// impl `uuid`
	fn uuid(&self) -> Option<&str> {
		if let Some(uuid) = &self.node.borrow().uuid {
			return Some(to_static_str(uuid.clone()));
		}
		None
	}

	/// impl `owner_document`
	fn owner_document(&self) -> MaybeDoc {
		if let Some(root) = &self.node.borrow().root {
			Some(Box::new(Document {
				node: Rc::clone(root),
			}))
		} else {
			None
		}
	}
	/// impl `text_content`
	fn text_content(&self) -> &str {
		to_static_str(self.node.borrow().build(
			&RenderOptions {
				decode_entity: true,
				..Default::default()
			},
			matches!(self.node_type(), INodeType::Element),
		))
	}

	/// impl `set_text`
	fn set_text(&mut self, content: &str) {
		let node_type = self.node_type();
		let mut node = self.node.borrow_mut();
		match node_type {
			INodeType::Element => {
				if content.is_empty() {
					node.childs = None;
				} else {
					let content = encode(content, SpecialChars, NamedOrDecimal);
					let text_node = Node::create_text_node(&content, None);
					node.childs = Some(vec![Rc::new(RefCell::new(text_node))]);
				}
			}
			INodeType::Text => {
				if content.is_empty() {
					if let Some(parent) = &self.node.borrow_mut().parent {
						if let Some(parent) = parent.upgrade() {
							if let Some(siblings) = &mut parent.borrow_mut().childs {
								// remove the node
								if let Some(index) = get_index_then_do(&self.node, siblings) {
									siblings.remove(index);
								}
							}
						}
					}
				} else {
					// replace the text node
					let text_node = Node::create_text_node(&content, None);
					*node = text_node;
				}
			}
			_ => {
				// other node types nothing to do
			}
		}
	}

	/// impl `set_html`
	fn set_html(&mut self, content: &str) {
		let doc = Doc::parse(
			content,
			ParseOptions {
				auto_remove_nostart_endtag: true,
				..Default::default()
			},
		)
		.unwrap();
		if let Some(childs) = &mut doc.get_root_node().borrow_mut().childs {
			// set childs with new childs
			match self.node_type() {
				INodeType::Element => self.node.borrow_mut().childs = Some(childs.split_off(0)),
				INodeType::Text => {
					if let Some(parent) = &self.node.borrow_mut().parent {
						if let Some(parent) = parent.upgrade() {
							if let Some(siblings) = &mut parent.borrow_mut().childs {
								if let Some(index) = get_index_then_do(&self.node, siblings) {
									// delete the node and append childs
									siblings.splice(index..index + 1, childs.split_off(0));
								}
							}
						}
					}
				}
				_ => {
					// nothing to do with other nodes
				}
			};
		} else {
			// remove the childs
			self.node.borrow_mut().childs = None;
		}
	}
}

impl ITextTrait for Dom {}

impl IUncareNodeTrait for Dom {}

impl IElementTrait for Dom {
	/// impl `tag_name`
	fn tag_name(&self) -> &str {
		match self.node_type() {
			INodeType::Element => {
				if let Some(meta) = &self.node.borrow().meta {
					let name = meta.borrow().get_name(false);
					return to_static_str(name);
				}
				panic!("Html syntax error: not found a tag name.");
			}
			INodeType::Document | INodeType::DocumentFragement => "",
			cur_type => panic!("The node type of '{:?}' doesn't have a tag name.", cur_type),
		}
	}

	/// impl `children`
	fn child_nodes<'b>(&self) -> Vec<BoxDynNode<'b>> {
		if let Some(childs) = &self.node.borrow().childs {
			let mut result = Vec::with_capacity(childs.len());
			for cur in childs {
				result.push(Box::new(Dom { node: cur.clone() }) as BoxDynNode);
			}
			return result;
		}
		vec![]
	}
	/// impl `get_attribute`
	fn get_attribute(&self, name: &str) -> Option<IAttrValue> {
		if let Some(meta) = &self.node.borrow().meta {
			for attr in &meta.borrow().attrs {
				if let Some(key) = &attr.key {
					if key.content == name {
						if let Some(value) = &attr.value {
							let attr_value = value.content.clone();
							return Some(IAttrValue::Value(attr_value, attr.quote));
						} else {
							return Some(IAttrValue::True);
						}
					}
				}
			}
		}
		None
	}
	/// impl `set_attribute`
	fn set_attribute(&mut self, name: &str, value: Option<&str>) {
		let mut need_quote = false;
		let mut quote: char = '"';
		let pos = CodePosAt::default();
		if let Some(meta) = &self.node.borrow().meta {
			let value = value.map(|v| {
				let mut find_quote: bool = false;
				let mut content = String::with_capacity(v.len());
				// loop the chars
				for ch in v.chars() {
					if !need_quote {
						need_quote = Attr::need_quoted_char(&ch);
					}
					if ch == '"' || ch == '\'' {
						if find_quote {
							if quote == ch {
								// find more quotes
								content.push('\\');
							}
						} else {
							// if first is double quote, change the variable `quote` to single quote
							find_quote = true;
							if ch == '"' {
								quote = '\'';
							}
						}
					}
					content.push(ch);
				}
				AttrData {
					content,
					begin_at: pos,
					end_at: pos,
				}
			});
			// first, check if the attribute has exist.
			for attr in &mut meta.borrow_mut().attrs {
				if let Some(key) = &attr.key {
					if key.content == name {
						// find the attribute
						attr.value = value;
						return;
					}
				}
			}
			// new attribute, add it to queue.
			let quote = if value.is_some() { Some(quote) } else { None };
			meta.borrow_mut().attrs.push(Attr {
				key: Some(AttrData {
					content: name.into(),
					begin_at: pos,
					end_at: pos,
				}),
				value,
				quote,
				need_quote,
			});
		}
	}
	/// impl `remove_attribute`
	fn remove_attribute(&mut self, name: &str) {
		if let Some(meta) = &self.node.borrow().meta {
			let mut find_index: Option<usize> = None;
			for (index, attr) in meta.borrow().attrs.iter().enumerate() {
				if let Some(key) = &attr.key {
					if key.content == name {
						find_index = Some(index);
						break;
					}
				}
			}
			if let Some(index) = find_index {
				meta.borrow_mut().attrs.remove(index);
			}
		}
	}

	/// impl `inner_html`
	fn inner_html(&self) -> &str {
		to_static_str(self.node.borrow().build(
			&RenderOptions {
				inner_html: true,
				..Default::default()
			},
			false,
		))
	}

	/// impl `outer_html`
	fn outer_html(&self) -> &str {
		to_static_str(self.node.borrow().build(&Default::default(), false))
	}

	/// impl `remov_child`
	fn remove_child(&mut self, node: BoxDynElement) {
		if let Some(parent) = &node.parent() {
			if self.is(parent) {
				// is a child
				if let Some(childs) = self.node.borrow_mut().childs.as_mut() {
					let mut find_index: Option<usize> = None;
					for (index, child) in childs.iter().enumerate() {
						let dom = Box::new(Dom {
							node: Rc::clone(child),
						}) as BoxDynElement;
						if node.is(&dom) {
							find_index = Some(index);
						}
					}
					if let Some(index) = find_index {
						childs.remove(index);
					}
				}
			}
		}
	}
	// append child
	fn insert_adjacent(&mut self, position: &InsertPosition, node: &BoxDynElement) {
		// base validate
		let action = position.action();
		self.validate_dom_change(&node, action);
		let orig_node = node.cloned();
		let node_type = node.node_type();
		let specified: Box<dyn Any> = node.cloned().to_node();
		if let Ok(dom) = specified.downcast::<Dom>() {
			// remove current node from parent's childs
			if let Some(parent) = &mut orig_node.parent() {
				parent.remove_child(orig_node);
			}
			// get the nodes
			let mut nodes = match node_type {
				INodeType::DocumentFragement => {
					if let Some(childs) = &dom.node.borrow().childs {
						childs
							.iter()
							.map(|v| Rc::clone(&v))
							.collect::<Vec<RefNode>>()
					} else {
						vec![]
					}
				}
				_ => {
					vec![dom.node]
				}
			};
			// filter the node allowed
			let tag_name = self.tag_name();
			let mut not_allowed_indexs: Vec<usize> = Vec::with_capacity(nodes.len());
			for (index, node) in nodes.iter().enumerate() {
				if !allow_insert(tag_name, node.borrow().node_type) {
					not_allowed_indexs.push(index);
				}
			}
			let not_allowed_num = not_allowed_indexs.len();
			if not_allowed_num == nodes.len() {
				return;
			}
			if not_allowed_num > 0 {
				retain_by_index(&mut nodes, &not_allowed_indexs);
			}
			// insert
			use InsertPosition::*;
			match position {
				BeforeBegin | AfterEnd => {
					// insertBefore,insertAfter
					if let Some(parent) = &self.node.borrow_mut().parent {
						if let Some(parent) = parent.upgrade() {
							if let Some(siblings) = &mut parent.borrow_mut().childs {
								if let Some(mut index) = get_index_then_do(&self.node, siblings) {
									// delete the node and append childs
									if *position == AfterEnd {
										index += 1;
									}
									siblings.splice(index..index, (&mut nodes).split_off(0));
								}
							}
						}
					}
				}
				AfterBegin | BeforeEnd => {
					// prepend, append
					if let Some(childs) = &mut self.node.borrow_mut().childs {
						if *position == BeforeEnd {
							childs.extend(nodes);
						} else {
							nodes.append(childs);
							*childs = nodes;
						}
					} else {
						self.node.borrow_mut().childs = Some(nodes);
					}
				}
			}
		} else {
			// not the Dom
			panic!("Can't {} that not implemented 'Dom'", action);
		}
	}
	/// impl `texts`
	fn texts<'b>(&self, limit_depth: u32) -> Option<Texts<'b>> {
		let limit_depth = if limit_depth == 0 {
			u32::MAX
		} else {
			limit_depth
		};
		let mut result: Texts = Texts::with_capacity(5);
		fn loop_handle(node: BoxDynElement, result: &mut Texts, cur_depth: u32, limit_depth: u32) {
			let child_nodes = node.child_nodes();
			if !child_nodes.is_empty() {
				let next_depth = cur_depth + 1;
				let recursive = next_depth < limit_depth;
				for node in &node.child_nodes() {
					match node.node_type() {
						INodeType::Text => {
							// append text node to result
							let node = node.clone_node();
							let text = node.typed().into_text().expect("TextNode must true");
							result.get_mut_ref().push(text);
						}
						INodeType::Element => {
							// if need recursive find the text node
							if recursive {
								let node = node.clone_node();
								let ele = node.typed().into_element().expect("TextNode must true");
								loop_handle(ele, result, next_depth, limit_depth);
							}
						}
						_ => {}
					}
				}
			}
		}
		let node = Box::new(Dom {
			node: Rc::clone(&self.node),
		}) as BoxDynElement;
		loop_handle(node, &mut result, 0, limit_depth);
		if !result.is_empty() {
			return Some(result);
		}
		None
	}
}

struct Document {
	node: Rc<RefCell<RootNode>>,
}

impl Document {
	fn bind_error(&mut self, handle: IErrorHandle) {
		*self.node.borrow().onerror.borrow_mut() = Some(Rc::new(handle));
	}
	fn list<'b>(&self) -> Elements<'b> {
		let root: Dom = Rc::clone(&self.node.borrow().get_node()).into();
		Elements::with_nodes(vec![Box::new(root)])
	}
}

impl IDocumentTrait for Document {
	fn get_element_by_id<'b>(&self, id: &str) -> Option<BoxDynElement<'b>> {
		if let Some(node) = self.node.borrow().get_element_by_id(id) {
			return Some(Box::new(Dom {
				node: Rc::clone(&node),
			}));
		}
		None
	}
	fn onerror(&self) -> Option<Rc<IErrorHandle>> {
		if let Some(error_handle) = &(*self.node.borrow().onerror.borrow()) {
			Some(Rc::clone(error_handle))
		} else {
			None
		}
	}
}

impl From<Rc<RefCell<Node>>> for Dom {
	fn from(node: Rc<RefCell<Node>>) -> Self {
		Dom {
			node: Rc::clone(&node),
		}
	}
}

pub struct Vis;

impl Vis {
	// init the patterns and all
	pub(crate) fn parse_doc(html: &str) -> Result<Document, Box<dyn Error>> {
		mesdoc::init();
		let doc = Doc::parse(
			html,
			ParseOptions {
				auto_remove_nostart_endtag: true,
				..Default::default()
			},
		)?;
		Ok(Document {
			node: Rc::clone(&doc.root),
		})
	}
	// load the html
	pub fn load(html: &str) -> Result<Elements, Box<dyn Error>> {
		// nodes
		let doc = Vis::parse_doc(html)?;
		Ok(doc.list())
	}
	// load the html, and catch the errors
	pub fn load_catch(html: &str, handle: IErrorHandle) -> Elements {
		let doc = Vis::parse_doc(html);
		if let Ok(mut doc) = doc {
			doc.bind_error(handle);
			doc.list()
		} else {
			handle(doc.err().unwrap());
			Elements::new()
		}
	}
	// return a Elements
	pub fn dom<'b>(node: &BoxDynElement) -> Elements<'b> {
		Elements::with_nodes(vec![node.cloned()])
	}
}
