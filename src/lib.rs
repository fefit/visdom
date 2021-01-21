use ntree::selector::interface::{
	BoxDynNode, IAttrValue, IDocumentTrait, INodeTrait, INodeType, InsertPosition, MaybeDocResult,
	MaybeResult, NodeList, Result,
};
use ntree::{self, utils::retain_by_index};
use rphtml::{
	config::{ParseOptions, RenderOptions},
	parser::{allow_insert, Attr, AttrData, CodePosAt, Doc, Node, NodeType, RefNode, RootNode},
};
use std::rc::Rc;
use std::{any::Any, cell::RefCell};
/// type implement INodeTrait with Node
struct Dom {
	node: Rc<RefCell<Node>>,
}
impl Dom {
	fn validate_dom_change(&self, node: &BoxDynNode, method: &str) {
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
		while let Some(parent) = &cur.parent().unwrap_or(None) {
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

impl INodeTrait for Dom {
	fn to_node(self: Box<Self>) -> Box<dyn Any> {
		self
	}
	/// impl `cloned`
	fn cloned<'b>(&self) -> BoxDynNode<'b> {
		Box::new(Dom {
			node: self.node.clone(),
		})
	}
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
			INodeType::Document | INodeType::AbstractRoot => "",
			cur_type => panic!("The node type of '{:?}' doesn't have a tag name.", cur_type),
		}
	}
	/// impl `node_type`
	fn node_type(&self) -> INodeType {
		let node = self.node.borrow();
		match node.node_type {
			NodeType::AbstractRoot => {
				let (is_document, _) = node.is_document();
				if is_document {
					INodeType::Document
				} else {
					INodeType::AbstractRoot
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
	fn parent<'b>(&self) -> MaybeResult<'b> {
		if let Some(parent) = &self.node.borrow().parent {
			if let Some(node) = parent.upgrade() {
				let cur = Dom { node };
				return Ok(Some(Box::new(cur)));
			} else {
				return Err("");
			}
		}
		Ok(None)
	}
	/// impl `children`
	fn child_nodes<'b>(&self) -> Result<'b> {
		if let Some(childs) = &self.node.borrow().childs {
			let mut result = NodeList::with_capacity(childs.len());
			let nodes = result.get_mut_ref();
			for cur in childs {
				nodes.push(Box::new(Dom { node: cur.clone() }));
			}
			return Ok(result);
		}
		Ok(NodeList::new())
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
	/// impl `uuid`
	fn uuid(&self) -> Option<&str> {
		if let Some(uuid) = &self.node.borrow().uuid {
			return Some(to_static_str(uuid.clone()));
		}
		None
	}

	/// impl `owner_document`
	fn owner_document(&self) -> MaybeDocResult {
		if let Some(root) = &self.node.borrow().root {
			Ok(Some(Box::new(Document {
				root: Rc::clone(root),
			})))
		} else {
			Err("")
		}
	}
	/// impl `text_content`
	fn text_content(&self) -> &str {
		to_static_str(self.node.borrow().build(
			&RenderOptions {
				decode_entity: true,
				..Default::default()
			},
			true,
		))
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
	fn remove_child(&mut self, node: BoxDynNode) {
		if let Some(parent) = &node.parent().unwrap_or(None) {
			if self.is(parent) {
				// is a child
				if let Some(childs) = self.node.borrow_mut().childs.as_mut() {
					let mut find_index: Option<usize> = None;
					for (index, child) in childs.iter().enumerate() {
						let dom = Box::new(Dom {
							node: Rc::clone(child),
						}) as BoxDynNode;
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
	fn insert_adjacent(&mut self, position: &InsertPosition, node: &BoxDynNode) {
		// base validate
		let action = position.action();
		self.validate_dom_change(&node, action);
		let orig_node = node.cloned();
		let node_type = node.node_type();
		let specified: Box<dyn Any> = node.cloned().to_node();
		if let Ok(dom) = specified.downcast::<Dom>() {
			// remove current node from parent's childs
			if let Ok(Some(parent)) = &mut orig_node.parent() {
				parent.remove_child(orig_node);
			}
			// get the nodes
			let mut nodes = match node_type {
				INodeType::AbstractRoot => {
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
								let mut find_index: Option<usize> = None;
								for (index, sibling) in siblings.iter().enumerate() {
									if Node::is_same(&self.node, sibling) {
										find_index = Some(index);
									}
								}
								if let Some(mut index) = find_index {
									// insert
									if *position == AfterEnd {
										index += 1;
									}
									siblings.splice(index..index, nodes);
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
}

struct Document {
	root: Rc<RefCell<RootNode>>,
}
impl IDocumentTrait for Document {
	fn get_element_by_id<'b>(&self, id: &str) -> Option<BoxDynNode<'b>> {
		if let Some(node) = self.root.borrow().get_element_by_id(id) {
			return Some(Box::new(Dom {
				node: Rc::clone(&node),
			}));
		}
		None
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
	pub(crate) fn init() {
		ntree::init();
	}
	pub fn load(html: &str) -> Result {
		// init
		Vis::init();
		// nodes
		let doc = Doc::parse(
			html,
			ParseOptions {
				auto_remove_nostart_endtag: true,
				..Default::default()
			},
		)
		.map_err(|_| "")?;
		let root: Dom = Rc::clone(&doc.get_root_node()).into();
		Ok(NodeList::with_nodes(vec![Box::new(root)]))
	}
}
