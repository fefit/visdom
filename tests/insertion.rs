#![cfg(feature = "insertion")]
use std::result::Result as StdResult;
use visdom::types::BoxDynError;
use visdom::Vis;
type Result = StdResult<(), BoxDynError>;

#[test]
fn test_append_child() -> Result {
	const HTML: &str = r#"<div class="parent"><div class="first-child"></div></div>"#;
	let root = Vis::load(HTML)?;
	let mut parent = root.children(".parent");
	let first_child = parent.children(".first-child");
	let mut new_childs =
		Vis::load(r#"<div class="second-child"></div><div class="third-child"></div>"#)?;
	assert_eq!(0, first_child.get(0).unwrap().index());
	parent.append(&mut new_childs);
	assert_eq!(0, first_child.get(0).unwrap().index());
	let all_childs = parent.children("");
	let last_child = all_childs.last();
	assert_eq!(2, last_child.get(0).unwrap().index());
	// empty
	let mut empty = Vis::load("")?;
	empty.append_to(&mut parent);
	let last_child = parent.children("").last();
	assert_eq!(2, last_child.get(0).unwrap().index());
	Ok(())
}

#[test]
fn test_prepend_child() -> Result {
	const HTML: &str = r#"<div class="parent"><div class="third-child"></div></div>"#;
	let root = Vis::load(HTML)?;
	let mut parent = root.children(".parent");
	let last_child = parent.children(".third-child");
	let mut new_childs =
		Vis::load(r#"<div class="first-child"></div><div class="second-child"></div>"#)?;
	assert_eq!(0, last_child.get(0).unwrap().index());
	new_childs.prepend_to(&mut parent);
	assert_eq!(2, last_child.get(0).unwrap().index());
	let all_childs = parent.children("");
	let first_child = all_childs.eq(0);
	assert_eq!(0, first_child.get(0).unwrap().index());
	Ok(())
}

#[test]
fn test_insert_before() -> Result {
	const HTML: &str = r#"<div class="parent"><div class="third-child"></div></div>"#;
	let root = Vis::load(HTML)?;
	let parent = root.children(".parent");
	let mut third_child = parent.children(".third-child");
	let inserted = Vis::load(r#"<div class="first-child"></div><div class="second-child"></div>"#)?;
	let inserted_childs = inserted.children("");
	assert_eq!(0, third_child.get(0).unwrap().index());
	// append second child
	let mut second_child = inserted_childs.filter(".second-child");
	second_child.insert_before(&mut third_child);
	assert_eq!(1, third_child.get(0).unwrap().index());
	assert_eq!(0, second_child.get(0).unwrap().index());
	assert_eq!(1, inserted.children("").length());
	// append first_child
	let mut first_child = inserted_childs.filter(".first-child");
	first_child.insert_before(&mut second_child);
	assert_eq!(2, third_child.get(0).unwrap().index());
	assert_eq!(1, second_child.get(0).unwrap().index());
	assert_eq!(0, first_child.get(0).unwrap().index());
	assert_eq!(0, inserted.children("").length());
	Ok(())
}

#[test]
fn test_insert_after() -> Result {
	const HTML: &str = r#"<div class="parent"><div class="first-child"></div></div>"#;
	let root = Vis::load(HTML)?;
	let parent = root.children(".parent");
	let mut first_child = parent.children(".first-child");
	let inserted = Vis::load(r#"<div class="second-child"></div><div class="third-child"></div>"#)?;
	let inserted_childs = inserted.children("");
	assert_eq!(0, first_child.get(0).unwrap().index());
	// append second child
	let mut second_child = inserted_childs.filter(".second-child");
	second_child.insert_after(&mut first_child);
	assert_eq!(0, first_child.get(0).unwrap().index());
	assert_eq!(1, second_child.get(0).unwrap().index());
	assert_eq!(1, inserted.children("").length());
	// append third_child
	let mut third_child = inserted_childs.filter(".third-child");
	third_child.insert_after(&mut second_child);
	assert_eq!(2, third_child.get(0).unwrap().index());
	assert_eq!(1, second_child.get(0).unwrap().index());
	assert_eq!(0, first_child.get(0).unwrap().index());
	assert_eq!(0, inserted.children("").length());
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
fn test_allow_insert() -> Result {
	// --- void tags, not allowed insert any html ---
	let html = r#"<div id="content"><img src="picture.jpg" /></div>"#;
	let root = Vis::load(html)?;
	// set html will make no sence
	let mut img = root.find("img");
	img.set_html("<div class='test'></div>");
	assert_eq!(img.html(), "");
	// append
	let mut childs = Vis::load("abc<span>def</span><!--ghi-->")?;
	childs.append_to(&mut img);
	assert_eq!(img.html(), "");
	// ----- title -----
	let html = r#"<title></title>"#;
	let root = Vis::load(html)?;
	let mut title = root.find("title");
	title.set_html("ab<span></span>cd");
	assert_eq!(title.text(), "ab<span></span>cd");
	title.empty();
	let mut content = Vis::load("ab<span></span>cd")?;
	content.append_to(&mut title);
	assert_eq!(title.text(), "abcd");
	// ----- insert self----
	let html = r#"<div id="wrapper"><div id="inner"></div></div>"#;
	let root = Vis::load(html)?;
	let mut wrapper = root.find("#wrapper");
	let mut inner = wrapper.find("#inner");
	// insert parent to child, will make no sence
	wrapper.append_to(&mut inner);
	assert_eq!(wrapper.find("#inner").length(), 1);
	// insert to self, will not allowed by rust
	// inner.append_to(&mut inner);
	Ok(())
}

#[test]
#[should_panic]
fn test_append_wrong_document() {
	let html = r#"
  <!doctype html>
  <html>
    <head></head>
    <body>
      <div id="main">
      </div>
    </body>
  </html>"#;
	let mut root = Vis::load_catch(
		html,
		Box::new(|e| {
			panic!("{}", e.to_string());
		}),
	);
	let mut main = root.find("#main");
	main.append(&mut root);
}

#[test]
#[should_panic]
fn test_append_wrong_itself() {
	let html = r#"
  <!doctype html>
  <html>
    <head></head>
    <body>
      <div id="main">
      </div>
    </body>
  </html>"#;
	let root = Vis::load_catch(
		html,
		Box::new(|e| {
			panic!("{}", e.to_string());
		}),
	);
	let mut main = root.find("#main");
	let mut still_main = root.find("#main");
	main.append(&mut still_main);
}

#[test]
#[should_panic]
fn test_append_wrong_parent() {
	let html = r#"
  <!doctype html>
  <html>
    <head></head>
    <body>
      <div id="main">
        <div id="container"></div>
      </div>
    </body>
  </html>"#;
	let root = Vis::load_catch(
		html,
		Box::new(|e| {
			panic!("{}", e.to_string());
		}),
	);
	let mut child = root.find("#container");
	let mut parent = root.find("#main");
	child.append(&mut parent);
}
