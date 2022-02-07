use std::result::Result as StdResult;
use visdom::{
	types::{BoxDynError, INodeType},
	Vis,
};
type Result = StdResult<(), BoxDynError>;

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
	// pre tag
	let html: &str = r#"<pre class="parent"></pre>"#;
	let root = Vis::load(html)?;
	let mut parent = root.children(".parent");
	parent.set_text(setted);
	assert_eq!(parent.html(), encoded_setted);
	assert_eq!(parent.text(), setted);
	assert_eq!(parent.children("strong").length(), 0);
	parent.set_text("");
	assert!(parent.text().is_empty());
	// script tag
	let inner_script = "var a = 1;";
	let html = format!(r#"<script>{}</script>"#, inner_script);
	let root = Vis::load(&html)?;
	let mut script = root.find("script");
	assert_eq!(script.length(), 1);
	assert_eq!(script.text(), inner_script);
	// set text
	let inner_script = "var b = 2;";
	script.set_text(inner_script);
	assert_eq!(script.text(), inner_script);
	// style tag
	let root = Vis::load("<style></style>")?;
	let mut style = root.find("style");
	assert_eq!(style.length(), 1);
	assert_eq!(style.text(), "");
	let inner_style = "body{background:blue;}";
	style.set_html(inner_style);
	assert_eq!(style.text(), inner_style);
	assert_eq!(style.html(), inner_style);
	Ok(())
}

#[test]
fn test_text_content() {}

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

#[test]
fn test_texts() -> Result {
	let html = r##"
    <div id="content">FIRST-ABC<div>SECOND-ABC<style>.a{{color:red}}</style>SECOND-DEF</div><script>var a = 1;</script>FIRST-DEF</div>
  "##;
	let root = Vis::load(html)?;
	let content = root.find("#content");
	println!("content===>{}", content.length());
	let texts = content.texts(0);
	assert_eq!(texts.length(), 6);
	// top childs
	let texts_limit = content.texts(1);
	assert_eq!(texts_limit.length(), 3);
	// filters, ignore content nodes such as style/script
	let texts_filter = content.texts_by(
		0,
		Box::new(|_, node| !matches!(node.node_type(), INodeType::Element)),
	);
	assert_eq!(texts_filter.length(), 4);
	// filter also with limit depth
	let texts_filter = content.texts_by(
		1,
		Box::new(|_, node| !matches!(node.node_type(), INodeType::Element)),
	);
	assert_eq!(texts_filter.length(), 2);
	// just content tags
	let html = r##"<script>var a = 1;</script>"##;
	let root = Vis::load(html)?;
	let script = root.find("script");
	let mut texts = script.texts(0);
	assert_eq!(texts.length(), 1);
	texts.for_each(|_, node| {
		assert_eq!(node.text(), "var a = 1;");
		true
	});
	// filter content tags
	let texts = script.texts_by(
		0,
		Box::new(|_, node| !matches!(node.node_type(), INodeType::Element)),
	);
	assert_eq!(texts.length(), 0);
	Ok(())
}
