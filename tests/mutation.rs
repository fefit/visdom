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
