//! Visdom
//!
//! ### Description
//! A fast and easy to use library for html parsing & node selecting & node mutation, suitable for web scraping and html confusion.
//!
//! ### Features
//! - Easy to use: APIs are very closed to jQuery, most APIs you just need change the name from camelCase to snake_case.
//! - Standard css selectors: e.g. `#id`, `.class`, `p`, `[attr~=value]`, `:nth-child(odd)`, `:nth-of-type(n)`, `:not`, other jQuery-only selectors are supported too, such as `:contains`, `:header` and so on.
//! - Useful API methods: e.g. `find`, `filter`, `has`, `is`, `not`, `add`, `closest`, `map` and so on.
//! - Content mutations: `set_html`, `set_text` can also called by text nodes, external methods such as `append_text`、`prepend_text` are supported.
//! - Well tested: the unit tests have covered most cases, but if you meet any bugs or questions, welcome to submit issues or PR to us.
#[macro_use]
mod macros;
mod mesdoc;
// feature="text"
cfg_feat_text! {
	use mesdoc::interface::Texts;
}
// feature="mutation"
cfg_feat_insertion! {
	use mesdoc::interface::InsertPosition;
}
use mesdoc::interface::{
	BoxDynElement, BoxDynNode, BoxDynText, BoxDynUncareNode, Elements, IDocumentTrait, IElementTrait,
	IErrorHandle, IFormValue, INodeTrait, ITextTrait, IUncareNodeTrait, MaybeDoc, MaybeElement,
};

use mesdoc::utils::is_equal_chars;
use mesdoc::{error::Error as IError, utils::retain_by_index};
use rphtml::{
	config::RenderOptions,
	entity::{encode, encode_char, CharacterSet, EncodeType, ICodedDataTrait},
	parser::{
		allow_insert, is_content_tag, Attr, AttrData, Doc, DocHolder, NameCase, Node, NodeType, RefNode,
	},
};
use std::borrow::Cow;
use std::rc::Rc;
use std::{any::Any, cell::RefCell};
// re export `IAttrValue` `IEnumTyped` `INodeType`
pub mod types {
	// text
	cfg_feat_text! {
		pub use crate::mesdoc::interface::Texts;
	}
	pub use crate::mesdoc::error::BoxDynError;
	pub use crate::mesdoc::interface::{
		BoxDynElement, BoxDynNode, BoxDynText, Elements, IAttrValue, IDocumentTrait, IEnumTyped,
		IFormValue, INodeType,
	};
	pub use crate::mesdoc::selector::Combinator;
}

// re export `ParseOptions`
pub mod html {
	pub use rphtml::config::ParseOptions;
}

use crate::html::ParseOptions;
use crate::types::{BoxDynError, IAttrValue, IEnumTyped, INodeType};
/// type implement INodeTrait with Node
struct Dom;

impl Dom {
	fn halt(dom: &Rc<RefCell<Node>>, method: &str, message: &str) {
		if let Some(doc) = &dom.owner_document() {
			doc.trigger_error(Box::new(IError::InvalidTraitMethodCall {
				method: String::from(method),
				message: String::from(message),
			}));
		}
	}
	// when mutation feature is open
	cfg_feat_insertion! {
		fn validate_dom_change(dom: &Rc<RefCell<Node>>, node: &BoxDynElement, method: &str) -> bool {
			// test if current node is element node
			let my_node_type = dom.borrow().node_type;
			if my_node_type != NodeType::Tag {
				Dom::halt(
					dom,
					method,
					&format!("Can't {} for a {:?} type", method, my_node_type),
				);
				return false;
			}
			// document
			if let INodeType::Document = node.node_type() {
				Dom::halt(dom, method, &format!("Can't {} a document type", method));
				return false;
			}
			// test if same node
			if dom.is(node) {
				Dom::halt(
					dom,
					method,
					&format!("Can't {} a dom that contains itself.", method),
				);
				return false;
			}
			// test if the node is dom's parent node
			let mut cur = dom.cloned();
			while let Some(parent) = cur.parent() {
				if parent.is(node) {
					Dom::halt(
						dom,
						method,
						&format!("Can't {} a dom that contains it's parent", method),
					);
					return false;
				}
				cur = parent;
			}
			true
		}
	}
}

