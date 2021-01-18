use ntree::selector::interface::{
	BoxDynNode, IAttrValue, IDocumentTrait, INodeTrait, INodeType, MaybeDocResult, MaybeResult,
	NodeList, Result,
};
use ntree::{self};
use rphtml::{
	config::RenderOptions,
	parser::{Attr, AttrData, CodePosAt, Doc, Node, NodeType, RootNode},
};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::result::Result as StdResult;
/// type implement INodeTrait with Node
struct Dom {
	node: Rc<RefCell<Node>>,
}

fn to_static_str(orig: String) -> &'static str {
	Box::leak(orig.into_boxed_str())
}

impl INodeTrait for Dom {
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
			cur_type => panic!("The node type of '{:?}' doesn't have a tag name.", cur_type),
		}
	}
	/// impl `node_type`
	fn node_type(&self) -> INodeType {
		match self.node.borrow().node_type {
			NodeType::AbstractRoot => INodeType::Document,
			NodeType::Comment => INodeType::Comment,
			NodeType::Text => INodeType::Text,
			NodeType::SpacesBetweenTag => INodeType::Spaces,
			NodeType::Tag => INodeType::Element,
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
			return;
		}
		panic!("Can't apply 'set_attribute' to node");
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
		to_static_str(self.node.borrow().build(&Default::default(), true))
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

pub struct Vis {
	pub doc: Doc,
}

impl Vis {
	pub fn init() {
		ntree::init();
	}
	pub fn load(html: &str) -> StdResult<Vis, Box<dyn Error>> {
		let doc = Doc::parse(html, Default::default())?;
		Ok(Vis { doc })
	}
	// find
	pub fn find(&self, query: &str) -> Result {
		let root: Dom = Rc::clone(&self.doc.get_root_node()).into();
		let root_nodes = NodeList::with_nodes(vec![Box::new(root)]);
		root_nodes.find(query)
	}
}
