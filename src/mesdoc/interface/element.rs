cfg_feat_text! {
	use super::Texts;
}
use super::{BoxDynNode, BoxDynText, Elements, INodeTrait, INodeType};
use crate::mesdoc::error::{BoxDynError, Error as IError};
use std::ops::Range;

pub type BoxDynElement<'a> = Box<dyn IElementTrait + 'a>;
pub type MaybeElement<'a> = Option<BoxDynElement<'a>>;

#[derive(Debug)]
pub enum IAttrValue {
	Value(String, Option<char>),
	True,
}

impl IAttrValue {
	/// pub fn `is_true`
	pub fn is_true(&self) -> bool {
		matches!(self, IAttrValue::True)
	}
	/// pub fn `is_str`
	pub fn is_str(&self, value: &str) -> bool {
		match self {
			IAttrValue::Value(v, _) => v == value,
			IAttrValue::True => value.is_empty(),
		}
	}
	/// pub fn `to_list`
	pub fn to_list(&self) -> Vec<&str> {
		match self {
			IAttrValue::Value(v, _) => v.trim().split_ascii_whitespace().collect::<Vec<&str>>(),
			IAttrValue::True => vec![],
		}
	}
}

/// impl `ToString` for IAttrValue
impl ToString for IAttrValue {
	fn to_string(&self) -> String {
		match self {
			IAttrValue::Value(v, _) => v.clone(),
			IAttrValue::True => String::new(),
		}
	}
}

/// IFormValue
#[derive(Debug)]
pub enum IFormValue {
	Single(String),
	Multiple(Vec<String>),
}

impl ToString for IFormValue {
	fn to_string(&self) -> String {
		match self {
			IFormValue::Single(v) => v.clone(),
			IFormValue::Multiple(v) => v.join(","),
		}
	}
}

// impl IntoIterator for IFormValue
impl IntoIterator for IFormValue {
	type Item = String;
	type IntoIter = std::vec::IntoIter<String>;
	fn into_iter(self) -> Self::IntoIter {
		match self {
			IFormValue::Multiple(v) => v.into_iter(),
			IFormValue::Single(_) => vec![].into_iter(),
		}
	}
}

