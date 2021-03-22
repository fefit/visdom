use std::error::Error;
use std::result::Result as StdResult;
use visdom::Vis;
type Result = StdResult<(), Box<dyn Error>>;

#[test]
fn test_attribute_selector() -> Result {
	let html = r##"
  <nav id="lang">
    <a href="#" lang="en">en</a>
    <a href="#" lang="en-UK">en-UK</a>
    <a href="#" lang="english">english</a>
    <a href="#" lang="uk en">uk-en</a>
  </nav>
  "##;
	let root = Vis::load(&html)?;
	let lang = root.find("#lang");
	let links = lang.children("");
	// equal
	let equal_en = links.filter("[lang='en']");
	assert_eq!(equal_en.length(), 1);
	// begin with
	let begin_en = links.filter("[lang^='en']");
	assert_eq!(begin_en.length(), 3);
	// end with
	let end_en = links.filter("[lang$='en']");
	assert_eq!(end_en.length(), 2);
	// appear
	let appear_en = links.filter("[lang*='en']");
	assert_eq!(appear_en.length(), 4);
	// equal or begin with `en-`
	let split_en = links.filter("[lang|= en]");
	assert_eq!(split_en.length(), 2);
	// not equal to  `en`
	let not_en = links.filter("[lang!=en]");
	assert_eq!(not_en.length(), 3);
	// split list en
	let ws_en = links.filter("[lang~='en']");
	assert_eq!(ws_en.length(), 2);
	Ok(())
}

#[test]
fn test_id_selector() -> Result {
	let html = r##"
  <nav id="lang">
    <a id="link"></a>
  </nav>
  "##;
	let root = Vis::load(&html)?;
	let lang = root.find("#lang");
	assert_eq!(lang.length(), 1);
	// link
	let link = root.find("#link");
	assert_eq!(link.length(), 1);
	// nested
	let link = root.find("#lang #link");
	assert_eq!(link.length(), 1);
	// not found
	let link = root.find("#none #link");
	assert_eq!(link.length(), 0);
	Ok(())
}

#[test]
fn test_class_selector() -> Result {
	let html = r##"
  <nav id="lang">
    <a class="en link"></a>
    <a class="en-US link"></a>
    <span class="en"></span>
  </nav>
  "##;
	let root = Vis::load(&html)?;
	let lang = root.find("#lang");
	assert_eq!(lang.find(".link").length(), 2);
	assert_eq!(lang.find(".en").length(), 2);
	assert_eq!(lang.find(".en.link").length(), 1);
	assert_eq!(lang.find("a.link[class|='en']").length(), 1);
	Ok(())
}

#[test]
fn test_tagname_selector() -> Result {
	// ignore tag name cases
	let html = r##"
    <Div></div>
  "##;
	let root = Vis::load(html)?;
	let div = root.find("div");
	assert_eq!(div.length(), 1);
	assert_eq!(div.get(0).unwrap().tag_name(), "DIV");
	// tagname with namespace
	let html = r##"
    <Form:Item></Form:Item>
  "##;
	let root = Vis::load(html)?;
	let item = root.find("FORM\\:ITEM");
	assert_eq!(item.length(), 1);
	assert_eq!(item.get(0).unwrap().tag_name(), "FORM:ITEM");
	Ok(())
}

#[test]
fn test_selector_pseudo_header() -> Result {
	let html = r#"<h1></h1><div></div>"#;
	let root = Vis::load(html)?;
	let hgroups = root.find(":header");
	assert_eq!(hgroups.length(), 1);
	let not_hgroups = root.find(":not(:header)");
	assert_eq!(not_hgroups.length(), 1);
	Ok(())
}

#[test]
fn test_selector_pseudo_contains() -> Result {
	let html = r#"<h1>abc</h1><div>a&amp;</div>"#;
	let root = Vis::load(html)?;
	// has 'a'
	let text_a = root.find(":contains('a')");
	assert_eq!(text_a.length(), 2);
	// 'b'
	let text_b = root.find(":contains('b')");
	assert_eq!(text_b.length(), 1);
	// escape ;
	let text_escape = root.find(":contains(\"&\")");
	assert_eq!(text_escape.length(), 1);
	// more
	let html = r##"
  <div id="content">
    <p>Visdom</p>
    <p>
      Vis<span>dom</span>!
    </p>
    <p>
      Vis&nbsp;<span>dom</span>!
    </p>
  </div>
  "##;
	let root = Vis::load(&html)?;
	let content = root.find("#content");
	assert_eq!(content.find("p:contains('Visdom')").length(), 2);
	// npsp; &#160; space: &#32;
	assert_eq!(content.find("p:contains('Vis dom')").length(), 0);
	Ok(())
}

#[test]
fn test_selector_pseudo_empty() -> Result {
	let html = r#"<h1>abc</h1><div></div><p><!--comment--></p><b> </b>"#;
	let root = Vis::load(html)?;
	// empty
	let empty = root.find(":empty");
	assert_eq!(empty.length(), 2);
	Ok(())
}

#[test]
fn test_selector_pseudo_nth_last_of_type() -> Result {
	let html = r#"
    <!doctype html>
    <html lang="en">
      <head>
        <meta charset="utf-8">
        <title>:nth-last-of-type</title>
      </head>
    <body>
      <dl>
        <dt>dt1</dt>
          <dd>dd1</dd>
          <dd>dd2</dd>
          <dd>dd3</dd>
        <dt>dt2</dt>
          <dd>dd4</dd>
        <dt>dt3</dt>
          <dd>dd5</dd>
          <dd>dd6</dd>
      </dl>
    </body>
    </html>
  "#;
	let root = Vis::load(&html)?;
	let dl = root.find("dl");
	// :nth-last-of-type(1)
	let last_type_child = dl.children(":nth-last-of-type(1)");
	assert_eq!(last_type_child.length(), 2);
	assert_eq!(last_type_child.text(), "dt3dd6");
	// :nth-last-of-type(odd)
	let last_odd_type_childs = dl.children(":nth-last-of-type(odd)");
	assert_eq!(last_odd_type_childs.length(), 5);
	assert_eq!(last_odd_type_childs.text(), "dt1dd2dd4dt3dd6",);
	// :nth-last-of-type(3n)
	let childs_type_last_3n = dl.children(":nth-last-of-type(3n)");
	assert_eq!(childs_type_last_3n.length(), 3);
	assert_eq!(childs_type_last_3n.text(), "dt1dd1dd4");
	Ok(())
}
