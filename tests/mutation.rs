#![cfg(any(feature = "destroy", feature = "insertion"))]
use std::assert_eq;
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

#[test]
fn test_insert() -> Result {
	let html = r#"
  <div>
      <img src="a.png" />
      <img src="b.jpg" />
      <img src="c.webp" />
  </div>
  "#;
	let fragement = Vis::load(html)?;
	let root_div = fragement.children("div");
	let mut img_list = root_div.find("img[src]");
	img_list.for_each(|_, ele| {
		let attr_src = ele.get_attribute("src").unwrap().to_string();
		if attr_src.ends_with(".png") {
			let mut img = Vis::dom(ele);
			let mut svg = Vis::load("<svg></svg>").unwrap();
			svg.insert_before(&mut img);
			img.remove();
		}
		true
	});
	let now_img_list = root_div.find("img[src]");
	assert_eq!(now_img_list.length(), 2);
	let now_svg = root_div.find("svg");
	assert_eq!(now_svg.length(), 1);
	Ok(())
}

#[test]
fn test_replace_with() -> Result {
	let html = r#"
  <div>
      <img src="a.png" />
      <img src="b.jpg" />
      <img src="c.webp" />
  </div>
  "#;
	let fragement = Vis::load(html)?;
	let root_div = fragement.children("div");
	let mut img_list = root_div.find("img[src]");
	img_list.for_each(|_, ele| {
		let attr_src = ele.get_attribute("src").unwrap().to_string();
		if attr_src.ends_with(".png") {
			let mut img = Vis::dom(ele);
			let mut svg = Vis::load("<svg></svg>").unwrap();
			img.replace_with(&mut svg);
		}
		true
	});
	let now_img_list = root_div.find("img[src]");
	assert_eq!(now_img_list.length(), 2);
	let now_svg = root_div.find("svg");
	assert_eq!(now_svg.length(), 1);
	Ok(())
}

#[test]
fn test_clone() -> Result {
	let menu_html = r#"<menu class="menu">
  <h3>Title</h3>
  <ul class="list">
    <li class="item-1">item1</li>
    <li class="item-2">item2</li>
  </ul>
  </menu>"#;
	let html = format!(
		r#"
  <h2>logo</h2>
  {}
  "#,
		menu_html
	);
	let fragement = Vis::load(html)?;
	let menu = fragement.find(">.menu");
	let clone_menu = menu.clone();
	let mut clone_h3 = clone_menu.find(">h3");
	clone_h3.set_text("h3");
	assert_eq!(menu.outer_html(), menu_html);
	assert_eq!(clone_h3.text(), "h3");
	let mut clone_item_1 = clone_menu.find(".item-1");
	clone_item_1.add_class("item");
	assert_eq!(menu.outer_html(), menu_html);
	assert!(clone_item_1.has_class("item"));
	clone_item_1.remove_class("item-1").add_class("item-3");
	clone_item_1.append_to(&mut menu.find("ul.list"));
	assert_eq!(menu.find(".list > li").length(), 3);
	assert!(menu.find(".list > li").eq(2).has_class("item-3"));
	assert_eq!(clone_menu.find(".list > li").length(), 1);
	assert_eq!(clone_menu.find(".list > li").first().text(), "item2");
	Ok(())
}
