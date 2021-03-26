use std::error::Error;
use std::result::Result as StdResult;
use visdom::Vis;
type Result = StdResult<(), Box<dyn Error>>;

#[test]
fn test_dom_remove_child() -> Result {
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
fn test_dom_append_child() -> Result {
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
	parent.append(&mut empty);
	let last_child = parent.children("").last();
	assert_eq!(2, last_child.get(0).unwrap().index());
	Ok(())
}

#[test]
fn test_dom_prepend_child() -> Result {
	const HTML: &str = r#"<div class="parent"><div class="third-child"></div></div>"#;
	let root = Vis::load(HTML)?;
	let mut parent = root.children(".parent");
	let last_child = parent.children(".third-child");
	let mut new_childs =
		Vis::load(r#"<div class="first-child"></div><div class="second-child"></div>"#)?;
	assert_eq!(0, last_child.get(0).unwrap().index());
	parent.prepend(&mut new_childs);
	assert_eq!(2, last_child.get(0).unwrap().index());
	let all_childs = parent.children("");
	let first_child = all_childs.eq(0);
	assert_eq!(0, first_child.get(0).unwrap().index());
	Ok(())
}

#[test]
fn test_dom_insert_before() -> Result {
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
fn test_dom_insert_after() -> Result {
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
fn test_dom_set_html() -> Result {
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
fn test_dom_set_text() -> Result {
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
