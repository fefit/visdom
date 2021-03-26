use std::error::Error;
use std::result::Result as StdResult;
use visdom::Vis;
type Result = StdResult<(), Box<dyn Error>>;

#[test]
fn test_set_html() -> Result {
	let html: &str = r#"<div class="parent"></div>"#;
	// normal tag
	let root = Vis::load(html)?;
	let mut parent = root.children(".parent");
	let setted = "This is a <strong>test</strong>!";
	let only_text = "This is a test!";
	parent.set_html(setted);
	assert_eq!(parent.text(), only_text);
	assert_eq!(parent.children("strong").length(), 1);
	assert_eq!(parent.html(), setted);
	parent.set_html("");
	assert!(parent.html().is_empty());
	// pre tag
	let html: &str = r#"<pre class="parent"></pre>"#;
	let root = Vis::load(html)?;
	let mut parent = root.children(".parent");
	parent.set_html(setted);
	assert_eq!(parent.html(), setted);
	assert_eq!(parent.text(), only_text);
	assert_eq!(parent.children("strong").length(), 1);
	parent.set_html("");
	assert!(parent.html().is_empty());
	// text node
	let text = only_text;
	let html = format!(r#"<div class="parent">{}</div>"#, text);
	let root = Vis::load(&html)?;
	let parent = root.children(".parent");
	let mut texts = parent.texts(1);
	assert_eq!(texts.length(), 1);
	texts.for_each(|_, node| {
		assert_eq!(node.text(), text);
		node.set_html("This is a <strong>test</strong>!");
		true
	});
	assert_eq!(parent.text(), text);
	assert_eq!(parent.children("strong").length(), 1);
	assert_eq!(parent.children("strong").text(), "test");
	Ok(())
}

#[test]
fn test_set_text() -> Result {
	let html: &str = r#"<div class="parent"></div>"#;
	// normal tag
	let root = Vis::load(html)?;
	let mut parent = root.children(".parent");
	let setted = "This is a <strong>test</strong>!";
	let encoded_setted = "This is a &lt;strong&gt;test&lt;/strong&gt;!";
	parent.set_text(setted);
	assert_eq!(parent.text(), setted);
	assert_eq!(parent.children("strong").length(), 0);
	assert_eq!(parent.html(), encoded_setted);
	parent.set_text("");
	assert!(parent.text().is_empty());
	// content tag
	let html: &str = r#"<pre class="parent"></pre>"#;
	let root = Vis::load(html)?;
	let mut parent = root.children(".parent");
	parent.set_text(setted);
	assert_eq!(parent.html(), encoded_setted);
	assert_eq!(parent.text(), setted);
	assert_eq!(parent.children("strong").length(), 0);
	parent.set_text("");
	assert!(parent.text().is_empty());
	Ok(())
}

#[test]
fn test_inner_html() -> Result {
	let inner_html = "abc<span>def</span>ghj";
	let code = format!("<div>{}</div>", inner_html);
	let root = Vis::load(&code)?;
	assert_eq!(root.find("div").get(0).unwrap().html(), inner_html);
	assert_eq!(root.find("div").html(), inner_html);
	assert_eq!(root.find("p").html(), "");
	Ok(())
}

#[test]
fn test_outer_html() -> Result {
	let inner_html = "abc<span>def</span>ghj";
	let code = format!("<div>{}</div>", inner_html);
	let root = Vis::load(&code)?;
	assert_eq!(root.find("div").get(0).unwrap().outer_html(), code);
	assert_eq!(root.find("div").outer_html(), code);
	assert_eq!(root.find("p").outer_html(), "");
	Ok(())
}
