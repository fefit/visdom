use std::error::Error;
use std::result::Result as StdResult;
use visdom::Vis;
type Result = StdResult<(), Box<dyn Error>>;

#[test]
fn test_normal_attr() -> Result {
	const ATTR_NAME: &str = "contenteditable";
	const HTML: &str = r#"<div contenteditable><input type="text" type="file" /></div>"#;
	let root = Vis::load(HTML)?;
	let mut div = root.children("div");
	// get attribute
	let value = div.attr(ATTR_NAME);
	assert!(value.is_some() && value.unwrap().is_true());
	// remove attribute
	div.remove_attr(ATTR_NAME);
	let now_value = div.attr(ATTR_NAME);
	assert!(now_value.is_none());
	// set again
	div.set_attr(ATTR_NAME, None);
	let value = div.attr(ATTR_NAME);
	assert!(value.is_some() && value.unwrap().is_true());
	// always get the first appeared attribute
	let input = div.children("input");
	let value = input.attr("type");
	assert!(value.is_some() && value.unwrap().is_str("text"));
	Ok(())
}

#[test]
fn test_class_attr() -> Result {
	const HTML: &str = r#"<div class="first"></div>"#;
	const ATTR_NAME: &str = "class";
	let root = Vis::load(HTML)?;
	let mut div = root.children("div");
	// get class
	let value = div.attr(ATTR_NAME);
	assert!(value.is_some() && value.unwrap().is_str("first"));
	// remove class
	div.remove_class("first");
	let now_value = div.attr(ATTR_NAME);
	assert!(now_value.is_some() && now_value.unwrap().is_str(""));
	// set again
	div.add_class("first  second");
	let value = div.attr(ATTR_NAME);
	assert!(value.is_some());
	// get the class list
	let value = value.unwrap();
	let cls_list = value.to_list();
	assert!(cls_list.contains(&"first"));
	assert!(cls_list.contains(&"second"));
	assert!(value.is_str("first second"));
	// toggle class, remove "first", add "third"
	div.toggle_class("first third");
	assert!(!div.has_class("first"));
	assert!(div.has_class("second"));
	assert!(div.has_class("third"));
	let value = div.attr(ATTR_NAME).unwrap();
	assert!(value.is_str("second third"));
	Ok(())
}
