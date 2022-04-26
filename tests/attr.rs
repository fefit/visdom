use std::result::Result as StdResult;
use visdom::types::BoxDynError;
use visdom::Vis;
type Result = StdResult<(), BoxDynError>;

#[test]
fn test_normal_attr() -> Result {
	const ATTR_NAME: &str = "contenteditable";
	const HTML: &str = r#"<div contenteditable><input type="text" type="file" /></div>"#;
	let root = Vis::load(HTML)?;
	let mut div = root.children("div");
	// has attribute
	assert!(div.has_attr(ATTR_NAME));
	assert!(!div.has_attr("content"));
	// get attribute
	let value = div.attr(ATTR_NAME);
	assert!(value.is_some() && value.unwrap().is_true());
	assert!(root.find("p").attr(ATTR_NAME).is_none());
	// remove attribute
	div.remove_attr(ATTR_NAME);
	let now_value = div.attr(ATTR_NAME);
	assert!(now_value.is_none());
	// set again
	div.set_attr(ATTR_NAME, None);
	let value = div.attr(ATTR_NAME);
	assert!(value.is_some());
	assert!(value.as_ref().unwrap().is_true());
	assert!(value.as_ref().unwrap().is_str(""));
	assert!(value.as_ref().unwrap().to_string() == "");
	assert!(value.as_ref().unwrap().to_list().is_empty());
	// always get the first appeared attribute
	let input = div.children("input");
	let value = input.attr("type");
	assert!(value.is_some());
	assert!(value.as_ref().unwrap().is_str("text"));
	assert!(value.as_ref().unwrap().to_string() == "text");
	assert_eq!(value.as_ref().unwrap().to_list(), vec!["text"]);
	// ignore attribute cases: issue #2
	let html: &str = r#"<input type="text" READONly /></div>"#;
	let root = Vis::load(html)?;
	let mut input = root.children("[readOnly]");
	assert_eq!(input.length(), 1);
	let title = "this's a title";
	input.set_attr("title", Some(title));
	assert_eq!(input.attr("title").unwrap().to_string(), title);
	let title = "\"this's a\" title";
	input.set_attr("title", Some(title));
	assert_eq!(
		input.attr("title").unwrap().to_string(),
		title.replace('\'', "&apos;")
	);
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
	// remove class and set again
	div.remove_attr("class");
	assert!(!div.has_attr("class"));
	div.add_class("first");
	assert!(div.has_class("first"));
	// remove class and toggle again
	div.remove_attr("class");
	div.toggle_class("first second");
	assert!(div.has_class("first"));
	assert!(div.has_class("second"));
	Ok(())
}
