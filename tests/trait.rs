use std::result::Result as StdResult;
use visdom::types::BoxDynError;
use visdom::Vis;
type Result = StdResult<(), BoxDynError>;

#[test]
fn test_document_trait() -> Result {
	let page_title = "Vis<dom>";
	let html = format!(
		r#"
    <!doctype html>
    <html>
      <head>
        <title>{}</title>
      </head>
      <body>
        Visdom!
      </body>
    </html>
  "#,
		page_title
	);
	let root = Vis::load(&html)?;
	// get document
	let doc = root.document();
	assert!(doc.is_some());
	// doc methods
	let doc = doc.unwrap();
	// title
	let title = doc.title();
	assert!(title.is_some() && title.unwrap() == page_title);
	// head
	let head = doc.head();
	assert!(head.is_some() && head.unwrap().children().filter("title").text() == page_title);
	// body
	let body = doc.body();
	assert!(body.is_some() && body.as_ref().unwrap().previous_element_sibling().is_some());
	assert_eq!(
		body
			.as_ref()
			.unwrap()
			.previous_element_sibling()
			.unwrap()
			.tag_name(),
		"HEAD"
	);
	// source code
	assert_eq!(doc.source_code(), html);
	// document element
	let doc_element = doc.document_element();
	assert!(doc_element.is_some());
	assert_eq!(doc_element.unwrap().tag_name(), "HTML");
	// document fragement
	let html = r##"<div>just a document fragement</div>"##;
	let root = Vis::load(html)?;
	let doc = root.document();
	assert!(doc.is_some());
	let doc = doc.unwrap();
	// no title, head, body, documentElement
	assert!(doc.title().is_none());
	assert!(doc.head().is_none());
	assert!(doc.document_element().is_none());
	assert!(doc.body().is_none());
	Ok(())
}

#[cfg(feature = "text")]
#[test]
fn test_text_trait() -> Result {
	let html = r#"
    <!doctype html>
    <html>
      <head>
        <title>test text trait</title>
      </head>
      <body>
        <div id="content">Vis<span>dom</span></div>
      </body>
    </html>
  "#;
	let root = Vis::load(html)?;
	// content
	let id_content = root.find("#content");
	// get all texts
	let mut texts = id_content.texts(0);
	assert_eq!(texts.length(), 2);
	assert!(texts.get_ref().get(0).unwrap().text() == "Vis");
	assert!(texts.get_ref().get(1).unwrap().text() == "dom");
	// append text, prepend text
	texts.for_each(|_, node| {
		node.prepend_text("^");
		node.append_text("$");
		true
	});
	assert!(texts.get_ref().get(0).unwrap().text() == "^Vis$");
	assert!(texts.get_ref().get(1).unwrap().text() == "^dom$");
	// remove text
	texts.remove();
	// get now texts
	let texts = id_content.texts(0);
	assert_eq!(texts.length(), 0);
	// append text for content tag
	let root = Vis::load("<script></script>")?;
	let mut script_text = root.find("script").texts(1);
	script_text.for_each(|_, text_node| {
		assert_eq!(text_node.text(), "");
		text_node.prepend_text("var a;");
		text_node.append_text("var b;");
		assert_eq!(text_node.text(), "var a;var b;");
		true
	});
	// style
	let root = Vis::load("<style></style>")?;
	let mut style_text = root.find("style").texts(1);
	style_text.for_each(|_, text_node| {
		assert_eq!(text_node.text(), "");
		text_node.append_text("{}");
		text_node.prepend_text("body");
		assert_eq!(text_node.text(), "body{}");
		true
	});
	// text_contents vs text_chars
	let root = Vis::load(r#"<a>&lt;span&gt;&amp;</a>"#)?;
	let a_link = root.find("a");
	let mut texts = a_link.texts(1);
	texts.for_each(|_, ele| {
		assert_eq!(ele.text(), "<span>&");
		assert_eq!(
			ele.text_chars().iter().collect::<String>(),
			"&lt;span&gt;&amp;"
		);
		true
	});
	Ok(())
}

#[test]
fn test_node_trait() -> Result {
	let html = r#"
    <!doctype html>
    <html>
      <head>
        <title>test text trait</title>
      </head>
      <body>
        <div id="content">Vis<span>dom</span></div>
      </body>
    </html>
  "#;
	let root = Vis::load(html)?;
	// get root
	let root_element = root.get(0).unwrap().root_element().unwrap();
	assert!(root_element.is(&root_element.root_element().unwrap()));
	Ok(())
}

#[test]
#[cfg(feature = "text")]
fn test_node_text_trait() -> Result {
	let html = r#"
    <!doctype html>
    <html>
      <head>
        <title>test text trait</title>
      </head>
      <body>
        <div id="content">Vis<span>dom</span></div>
      </body>
    </html>
  "#;
	let root = Vis::load(html)?;
	// content
	let content = root.find("#content");
	assert!(content
		.get(0)
		.unwrap()
		.clone_node()
		.typed()
		.into_text()
		.is_none());
	let mut texts = content.texts(1);
	texts.for_each(|_, text_node| {
		assert!(text_node.clone_node().typed().into_element().is_none());
		true
	});
	Ok(())
}
