use super::{BoxDynElement, BoxDynText, BoxDynUncareNode, MaybeDoc, MaybeElement};
use std::any::Any;
#[derive(Debug)]
pub enum INodeType {
	Element = 1,
	Text = 3,
	XMLCDATA = 4,
	Comment = 8,
	Document = 9,
	HTMLDOCTYPE = 10,
	DocumentFragement = 11,
	Other = 14,
}

impl INodeType {
	pub fn is_element(&self) -> bool {
		matches!(self, INodeType::Element)
	}
}

pub type BoxDynNode<'a> = Box<dyn INodeTrait + 'a>;
pub enum IEnumTyped<'a> {
	Element(BoxDynElement<'a>),
	Text(BoxDynText<'a>),
	UncareNode(BoxDynUncareNode<'a>),
}

impl<'a> IEnumTyped<'a> {
	pub fn into_element(self) -> Option<BoxDynElement<'a>> {
		match self {
			IEnumTyped::Element(ele) => Some(ele),
			_ => None,
		}
	}
	pub fn into_text(self) -> Option<BoxDynText<'a>> {
		match self {
			IEnumTyped::Text(ele) => Some(ele),
			_ => None,
		}
	}
}

pub trait INodeTrait {
	fn to_node(self: Box<Self>) -> Box<dyn Any>;
	// clone a node
	fn clone_node<'b>(&self) -> BoxDynNode<'b>;
	// copy a node
	fn copy_node<'b>(&self) -> BoxDynNode<'b>;
	// typed, whether element or text
	fn typed<'b>(self: Box<Self>) -> IEnumTyped<'b>;
	// get ele type
	fn node_type(&self) -> INodeType;
	// find parents
	fn parent<'b>(&self) -> MaybeElement<'b>;

	// owner document
	fn owner_document(&self) -> MaybeDoc;
	// root element
	fn root_element<'b>(&self) -> Option<BoxDynElement<'b>> {
		if let Some(doc) = &self.owner_document() {
			let root_node = &doc.get_root_node();
			return Box::new(root_node.clone_node()).typed().into_element();
		}
		None
	}
	// text
	fn text_content(&self) -> String {
		return self.text_contents().iter().collect::<String>();
	}
	fn text(&self) -> String {
		self.text_content()
	}
	fn text_contents(&self) -> Vec<char>;
	// this don't decode the entity
	fn text_chars(&self) -> Vec<char>;
	// set text
	fn set_text(&mut self, content: &str);
	// set html
	fn set_html(&mut self, content: &str);
	// node index
	fn index(&self) -> usize;
}

#[cfg(test)]
mod tests {
	use super::INodeType;
	#[test]
	fn test_inode_type() {
		let element_type = INodeType::Element;
		assert!(format!("{:?}", element_type).contains("Element"));
		assert!(element_type.is_element());
	}
}
