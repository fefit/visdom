#![cfg(any(feature = "destory", feature = "insertion"))]
use std::result::Result as StdResult;
use visdom::types::BoxDynError;
use visdom::Vis;
type Result = StdResult<(), BoxDynError>;

#[test]
fn test_remove_child() -> Result {
	const HTML: &str = r#"<div><p></p><ul></ul><ol></ol></div>"#;
	let root = Vis::load(HTML)?;
	let div = root.children("div");
	let p = div.children("p");
	let ul = div.children("ul");
	let ol = div.children("ol");
	// remove before
	assert_eq!(0, p.get(0).unwrap().index());
	assert_eq!(1, ul.get(0).unwrap().index());
	assert_eq!(2, ol.get(0).unwrap().index());
	p.remove();
	assert_eq!(0, ul.get(0).unwrap().index());
	assert_eq!(1, ol.get(0).unwrap().index());
	// remove center
	let root = Vis::load(HTML)?;
	let div = root.children("div");
	let p = div.children("p");
	let ul = div.children("ul");
	let ol = div.children("ol");
	ul.remove();
	assert_eq!(0, p.get(0).unwrap().index());
	assert_eq!(1, ol.get(0).unwrap().index());
	// remove after
	let root = Vis::load(HTML)?;
	let div = root.children("div");
	let p = div.children("p");
	let ul = div.children("ul");
	let ol = div.children("ol");
	ol.remove();
	assert_eq!(0, p.get(0).unwrap().index());
	assert_eq!(1, ul.get(0).unwrap().index());
	Ok(())
}

#[test]
fn test_empty() -> Result {
	let html = r#"<div id="content">This is a <strong>test</strong>!</div>"#;
	let root = Vis::load(html)?;
	let mut content = root.find("#content");
	assert_eq!(content.length(), 1);
	assert_eq!(content.children("strong").length(), 1);
	content.empty();
	assert_eq!(content.children("strong").length(), 0);
	assert_eq!(content.html(), "");
	Ok(())
}

#[test]
fn test_remove() -> Result {
	let html = r#"<div id="content">This is a <strong>test</strong>!</div>"#;
	let root = Vis::load(html)?;
	let content = root.find("#content");
	assert_eq!(content.find("strong").length(), 1);
	content.find("strong").remove();
	assert_eq!(content.find("strong").length(), 0);
	assert_eq!(content.text(), "This is a !");
	Ok(())
}
