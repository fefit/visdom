//! # Visdom
//!
//! ## Description
//! A library use jquery-like api to operate html, easy to use for scraping and poisoning html.
//!
//! ## Supported
//! - Support most css selectors, e.g. `:nth-child`, `:nth-of-type`, `:not` and so on.
//! - Support most selector methods, such as `find`,`filter`,`has`, `not`
//! - Support powerful text operation ability: `set_html`, `set_text`, `append_text` can used for text node.
use mesdoc::interface::{
	BoxDynElement, BoxDynNode, BoxDynText, BoxDynUncareNode, Elements, IAttrValue, IDocumentTrait,
	IElementTrait, IEnumTyped, IErrorHandle, INodeTrait, INodeType, ITextTrait, IUncareNodeTrait,
	InsertPosition, MaybeDoc, MaybeElement, Texts,
};
use mesdoc::{self, error::Error as IError, utils::retain_by_index};
use rphtml::{
	config::{ParseOptions, RenderOptions},
	entity::{encode, EncodeType::NamedOrDecimal, EntitySet::SpecialChars},
	parser::{
		allow_insert, is_content_tag, Attr, AttrData, CodePosAt, Doc, Node, NodeType, RefNode, RootNode,
	},
};
use std::error::Error;
use std::rc::Rc;
use std::{any::Any, cell::RefCell};
/// type implement INodeTrait with Node
struct Dom {
	node: Rc<RefCell<Node>>,
}

impl Dom {
	fn halt(&self, method: &str, message: &str) {
		if let Some(doc) = &self.owner_document() {
			doc.trigger_error(Box::new(IError::InvalidTraitMethodCall {
				method: String::from(method),
				message: String::from(message),
			}));
		}
	}

	fn validate_dom_change(&self, node: &BoxDynElement, method: &str) -> bool {
		// test if current node is element node
		let my_node_type = self.node.borrow().node_type;
		if my_node_type != NodeType::Tag {
			self.halt(
				method,
				&format!("Can't {} for a {:?} type", method, my_node_type),
			);
			return false;
		}
		// document
		if let INodeType::Document = node.node_type() {
			self.halt(method, &format!("Can't {} of a document type", method));
			return false;
		}
		// test if same node
		if self.is(&node) {
			self.halt(method, &format!("Can't {} of self.", method));
			return false;
		}
		// test if the node is self's parent node
		let mut cur = self.cloned();
		while let Some(parent) = &cur.parent() {
			if parent.is(&node) {
				self.halt(method, &format!("Can't {} of self's parent", method));
				return false;
			}
			cur = parent.cloned();
		}
		true
	}
}

fn to_static_str(orig: String) -> &'static str {
	Box::leak(orig.into_boxed_str())
}

fn reset_next_siblings_index(start_index: usize, childs: &[RefNode]) {
	for (step, node) in childs.iter().enumerate() {
		node.borrow_mut().index = start_index + step;
	}
}

fn remove_not_allowed_nodes(tag_name: &str, nodes: &mut Vec<RefNode>) -> Vec<usize> {
	let mut not_allowed_indexs: Vec<usize> = Vec::with_capacity(nodes.len());
	for (index, node) in nodes.iter().enumerate() {
		if !allow_insert(tag_name, node.borrow().node_type) {
			not_allowed_indexs.push(index);
		}
	}
	if !not_allowed_indexs.is_empty() {
		retain_by_index(nodes, &not_allowed_indexs);
	}
	not_allowed_indexs
}

impl INodeTrait for Dom {
	/// impl `to_node`
	fn to_node(self: Box<Self>) -> Box<dyn Any> {
		self
	}

