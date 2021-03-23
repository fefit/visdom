use std::error::Error;
use std::result::Result as StdResult;
use visdom::Vis;
type Result = StdResult<(), Box<dyn Error>>;

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
	assert_eq!(doc.is_some(), true);
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
	// document element
	let doc_element = doc.document_element();
	assert!(doc_element.is_some());
	assert_eq!(doc_element.unwrap().tag_name(), "HTML");
	Ok(())
}

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
	assert_eq!(root_element.is(&root_element.root_element().unwrap()), true);
	Ok(())
}