fn reset_next_siblings_index(start_index: usize, childs: &[RefNode]) {
	for (step, node) in childs.iter().enumerate() {
		node.borrow_mut().index = start_index + step;
	}
}

fn remove_not_allowed_nodes(tag_name: &[char], nodes: &mut Vec<RefNode>) -> bool {
	let mut not_allowed_indexs: Vec<usize> = Vec::with_capacity(nodes.len());
	let orig_len = nodes.len();
	for (index, node) in nodes.iter().enumerate() {
		if !allow_insert(tag_name, node.borrow().node_type) {
			not_allowed_indexs.push(index);
		}
	}
	if !not_allowed_indexs.is_empty() {
		retain_by_index(nodes, &not_allowed_indexs);
	}
	let now_allowed_len = not_allowed_indexs.len();
	now_allowed_len > 0 && now_allowed_len != orig_len
}

fn check_if_content_tag(name: &[char]) -> bool {
	is_content_tag(name, &Some(NameCase::Lower))
}

impl INodeTrait for Rc<RefCell<Node>> {
	/// impl `to_node`
	fn to_node(self: Box<Self>) -> Box<dyn Any> {
		self
	}

	/// impl `index`
	fn index(&self) -> usize {
		self.borrow().index
	}

	/// The current version of this method only implements the clone of `Rc` pointers.
	/// This is different from the standard `clone_node` method.
	/// If you want to use `clone_node` method with the standard semantics, now you can use `copy_node` instead.
	/// This method may be changed in future versions to be consist with the standard `clone_node` semantics.
	fn clone_node<'b>(&self) -> BoxDynNode<'b> {
		Box::new(self.clone())
	}

	/// impl standard semantics `clone_node` method
	fn copy_node<'b>(&self) -> BoxDynNode<'b> {
		Box::new(self.borrow().clone_node())
	}

	/// impl `typed`
	fn typed<'b>(self: Box<Self>) -> IEnumTyped<'b> {
		match self.node_type() {
			INodeType::Element | INodeType::DocumentFragement | INodeType::Document => {
				IEnumTyped::Element(self as BoxDynElement)
			}
			INodeType::Text => IEnumTyped::Text(self as BoxDynText),
			_ => IEnumTyped::UncareNode(self as BoxDynUncareNode),
		}
	}