	/// impl `index`
	fn index(&self) -> usize {
		self.node.borrow().index
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
		match node_type {
			INodeType::Element => {
				let tag_name = self.tag_name();
				let no_content_tag = !is_content_tag(tag_name);
				let mut node = self.node.borrow_mut();
				if !content.is_empty() {
					let content = encode(content, SpecialChars, NamedOrDecimal);
					if no_content_tag {
						let mut text_node = Node::create_text_node(&content, None);
						// set text node parent
						text_node.parent = Some(Rc::downgrade(&self.node));
						// set childs
						node.childs = Some(vec![Rc::new(RefCell::new(text_node))]);
					} else {
						node.content = Some(content.chars().collect::<Vec<char>>());
					}
				} else {
					// empty content
					if no_content_tag {
						node.childs = None;
					} else {
						node.content = None;
					}
				}
			}
			INodeType::Text => {
				if content.is_empty() {
					self.halt("set_text",
            "the text parameter can't be empty, if you want to remove a text node, you can use 'remove' method instead."
          );
				} else {
					// replace the text content
					self.node.borrow_mut().content = Some(content.chars().collect::<Vec<char>>());
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
		if let Some(nodes) = &mut doc.get_root_node().borrow_mut().childs {
			// set childs with new childs
			match self.node_type() {
				INodeType::Element => {
					let nodes = nodes.split_off(0);
					for node in &nodes {
						node.borrow_mut().parent = Some(Rc::downgrade(&self.node));
					}
					self.node.borrow_mut().childs = Some(nodes);
				}
				INodeType::Text => {
					let index = self.index();
					if let Some(parent) = &self.node.borrow_mut().parent {
						if let Some(parent) = &parent.upgrade() {
							let tag_name = parent
								.borrow()
								.meta
								.as_ref()
								.map(|meta| meta.borrow().get_name(true))
								.expect("`set_html` a tag name must have");
							if let Some(childs) = &mut parent.borrow_mut().childs {
								let mut nodes = nodes.split_off(0);
								let not_allowed_indexs = remove_not_allowed_nodes(&tag_name, &mut nodes);
								if !not_allowed_indexs.is_empty() {
									let start_index = not_allowed_indexs[0];
									reset_next_siblings_index(start_index, &nodes[start_index..]);
								} else {
									// change append nodes index begin with index
									reset_next_siblings_index(index, &nodes[..]);
								}
								// not last node
								if index < childs.len() - 1 {
									// change next siblings index begin with index+nodes.len
									reset_next_siblings_index(index + nodes.len(), &childs[index + 1..]);
								}
								// delete the node and append childs
								if !nodes.is_empty() {
									// set nodes parent
									for node in &nodes {
										node.borrow_mut().parent = Some(Rc::downgrade(parent));
									}
									childs.splice(index..index + 1, nodes);
								} else {
									// just remove self
									childs.remove(index);
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

impl ITextTrait for Dom {
	// delete the node
	fn remove(self: Box<Self>) {
		let index = self.index();
		if let Some(parent) = &self.node.borrow_mut().parent {
			if let Some(parent) = parent.upgrade() {
				if let Some(childs) = &mut parent.borrow_mut().childs {
					// remove the text node
					childs.remove(index);
					// change next siblings index
					reset_next_siblings_index(index, &childs[index..]);
				}
			}
		}
	}

	// append text
	fn append_text(&mut self, content: &str) {
		let chars = content.chars().collect::<Vec<char>>();
		if let Some(content) = &mut self.node.borrow_mut().content {
			content.extend(chars);
		} else {
			self.node.borrow_mut().content = Some(chars);
		}
	}

	// prepend text
	fn prepend_text(&mut self, content: &str) {
		let chars = content.chars().collect::<Vec<char>>();
		if let Some(content) = &mut self.node.borrow_mut().content {
			content.splice(0..0, chars);
		} else {
			self.node.borrow_mut().content = Some(chars);
		}
	}
}

impl IUncareNodeTrait for Dom {}

impl IElementTrait for Dom {
	/// impl `tag_name`
	fn tag_name(&self) -> &str {
		match self.node_type() {
			INodeType::Element => {
				if let Some(meta) = &self.node.borrow().meta {
					let name = meta.borrow().get_name(true);
					return to_static_str(name);
				}
				self.halt("tag_name", "Html syntax error: not found a tag name.");
			}
			INodeType::Document | INodeType::DocumentFragement => {}
			cur_type => self.halt(
				"tag_name",
				&format!("The node type of '{:?}' doesn't have a tag name.", cur_type),
			),
		};
		""
	}

	/// impl `children`
	fn child_nodes_length(&self) -> usize {
		self
			.node
			.borrow()
			.childs
			.as_ref()
			.map_or(0, |childs| childs.len())
	}
	fn child_nodes_item<'b>(&self, index: usize) -> Option<BoxDynNode<'b>> {
		if let Some(childs) = &self.node.borrow().childs {
			return childs.get(index).map(|node| {
				Box::new(Dom {
					node: Rc::clone(node),
				}) as BoxDynNode
			});
		}
		None
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
			let find_index = meta.borrow().attrs.iter().position(|attr| {
				if let Some(key) = &attr.key {
					return key.content == name;
				}
				false
			});
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
	fn remove_child(&mut self, ele: BoxDynElement) {
		if let Some(parent) = &ele.parent() {
			if self.is(parent) {
				// is a child
				if let Some(childs) = self.node.borrow_mut().childs.as_mut() {
					let index = ele.index();
					// if not the last child
					if index != childs.len() - 1 {
						reset_next_siblings_index(index, &childs[index + 1..]);
					}
					// remove child
					childs.remove(index);
				}
			}
		}
	}
	// append child
	fn insert_adjacent(&mut self, position: &InsertPosition, node: &BoxDynElement) {
		// base validate
		let action = position.action();
		if !self.validate_dom_change(&node, action) {
			return;
		}
		let node_type = node.node_type();
		let specified: Box<dyn Any> = node.cloned().to_node();
		if let Ok(dom) = specified.downcast::<Dom>() {
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
					// remove current node from parent's childs
					if let Some(parent) = &mut node.parent() {
						parent.remove_child(node.cloned());
					}
					vec![dom.node]
				}
			};
			// filter the node allowed
			let tag_name = self.tag_name();
			let not_allowed_indexs = remove_not_allowed_nodes(tag_name, &mut nodes);
			let has_not_allowed = !not_allowed_indexs.is_empty();
			if nodes.is_empty() {
				return;
			}
			// insert
			use InsertPosition::*;
			match position {
				BeforeBegin | AfterEnd => {
					// get index first, for borrow check
					let mut index = self.index();
					let mut nexts: Vec<RefNode> = vec![];
					let insert_len = nodes.len();
					// it's insertAfter, increase the insertion index
					if *position == AfterEnd {
						index += 1;
					}
					if has_not_allowed {
						let start_index = not_allowed_indexs[0];
						reset_next_siblings_index(start_index, &nodes[start_index..]);
					} else {
						// always reset nodes
						reset_next_siblings_index(index, &nodes[..]);
					}
					if let Some(parent) = &self.node.borrow_mut().parent {
						if let Some(parent) = &parent.upgrade() {
							if let Some(childs) = &mut parent.borrow_mut().childs {
								// split the nexts for reset index.
								if index < childs.len() {
									nexts = childs.split_off(index);
								}
								// set node parent
								for node in &nodes {
									node.borrow_mut().parent = Some(Rc::downgrade(parent));
								}
								// insert nodes at the end
								childs.extend(nodes);
							}
						}
					}
					if !nexts.is_empty() {
						// reset nexts index
						reset_next_siblings_index(index + insert_len, &nexts[..]);
						// for borrrow check
						if let Some(parent) = &self.node.borrow_mut().parent {
							if let Some(parent) = parent.upgrade() {
								if let Some(childs) = &mut parent.borrow_mut().childs {
									//  insert nodes
									childs.extend(nexts);
								}
							}
						}
					}
				}
				AfterBegin | BeforeEnd => {
					// set nodes parent
					for node in &nodes {
						node.borrow_mut().parent = Some(Rc::downgrade(&self.node));
					}
					// prepend, append
					if let Some(childs) = &mut self.node.borrow_mut().childs {
						if *position == BeforeEnd {
							// reset nodes index
							reset_next_siblings_index(childs.len(), &nodes[..]);
							// append nodes
							childs.extend(nodes);
						} else {
							// reset if needed
							if has_not_allowed {
								let start_index = not_allowed_indexs[0];
								reset_next_siblings_index(start_index, &nodes[start_index..]);
							}
							// reset childs index
							reset_next_siblings_index(nodes.len(), &childs[..]);
							// append childs to nodes
							nodes.append(childs);
							// set childs to nodes
							*childs = nodes;
						}
					} else {
						if has_not_allowed {
							// reset nodes index
							let start_index = not_allowed_indexs[0];
							reset_next_siblings_index(start_index, &nodes[start_index..]);
						}
						self.node.borrow_mut().childs = Some(nodes);
					}
				}
			}
		} else {
			// not the Dom
			self.halt(
				action,
				&format!("Can't {} that not implemented 'Dom'", action),
			);
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
								let ele = node.typed().into_element().expect("ElementNode must true");
								loop_handle(ele, result, next_depth, limit_depth);
							}
						}
						_ => {}
					}
				}
			} else if is_content_tag(node.tag_name()) {
				// content tag, change the element into text type
				result.get_mut_ref().push(
					node
						.into_text()
						.expect("Content tag must be able to translate into text node"),
				);
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

	/// impl `into_text`
	fn into_text<'b>(self: Box<Self>) -> Result<BoxDynText<'b>, Box<dyn Error>> {
		if is_content_tag(self.tag_name()) {
			Ok(self as BoxDynText)
		} else {
			Err(Box::new(IError::InvalidTraitMethodCall {
				method: "into_text".into(),
				message: "Can't call 'into_text' with tags those are not content tags.".into(),
			}))
		}
	}

	/// impl `is`, is much faster than compare the `uuid`
	fn is(&self, ele: &BoxDynElement) -> bool {
		let specified: Box<dyn Any> = ele.cloned().to_node();
		if let Ok(dom) = specified.downcast::<Dom>() {
			return Node::is_same(&self.node, &dom.node);
		}
		false
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
/// Vis: Entry struct of the `mesdoc`'s api.
///
/// # Examples
///
/// ```
/// use visdom::Vis;
/// use std::error::Error;
/// fn main()-> Result<(), Box<dyn Error>>{
///   let html = r##"
///      <!doctype html>
///      <html>
///        <head>
///          <meta charset="utf-8" />
///        </head>
///        <body>
///           <nav id="header">
///            <ul>
///              <li>Hello,</li>
///              <li>Vis</li>
///              <li>Dom</li>
///            </ul>
///          </nav>
///        </body>
///     </html>
///   "##;
///   let doc = Vis::load(html)?;
///   // All useful css selectors are well supported.
///   let header = doc.find("#header");
///   let list_items = header.children("ul > li");
///   assert_eq!(list_items.length(), 3);
///   assert_eq!(list_items.text(), "Hello,VisDom");
///   let second_child = list_items.filter(":nth-child(2)");
///   assert_eq!(second_child.text(), "Vis");
///   // Easy to operate the nodes.
///   let mut fourth_child = Vis::load("<li>!</li>")?;
///   let mut parent = list_items.parent("");
///   assert_eq!(parent.length(), 1);
///   fourth_child.append_to(&mut parent);
///   let mut cur_list_items = header.children("ul > li");
///   assert_eq!(cur_list_items.length(), 4);
///   assert_eq!(cur_list_items.text(), "Hello,VisDom!");
///   // Powerful api for operate text nodes
///   // use append_text and prepend_text to change text
///   let mut texts = cur_list_items.texts(0);
///   texts.for_each(|_index, text_node|{
///      text_node.prepend_text("[");
///      text_node.append_text("]");
///      true
///   });
///   assert_eq!(cur_list_items.text(), "[Hello,][Vis][Dom][!]");
///   // use set_text to change text
///   texts.for_each(|_index, text_node|{
///       text_node.set_text("@");
///       true
///   });
///   assert_eq!(cur_list_items.text(), "@@@@");
///   // use set_html to mix content
///   texts.for_each(|_index, text_node|{
///       let orig_text = text_node.text();
///       let replace_html = format!("<span>{}</span><b>!</b>", orig_text);
///       // Be careful that now the text_node is destoryed by `set_html`
///       text_node.set_html(&replace_html);
///       true
///   });
///   assert_eq!(cur_list_items.children("b").length(), 4);
///   assert_eq!(cur_list_items.text(), "@!@!@!@!");
///   Ok(())
/// }
/// ```
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