cfg_feat_insertion! {
	#[derive(Debug, PartialEq, Eq)]
	pub enum InsertPosition {
		BeforeBegin,
		AfterBegin,
		BeforeEnd,
		AfterEnd,
	}

	impl InsertPosition {
		pub fn action(&self) -> &'static str {
			use InsertPosition::*;
			match self {
				BeforeBegin => "insert before",
				AfterBegin => "prepend",
				BeforeEnd => "append",
				AfterEnd => "insert after",
			}
		}
	}
}
pub trait IElementTrait: INodeTrait {
	fn is(&self, ele: &BoxDynElement) -> bool;
	fn is_root_element(&self) -> bool;
	/// The current version is an implementation of `Rc` pointers with non-copy semantics.
	/// If you want to achieve complete copying of nodes in the current version, please use the `copied` method instead.
	/// The semantics of this method may be changed in future versions to be consist with the `copied` method.
	fn cloned<'b>(&self) -> BoxDynElement<'b> {
		let ele = self.clone_node();
		ele.typed().into_element().unwrap()
	}
	/// Copy a element
	fn copied<'b>(&self) -> BoxDynElement<'b> {
		let ele = self.copy_node();
		ele.typed().into_element().unwrap()
	}
	// next sibling
	fn next_element_sibling<'b>(&self) -> MaybeElement<'b> {
		// use child_nodes instead of chilren, reduce one loop
		if let Some(parent) = &self.parent() {
			// self index
			let index = self.index();
			let total = parent.child_nodes_length();
			// find the next
			for cur_index in index + 1..total {
				let ele = parent
					.child_nodes_item(cur_index)
					.expect("Child nodes item index must less than total");
				if matches!(ele.node_type(), INodeType::Element) {
					return Some(
						ele
							.typed()
							.into_element()
							.expect("Call `typed` for element ele."),
					);
				}
			}
		}
		None
	}
	// next siblings
	fn next_element_siblings<'b>(&self) -> Elements<'b> {
		// use child_nodes instead of chilren, reduce one loop
		if let Some(parent) = &self.parent() {
			// self index
			let index = self.index();
			let total = parent.child_nodes_length();
			let start_index = index + 1;
			// find the next
			let mut result: Elements = Elements::with_capacity(total - start_index);
			for cur_index in start_index..total {
				let ele = parent
					.child_nodes_item(cur_index)
					.expect("Child nodes item index must less than total");
				if matches!(ele.node_type(), INodeType::Element) {
					result.push(
						ele
							.typed()
							.into_element()
							.expect("Call `typed` for element ele."),
					);
				}
			}
			return result;
		}
		Elements::new()
	}
	// previous sibling
	fn previous_element_sibling<'b>(&self) -> MaybeElement<'b> {
		// use child_nodes instead of chilren, reduce one loop
		if let Some(parent) = &self.parent() {
			// self index
			let index = self.index();
			if index > 0 {
				// find the prev
				for cur_index in (0..index).rev() {
					let ele = parent
						.child_nodes_item(cur_index)
						.expect("Child nodes item index must less than total");
					if matches!(ele.node_type(), INodeType::Element) {
						return Some(
							ele
								.typed()
								.into_element()
								.expect("Call `typed` for element ele."),
						);
					}
				}
			}
		}
		None
	}
	// previous siblings
	fn previous_element_siblings<'b>(&self) -> Elements<'b> {
		// use child_nodes instead of chilren, reduce one loop
		if let Some(parent) = &self.parent() {
			// self index
			let index = self.index();
			if index > 0 {
				// find the prev
				let mut result: Elements = Elements::with_capacity(index);
				for cur_index in 0..index {
					let ele = parent
						.child_nodes_item(cur_index)
						.expect("Child nodes item index must less than total");
					if matches!(ele.node_type(), INodeType::Element) {
						result.push(
							ele
								.typed()
								.into_element()
								.expect("Call `typed` for element ele."),
						);
					}
				}
				return result;
			}
		}
		Elements::new()
	}
	// siblings
	fn siblings<'b>(&self) -> Elements<'b> {
		// use child_nodes instead of chilren, reduce one loop
		if let Some(parent) = &self.parent() {
			// self index
			let index = self.index();
			if index == 0 {
				return self.next_element_siblings();
			}
			let total = parent.child_nodes_length();
			if index == total - 1 {
				return self.previous_element_siblings();
			}
			let mut result: Elements = Elements::with_capacity(total - 1);
			fn loop_handle(range: &Range<usize>, parent: &BoxDynElement, result: &mut Elements) {
				for cur_index in range.start..range.end {
					let ele = parent
						.child_nodes_item(cur_index)
						.expect("Child nodes item index must less than total");
					if matches!(ele.node_type(), INodeType::Element) {
						result.push(
							ele
								.typed()
								.into_element()
								.expect("Call `typed` for element ele."),
						);
					}
				}
			}
			loop_handle(&(0..index), parent, &mut result);
			loop_handle(&(index + 1..total), parent, &mut result);
			return result;
		}
		Elements::new()
	}
	// value
	fn value(&self) -> IFormValue;
	// tag name
	fn tag_name(&self) -> String {
		self
			.tag_names()
			.iter()
			.map(|ch| ch.to_ascii_uppercase())
			.collect::<String>()
	}
	fn tag_names(&self) -> Vec<char>;
	// element child nodes
	fn child_nodes_length(&self) -> usize;
	fn child_nodes_item<'b>(&self, index: usize) -> Option<BoxDynNode<'b>>;
	fn child_nodes_item_since_by<'a>(
		&'a self,
		node_index: usize,
		reverse: bool,
		handle: Box<dyn FnMut(&dyn IElementTrait) -> bool + 'a>,
	);
	fn child_nodes<'b>(&self) -> Vec<BoxDynNode<'b>> {
		let total = self.child_nodes_length();
		let mut result = Vec::with_capacity(total);
		for index in 0..total {
			result.push(
				self
					.child_nodes_item(index)
					.expect("child nodes index must less than total."),
			);
		}
		result
	}
	// children
	fn children<'b>(&self) -> Elements<'b> {
		let total = self.child_nodes_length();
		let mut result = Elements::with_capacity(total);
		for index in 0..total {
			let node = self
				.child_nodes_item(index)
				.expect("child nodes index must less than total.");
			if let INodeType::Element = node.node_type() {
				result.push(node.typed().into_element().unwrap());
			}
		}
		result
	}
	fn children_by<'a>(&'a self, matcher: Box<dyn FnMut(&dyn IElementTrait) + 'a>);
	// attribute
	fn get_attribute(&self, name: &str) -> Option<IAttrValue>;
	fn set_attribute(&mut self, name: &str, value: Option<&str>);
	fn remove_attribute(&mut self, name: &str);
	fn has_attribute(&self, name: &str) -> bool {
		self.get_attribute(name).is_some()
	}
	// html
	fn html(&self) -> String {
		self.inner_html()
	}
	fn inner_html(&self) -> String;
	fn outer_html(&self) -> String;

	// append child, insert before
	cfg_feat_insertion! {
		fn insert_adjacent(&mut self, position: &InsertPosition, node: &BoxDynElement);
		fn replace_with(&mut self, node: &BoxDynElement);
	}
	cfg_feat_mutation! {
		// remove child
		fn remove_child(&mut self, ele: BoxDynElement);
	}
	// texts
	cfg_feat_text! {
		/// texts
		fn texts<'b>(&self, limit_depth: usize) -> Option<Texts<'b>> {
			let handle = Box::new(|_: usize, _: &BoxDynText| true);
			self.texts_by(limit_depth, &handle)
		}
		/// texts_by
		fn texts_by<'b>(
			&self,
			limit_depth: usize,
			handle: &dyn Fn(usize, &BoxDynText) -> bool,
		) -> Option<Texts<'b>> {
			self.texts_by_rec(limit_depth, handle, &Box::new(|_: &BoxDynElement|true))
		}
		/// texts_by_rec
		fn texts_by_rec<'b>(
			&self,
			_limit_depth: usize,
			_handle: &dyn Fn(usize, &BoxDynText) -> bool,
			_rec_handle: &dyn Fn(&BoxDynElement) -> bool
		) -> Option<Texts<'b>>{
			None
		}
	}
	// special for content tag, 'style','script','title','textarea'
	#[allow(clippy::boxed_local)]
	fn into_text<'b>(self: Box<Self>) -> Result<BoxDynText<'b>, BoxDynError> {
		Err(Box::new(IError::InvalidTraitMethodCall {
			method: "into_text".into(),
			message: "The into_text method is not implemented.".into(),
		}))
	}
}

#[cfg(test)]
mod tests {
	use super::IAttrValue;
	#[test]
	fn test_i_attr_value() {
		// string value
		let attr_value = IAttrValue::Value("Hello".into(), None);
		assert!(format!("{:?}", attr_value).contains("Hello"));
		assert!(attr_value.is_str("Hello"));
		assert!(!attr_value.is_true());
		assert_eq!(attr_value.to_list(), vec!["Hello"]);
		assert_eq!(attr_value.to_string(), "Hello");
		// flag value
		let attr_value = IAttrValue::True;
		assert!(attr_value.is_str(""));
		assert_eq!(attr_value.to_string(), "");
		assert!(attr_value.is_true());
		assert!(attr_value.to_list().is_empty());
	}
}