	/// impl `node_type`
	fn node_type(&self) -> INodeType {
		let node_type = self.borrow().node_type;
		match node_type {
			NodeType::AbstractRoot => {
				let (is_document, _) = self.borrow().is_document();
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
		if let Some(parent) = &self.borrow().parent {
			if let Some(node) = parent.upgrade() {
				return Some(Box::new(node));
			}
		}
		None
	}

	/// impl `owner_document`
	fn owner_document(&self) -> MaybeDoc {
		if let Some(root) = &self.borrow().root {
			if let Some(root) = &root.upgrade() {
				if let Some(doc) = &root.borrow().document {
					if let Some(ref_doc) = &doc.upgrade() {
						return Some(Box::new(Document {
							doc: Rc::clone(ref_doc).into(),
						}));
					}
				}
			}
		}
		None
	}

	/// impl `text_contents`
	fn text_contents(&self) -> Vec<char> {
		self.borrow().build(
			&RenderOptions {
				decode_entity: true,
				..Default::default()
			},
			matches!(self.node_type(), INodeType::Element | INodeType::Comment),
		)
	}

	/// impl `text_chars`
	/// `text_chars` keep the original characters in html
	fn text_chars(&self) -> Vec<char> {
		self.borrow().build(
			&Default::default(),
			matches!(self.node_type(), INodeType::Element),
		)
	}

	/// `set_text` for an element node or a text node
	/// # Notice
	/// For an element node, if the tag is a `script` or `style` or `textarea` or `title`, the content will not be encoded.
	/// Otherwise, the content will be encoded at first.
	fn set_text(&mut self, content: &str) {
		let node_type = self.node_type();
		match node_type {
			INodeType::Element => {
				let no_content_tag = !check_if_content_tag(&self.tag_names());
				let mut node = self.borrow_mut();
				if !content.is_empty() {
					if no_content_tag {
						// encode content
						let content = encode(
							content.as_bytes(),
							&EncodeType::NamedOrDecimal,
							&CharacterSet::Html,
						);
						let mut text_node = Node::create_text_node(content.to_chars().unwrap(), None);
						// set text node parent
						text_node.parent = Some(Rc::downgrade(self));
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
			INodeType::Text | INodeType::Comment => {
				if content.is_empty() {
					Dom::halt(self,"set_text",
            "the text parameter can't be empty, if you want to remove a text node, you can use 'remove' method instead."
          );
				} else {
					// replace the text content
					self.borrow_mut().content = Some(content.chars().collect::<Vec<char>>());
				}
			}
			_ => {
				// nothing to do with other node types
			}
		}
	}

	/// impl `set_html`
	fn set_html(&mut self, content: &str) {
		let mut is_element = true;
		let target = match self.node_type() {
			INodeType::Element => Some(Rc::clone(self)),
			INodeType::Text => {
				if let Some(parent) = &self.borrow_mut().parent {
					if let Some(parent) = &parent.upgrade() {
						is_element = false;
						Some(Rc::clone(parent))
					} else {
						None
					}
				} else {
					None
				}
			}
			_ => None,
		};
		if let Some(target) = &target {
			if check_if_content_tag(
				&target
					.borrow()
					.meta
					.as_ref()
					.expect("A tag use `set_html` must have a tag name.")
					.borrow()
					.name,
			) {
				// content tag, just set html as content, no need encode
				target.borrow_mut().content = Some(content.chars().collect::<Vec<char>>());
			} else {
				let doc_holder = Doc::parse(
					content,
					ParseOptions {
						auto_fix_unexpected_endtag: true,
						auto_fix_unescaped_lt: true,
						..Default::default()
					},
				)
				.unwrap();
				if let Some(nodes) = &mut doc_holder.get_root_node().borrow_mut().childs {
					let mut nodes = nodes.split_off(0);
					let has_not_allowed = remove_not_allowed_nodes(
						&target
							.borrow()
							.meta
							.as_ref()
							.expect("A tag use `set_html` must have a tag name.")
							.borrow()
							.name,
						&mut nodes,
					);
					let has_nodes = !nodes.is_empty();
					if has_nodes {
						// set nodes parent as target
						for node in &nodes {
							node.borrow_mut().parent = Some(Rc::downgrade(target));
						}
					}
					if is_element {
						// reset node indexs
						if has_not_allowed {
							reset_next_siblings_index(0, &nodes);
						}
						// set childs as new nodes
						(*target.borrow_mut()).childs = if has_nodes { Some(nodes) } else { None };
					} else if let Some(childs) = &mut target.borrow_mut().childs {
						let index = self.index();
						// not last node, whenever nodes is empty or not, reset next childs indexs
						if index < childs.len() - 1 {
							// change next siblings index begin with index+nodes.len
							reset_next_siblings_index(index + nodes.len(), &childs[index + 1..]);
						}
						// delete the node and append childs
						if has_nodes {
							// change append nodes index begin with index
							reset_next_siblings_index(index, &nodes);
							// remove self and insert nodes
							childs.splice(index..index + 1, nodes);
						} else {
							// then just remove self
							childs.remove(index);
						}
					} else {
						// text node's parent can't be empty
					}
				} else {
					// empty html, just set childs to none
					target.borrow_mut().childs = None;
				}
			}
		}
	}
}

impl ITextTrait for Rc<RefCell<Node>> {
	cfg_feat_text! {
		/// Remove a text node.
		fn remove(self: Box<Self>) {
			let index = self.index();
			if let Some(parent) = &self.borrow_mut().parent {
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
			if let Some(content) = &mut self.borrow_mut().content {
				content.extend(chars);
			} else {
				self.borrow_mut().content = Some(chars);
			}
		}

		// prepend text
		fn prepend_text(&mut self, content: &str) {
			let chars = content.chars().collect::<Vec<char>>();
			if let Some(content) = &mut self.borrow_mut().content {
				content.splice(0..0, chars);
			} else {
				self.borrow_mut().content = Some(chars);
			}
		}
	}
}

impl IUncareNodeTrait for Rc<RefCell<Node>> {}

impl IElementTrait for Rc<RefCell<Node>> {
	/// impl `names`
	fn tag_names(&self) -> Vec<char> {
		match self.node_type() {
			INodeType::Element => {
				if let Some(meta) = &self.borrow().meta {
					return meta
						.borrow()
						.name
						.iter()
						.map(|ch| ch.to_ascii_lowercase())
						.collect();
				}
				Dom::halt(self, "tag_name", "Html syntax error: not found a tag name.");
			}
			INodeType::Document | INodeType::DocumentFragement => {}
			cur_type => Dom::halt(
				self,
				"tag_name",
				&format!("The node type of '{:?}' doesn't have a tag name.", cur_type),
			),
		};
		vec![]
	}
	/// impl `value`
	fn value(&self) -> IFormValue {
		let tag_name = self.tag_names();
		let input_tag = ['i', 'n', 'p', 'u', 't'];
		let select_tag = ['s', 'e', 'l', 'e', 'c', 't'];
		let option_tag = ['o', 'p', 't', 'i', 'o', 'n'];
		let textarea_tag = ['t', 'e', 'x', 't', 'a', 'r', 'e', 'a'];
		// if it's an (input)
		if is_equal_chars(&tag_name, &input_tag) || is_equal_chars(&tag_name, &option_tag) {
			// input
			if let Some(IAttrValue::Value(v, _)) = self.get_attribute("value") {
				return IFormValue::Single(v);
			}
			return IFormValue::Single(String::from(""));
		}
		// if it's a (select)
		if is_equal_chars(&tag_name, &select_tag) {
			let is_multiple = self.has_attribute("multiple");
			// default value, if no option has "selected"
			let mut default_value: Option<String> = None;
			// if select's 'multiple' attribute is setted
			let mut values: Vec<String> = vec![];
			// collect option values
			fn collect_option_values(
				parent: &RefNode,
				default_value: &mut Option<String>,
				values: &mut Vec<String>,
				option_tag: &[char],
				is_multiple: bool,
				level: u8,
			) {
				if let Some(childs) = &parent.borrow().childs {
					for child in childs {
						if matches!(child.node_type(), INodeType::Element) {
							let child_tag = child.tag_names();
							if is_equal_chars(&child_tag, option_tag) {
								let is_selected = child.has_attribute("selected");
								if is_selected || (default_value.is_none() && level == 0) {
									let value = if let Some(IAttrValue::Value(v, _)) = child.get_attribute("value") {
										v
									} else {
										String::from("")
									};
									if is_selected {
										values.push(value);
										// break if is single select
										if !is_multiple {
											break;
										}
									} else {
										*default_value = Some(value);
									}
								}
							} else {
								collect_option_values(
									child,
									default_value,
									values,
									option_tag,
									is_multiple,
									level + 1,
								);
							}
						}
					}
				}
			}
			// get the values of option
			collect_option_values(
				self,
				&mut default_value,
				&mut values,
				&option_tag,
				is_multiple,
				0,
			);
			// if is multiple select
			if is_multiple {
				return IFormValue::Multiple(values);
			}
			// single select, check if has selected option
			if !values.is_empty() {
				return IFormValue::Single(values.remove(0));
			}
			// no selected option or no option tag
			return IFormValue::Single(default_value.unwrap_or_else(|| String::from("")));
		}
		// if it's an (textarea)
		if is_equal_chars(&tag_name, &textarea_tag) {
			// textarea
			return IFormValue::Single(self.text());
		}
		// other element
		IFormValue::Single(String::from(""))
	}
	/// impl `children`
	fn child_nodes_length(&self) -> usize {
		self
			.borrow()
			.childs
			.as_ref()
			.map_or(0, |childs| childs.len())
	}
	fn child_nodes_item<'b>(&self, index: usize) -> Option<BoxDynNode<'b>> {
		if let Some(childs) = &self.borrow().childs {
			return childs
				.get(index)
				.map(|node| Box::new(Rc::clone(node)) as BoxDynNode);
		}
		None
	}
	fn child_nodes_item_since_by<'a>(
		&'a self,
		node_index: usize,
		reverse: bool,
		mut handle: Box<dyn FnMut(&dyn IElementTrait) -> bool + 'a>,
	) {
		if let Some(childs) = &self.borrow().childs {
			if reverse {
				for child in childs[0..=node_index].iter().rev() {
					if matches!(child.node_type(), INodeType::Element) {
						// match handle
						if !handle(child) {
							break;
						}
					}
				}
			} else {
				for child in childs[node_index..].iter() {
					if matches!(child.node_type(), INodeType::Element) {
						// match handle
						if !handle(child) {
							break;
						}
					}
				}
			}
		}
	}

	fn children_by<'a>(&'a self, mut matcher: Box<dyn FnMut(&dyn IElementTrait) + 'a>) {
		if let Some(childs) = &self.borrow().childs {
			for child in childs {
				if matches!(child.node_type(), INodeType::Element) {
					matcher(child);
				}
			}
		}
	}
	/// impl `get_attribute`
	fn get_attribute(&self, name: &str) -> Option<IAttrValue> {
		// use lowercase to get attribute: issue: #2
		let node = &self.borrow();
		let meta = node
			.meta
			.as_ref()
			.expect("Element node must have a meta field.");
		// if has meta, then compare with lowercase
		let lc_name_map = &meta.borrow().lc_name_map;
		if !lc_name_map.is_empty() {
			if let Some(&index) = lc_name_map.get(&name.to_ascii_lowercase()) {
				let attrs = &meta.borrow().attrs;
				let attr = &attrs[index];
				if let Some(value) = &attr.value {
					let attr_value = value.content.clone();
					return Some(IAttrValue::Value(attr_value.iter().collect(), attr.quote));
				} else {
					return Some(IAttrValue::True);
				}
			}
		}
		None
	}

	/// impl `set_attribute`
	fn set_attribute(&mut self, name: &str, value: Option<&str>) {
		let mut need_quote = false;
		let mut quote: char = '"';
		if let Some(meta) = &self.borrow().meta {
			let value = value.map(|v| {
				let mut find_quote: bool = false;
				let mut content = Vec::with_capacity(v.len());
				// loop the chars
				for ch in v.chars() {
					if !need_quote {
						need_quote = Attr::need_quoted_char(&ch);
					}
					if ch == '"' || ch == '\'' {
						if find_quote {
							if quote == ch {
								// find more quotes
								if let Some(entity) = encode_char(&ch, &EncodeType::Named) {
									entity.write_chars(&mut content);
								} else {
									content.push(ch);
								}
							} else {
								content.push(ch);
							}
						} else {
							// if first is double quote, change the variable `quote` to single quote
							find_quote = true;
							if ch == '"' {
								quote = '\'';
							}
							content.push(ch);
						}
					} else {
						content.push(ch);
					}
				}
				AttrData { content }
			});
			// first, check if the attribute has exist.
			let lc_name = name.to_ascii_lowercase();
			let find_index = if let Some(&index) = meta.borrow().lc_name_map.get(&lc_name) {
				Some(index)
			} else {
				None
			};
			// find the attribute, just set the value
			if let Some(index) = find_index {
				meta.borrow_mut().attrs[index].value = value;
				return;
			}
			// new attribute, add it to the attrs and add a lowercase name to lc_name_map
			let index = meta.borrow().attrs.len();
			// insert name and index into name map
			meta.borrow_mut().lc_name_map.insert(lc_name, index);
			// add to attrs
			let quote = if value.is_some() { Some(quote) } else { None };
			meta.borrow_mut().attrs.push(Attr {
				key: Some(AttrData {
					content: name.chars().collect(),
				}),
				value,
				quote,
				need_quote,
			});
		}
	}

	/// impl `remove_attribute`
	fn remove_attribute(&mut self, name: &str) {
		if let Some(meta) = &self.borrow().meta {
			let mut find_index: Option<usize> = None;
			if !meta.borrow().lc_name_map.is_empty() {
				let lc_name = name.to_ascii_lowercase();
				if let Some(&index) = meta.borrow().lc_name_map.get(&lc_name) {
					find_index = Some(index);
				}
			}
			if let Some(index) = find_index {
				// set attr data as null data
				meta.borrow_mut().attrs[index] = Attr::default();
				// remove name from names map
				meta.borrow_mut().lc_name_map.remove(name);
			}
		}
	}

	/// impl `inner_html`
	fn inner_html(&self) -> String {
		self
			.borrow()
			.build(
				&RenderOptions {
					inner_html: true,
					encode_content: true,
					..Default::default()
				},
				false,
			)
			.iter()
			.collect::<String>()
	}

	/// impl `outer_html`
	fn outer_html(&self) -> String {
		self
			.borrow()
			.build(
				&RenderOptions {
					encode_content: true,
					..Default::default()
				},
				false,
			)
			.iter()
			.collect::<String>()
	}

	// when the feature `destroy` or `insertion` is open
	cfg_feat_mutation! {
		/// impl `remov_child`
		fn remove_child(&mut self, ele: BoxDynElement) {
			if let Some(parent) = &ele.parent() {
				if self.is(parent) {
					// is a child
					if let Some(childs) = self.borrow_mut().childs.as_mut() {
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
	}
	// when the feature `insertion` is open
	cfg_feat_insertion! {
		// append child, insert siblings
		fn insert_adjacent(&mut self, position: &InsertPosition, node: &BoxDynElement) {
			// base validate
			let action = position.action();
			if !Dom::validate_dom_change(self, node, action) {
				return;
			}
			let node_type = node.node_type();
			let specified: Box<dyn Any> = node.cloned().to_node();
			if let Ok(dom) = specified.downcast::<RefNode>() {
				// get the nodes
				let mut nodes = match node_type {
					INodeType::DocumentFragement => {
						if let Some(childs) = &dom.borrow().childs {
							childs.iter().map(Rc::clone).collect::<Vec<RefNode>>()
						} else {
							vec![]
						}
					}
					_ => {
						// remove current node from parent's childs
						if let Some(parent) = &mut node.parent() {
							parent.remove_child(node.cloned());
						}
						vec![*dom]
					}
				};
				// filter the node allowed
				let tag_name = self.tag_names();
				// insert
				use InsertPosition::*;
				// remove not allowed nodes, only insert childs
				if matches!(position, AfterBegin | BeforeEnd){
					remove_not_allowed_nodes(&tag_name, &mut nodes);
				}
				// check if is empty
				if nodes.is_empty() {
					return;
				}
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
						// always reset node indexs
						reset_next_siblings_index(index, &nodes);
						// split to prev and next
						if let Some(parent) = &self.borrow_mut().parent {
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
							reset_next_siblings_index(index + insert_len, &nexts);
							// for borrrow check
							if let Some(parent) = &self.borrow_mut().parent {
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
							node.borrow_mut().parent = Some(Rc::downgrade(self));
						}
						// prepend, append
						if let Some(childs) = &mut self.borrow_mut().childs {
							if *position == BeforeEnd {
								// reset nodes index
								reset_next_siblings_index(childs.len(), &nodes);
								// append nodes
								childs.extend(nodes);
							} else {
								// always reset nodes index
								reset_next_siblings_index(0, &nodes);
								// reset childs index
								reset_next_siblings_index(nodes.len(), childs);
								// append childs to nodes
								nodes.append(childs);
								// set childs to nodes
								*childs = nodes;
							}
							return;
						}
						// reset nodes index
						reset_next_siblings_index(0, &nodes);
						// set nodes as childs
						self.borrow_mut().childs = Some(nodes);
					}
				}
			} else {
				// not the Dom
				Dom::halt(
					self,
					action,
					&format!("Can't {} that not implemented 'Dom'", action),
				);
			}
		}
		// replace with
		fn replace_with(&mut self, node: &BoxDynElement){
			let node_type = node.node_type();
			let specified: Box<dyn Any> = node.cloned().to_node();
			let mut replace_ele: Option<RefNode> = None;
			if let Ok(dom) = specified.downcast::<RefNode>() {
				// get the nodes
				let nodes = match node_type {
					INodeType::DocumentFragement => {
						if let Some(childs) = &dom.borrow().childs {
							if childs.len() == 1 {
								replace_ele = Some(Rc::clone(&childs[0]));
							}
							childs.iter().map(Rc::clone).collect::<Vec<RefNode>>()
						} else {
							vec![]
						}
					}
					_ => {
						// remove current node from parent's childs
						if let Some(parent) = &mut node.parent() {
							parent.remove_child(node.cloned());
						}
						replace_ele = Some(Rc::clone(&dom));
						vec![*dom]
					}
				};
				// check if has nodes
				if nodes.is_empty(){
					return;
				}
				// get index first, for borrow check
				let index = self.index();
				let insert_len = nodes.len();
				// replace with nodes
				if let Some(parent) = &self.borrow_mut().parent {
					if let Some(parent) = &parent.upgrade() {
						if let Some(childs) = &mut parent.borrow_mut().childs {
							let next_index = index + 1;
							// reset node indexs
							reset_next_siblings_index(index, &nodes);
							// reset next childs indexs
							if index < childs.len(){
								reset_next_siblings_index(index + insert_len, &childs[next_index..]);
							}
							// set node parent
							for node in &nodes {
								node.borrow_mut().parent = Some(Rc::downgrade(parent));
							}
							// delete current index node
							// insert replace nodes
							childs.splice(index..next_index, nodes);
						}
					}
				}
				if let Some(replace_ele) = replace_ele {
					*self = replace_ele;
				}
			} else {
				let action = "replace with";
				Dom::halt(
					self,
					action,
					&format!("Can't {} that not implemented 'Dom'", action),
				);
			}
		}
	}

	// when the feature `text` is open
	cfg_feat_text! {
		/// impl `texts`
		fn texts_by_rec<'b>(
			&self,
			limit_depth: usize,
			handle: &dyn Fn(usize, &BoxDynText) -> bool,
			rec_handle: &dyn Fn(&BoxDynElement) -> bool
		) -> Option<Texts<'b>> {
			let limit_depth = if limit_depth == 0 {
				usize::MAX
			} else {
				limit_depth
			};
			let mut result: Texts = Texts::with_capacity(5);
			fn loop_handle(
				ele: BoxDynElement,
				result: &mut Texts,
				cur_depth: usize,
				limit_depth: usize,
				handle: &dyn Fn(usize, &BoxDynText) -> bool,
				rec_handle: &dyn Fn(&BoxDynElement) -> bool
			) {
				let child_nodes = ele.child_nodes();
				if !child_nodes.is_empty() {
					let next_depth = cur_depth + 1;
					let recursive = next_depth < limit_depth;
					for node in &child_nodes {
						match node.node_type() {
							INodeType::Text => {
								// append text node to result
								let node = node.clone_node();
								let text = node.typed().into_text().expect("TextNode must true");
								if handle(cur_depth, &text) {
									result.get_mut_ref().push(text);
								}
							}
							INodeType::Element => {
								let node = node.clone_node();
								let cur_ele = node.typed().into_element().expect("ElementNode must true");
								if check_if_content_tag(&cur_ele.tag_names()) {
									// check if content tag
									// content tag, change the element into text type
									let text = cur_ele
										.into_text()
										.expect("Content tag must be able to translate into text node");
									if handle(cur_depth, &text) {
										result.get_mut_ref().push(text);
									}
								} else if recursive && rec_handle(&cur_ele){
									// not content tags need recursive
									// if need recursive find the text node
									loop_handle(cur_ele, result, next_depth, limit_depth, handle, rec_handle);
								}
							}
							_ => {}
						}
					}
				} else if check_if_content_tag(&ele.tag_names()) {
					// content tag, change the element into text type
					let text = ele
						.into_text()
						.expect("Content tag must be able to translate into text node");
					if handle(cur_depth, &text) {
						result.get_mut_ref().push(text);
					}
				}
			}
			let ele = Box::new(Rc::clone(self)) as BoxDynElement;
			loop_handle(ele, &mut result, 0, limit_depth, handle, rec_handle);
			if !result.is_empty() {
				return Some(result);
			}
			None
		}
	}
	/// impl `into_text`
	fn into_text<'b>(self: Box<Self>) -> Result<BoxDynText<'b>, BoxDynError> {
		if check_if_content_tag(&self.tag_names()) {
			Ok(self as BoxDynText)
		} else {
			Err(Box::new(IError::InvalidTraitMethodCall {
				method: "into_text".into(),
				message: "Can't call 'into_text' with tags those are not content tags.".into(),
			}))
		}
	}

	/// impl `is`
	fn is(&self, ele: &BoxDynElement) -> bool {
		let specified: Box<dyn Any> = ele.cloned().to_node();
		if let Ok(dom) = specified.downcast::<RefNode>() {
			return Node::is_same(self, &dom);
		}
		false
	}

	/// impl `is_root_element`
	fn is_root_element(&self) -> bool {
		matches!(self.borrow().node_type, NodeType::AbstractRoot)
	}
}

pub struct Document {
	doc: DocHolder,
}

impl Document {
	fn bind_error(&mut self, handle: IErrorHandle) {
		*self.doc.borrow().onerror.borrow_mut() = Some(Rc::new(handle));
	}
	/// Get an elements set.
	pub fn elements<'b>(self) -> Elements<'b> {
		let root = Rc::clone(&self.doc.borrow().root);
		Elements::with_all(vec![Box::new(root)], Some(Box::new(self)))
	}
	/// Destroy the document
	pub fn destroy(self) {}
}

impl IDocumentTrait for Document {
	// get element by id
	fn get_element_by_id<'b>(&self, id: &str) -> Option<BoxDynElement<'b>> {
		if let Some(node) = self.doc.get_element_by_id(id) {
			return Some(Box::new(Rc::clone(&node)));
		}
		None
	}
	// source code
	fn source_code(&self) -> String {
		self.doc.render(&Default::default())
	}
	// get root node, in rphtml is abstract root node
	fn get_root_node<'b>(&self) -> BoxDynNode<'b> {
		Box::new(Rc::clone(&self.doc.borrow().root))
	}
	// onerror
	fn onerror(&self) -> Option<Rc<IErrorHandle>> {
		(*self.doc.borrow().onerror.borrow())
			.as_ref()
			.map(Rc::clone)
	}
}

/// Vis: Entry struct of the `mesdoc`'s api.
///
/// # Examples
///
/// ```
/// use visdom::Vis;
/// use visdom::types::BoxDynError;
/// fn main()-> Result<(), BoxDynError>{
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
///       // Be careful that now the text_node is destroyed by `set_html`
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
	pub(crate) fn options() -> ParseOptions {
		// use the most compatible options
		ParseOptions {
			auto_fix_unclosed_tag: true,
			auto_fix_unexpected_endtag: true,
			auto_fix_unescaped_lt: true,
			allow_self_closing: true,
			..Default::default()
		}
	}
	// parse a document with options
	pub(crate) fn parse_doc_options<'html>(
		html: impl Into<Cow<'html, str>>,
		options: ParseOptions,
	) -> Result<Document, BoxDynError> {
		mesdoc::init();
		let doc = Doc::parse(&html.into(), options)?;
		Ok(Document { doc })
	}
	/// load the html with options, get an elements collection
	pub fn load_options<'html>(
		html: impl Into<Cow<'html, str>>,
		options: ParseOptions,
	) -> Result<Elements<'html>, BoxDynError> {
		let doc = Vis::parse_doc_options(html, options)?;
		Ok(doc.elements())
	}
	/// load the html with options, and catch the errors
	pub fn load_options_catch<'html>(
		html: impl Into<Cow<'html, str>>,
		options: ParseOptions,
		handle: IErrorHandle,
	) -> Elements<'html> {
		let doc = Vis::parse_doc_options(html, options);
		if let Ok(mut doc) = doc {
			doc.bind_error(handle);
			doc.elements()
		} else {
			handle(doc.err().unwrap());
			Elements::new()
		}
	}
	/// load the html into elements
	pub fn load<'html>(html: impl Into<Cow<'html, str>>) -> Result<Elements<'html>, BoxDynError> {
		Vis::load_options(html, Vis::options())
	}
	/// load the html, and catch the errors
	pub fn load_catch<'html>(
		html: impl Into<Cow<'html, str>>,
		handle: IErrorHandle,
	) -> Elements<'html> {
		Vis::load_options_catch(html, Vis::options(), handle)
	}
	/// return an elements collection from an BoxDynElement
	pub fn dom<'b>(ele: &BoxDynElement) -> Elements<'b> {
		Elements::with_nodes(vec![ele.cloned()])
	}
}
