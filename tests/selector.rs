use ntree::interface::{IAttrValue, NodeList};
use std::error::Error;
use std::result::Result as StdResult;
use visdom::Vis;
type Result = StdResult<(), Box<dyn Error>>;
const HTML: &str = r##"
  <html>
    <head></head>
    <body>
      <div id="id">
        <div class="class">class-div</div>
        <p>
          p-tag
        </p>
      </div>
      <div id="nested">
        <div class="outer-div-1">
          <div class="inner-div-1-1">inner-div-1-1</div>
          <div class="inner-div-1-2">inner-div-<span>1</span>-<span>2</span></div>
        </div>
        <div class="outer-div-2">
          <div class="inner-div-2-1"></div>
          <div class="inner-div-2-2"></div>
        </div>
      </div>
    </body>
  </html>
"##;

fn is_attr(node_list: &NodeList, name: &str, value: &str) -> bool {
	match node_list.attr(name) {
		Some(IAttrValue::Value(v, _)) => v == value,
		_ => false,
	}
}
#[test]
fn test_selector_find() -> Result {
	let root = Vis::load(HTML)?;
	let id_ele = root.find("div#id");
	assert_eq!(id_ele.length(), 1);
	let children = id_ele.find("> *");
	assert_eq!(children.length(), 2);
	let p_ele = id_ele.find("p");
	assert_eq!(p_ele.length(), 1);
	let ul_ele = id_ele.find("ul");
	assert_eq!(ul_ele.length(), 0);
	// nested
	let nested = root.find("div#id ~ div#nested");
	assert_eq!(nested.length(), 1);
	let divs = nested.find("div");
	assert_eq!(divs.length(), 6);
	let is_position_ok = is_attr(&divs.eq(1), "class", "inner-div-1-1");
	assert!(is_position_ok);
	let sub_divs = divs.find("div");
	assert_eq!(sub_divs.length(), 4);
	// group
	let outer_and_inner = nested.find("[class|='outer'],[class|='inner']");
	assert_eq!(outer_and_inner.length(), 6);
	// assert!(is_attr(&outer_and_inner.eq(1), "class", "inner-div-1-1"));
	Ok(())
}

#[test]
fn test_selector_filter() -> Result {
	let root = Vis::load(HTML)?;
	let divs = root.find("div");
	let id_ele = divs.filter("#id");
	assert_eq!(id_ele.length(), 1);
	let div_in_id = divs.filter("#id > *");
	assert_eq!(div_in_id.length(), 1);
	let outer_div_in_nested = divs.filter("#nested > [class|='outer']");
	assert_eq!(outer_div_in_nested.length(), 2);
	let inner_div_in_nested = divs.filter("#nested > [class|='outer'] > [class|='inner']");
	assert_eq!(inner_div_in_nested.length(), 4);
	let id_not_ok_ele = divs.filter("div > #id");
	assert_eq!(id_not_ok_ele.length(), 0);
	let id_ok_ele = divs.filter("html body > #id");
	assert_eq!(id_ok_ele.length(), 1);
	Ok(())
}

#[test]
fn test_selector_filter_by() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	assert_eq!(id_divs.length(), 2);
	// filter #id
	let filter_id = id_divs.filter_by(|index, _| index == 0);
	assert_eq!(filter_id.length(), 1);
	assert!(is_attr(&filter_id, "id", "id"));
	// filter #id
	let filter_id = id_divs.filter_by(|_, ele| Vis::dom(ele).is("#id"));
	assert_eq!(filter_id.length(), 1);
	assert!(is_attr(&filter_id, "id", "id"));
	// filter #nested
	let filter_nested =
		id_divs.filter_by(|_, ele| Vis::dom(ele).has("[class|='outer']").length() > 0);
	assert_eq!(filter_nested.length(), 1);
	assert!(is_attr(&filter_nested, "id", "nested"));
	Ok(())
}

#[test]
fn test_selector_filter_in() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	// filter #id
	let filter_id = id_divs.filter_in(&id_ele);
	assert_eq!(filter_id.length(), 1);
	assert!(is_attr(&filter_id, "id", "id"));
	// filter #nested
	let nested_ele = id_divs.not_in(&id_ele).eq(0);
	let filter_nested = id_divs.filter_in(&nested_ele);
	assert_eq!(filter_nested.length(), 1);
	assert!(is_attr(&filter_nested, "id", "nested"));
	Ok(())
}

#[test]
fn test_selector_not() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	// not #id
	let not_id = id_ele.not("#id");
	assert_eq!(not_id.length(), 0);
	// not [id]
	let not_has_id = id_divs.not("[id]");
	assert_eq!(not_has_id.length(), 0);
	Ok(())
}

#[test]
fn test_selector_not_by() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	// not #id
	let not_id = id_ele.not_by(|_, node| {
		node
			.get_attribute("id")
			.map(|v| v.is_str("id"))
			.unwrap_or(false)
	});
	assert_eq!(not_id.length(), 0);
	// not [id]
	let not_has_id = id_divs.not_by(|_, node| node.get_attribute("id").is_some());
	assert_eq!(not_has_id.length(), 0);
	Ok(())
}

#[test]
fn test_selector_not_in() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	// not #id
	let not_id = id_ele.not_in(&id_divs);
	assert_eq!(not_id.length(), 0);
	// not #id
	let not_id = id_divs.not_in(&id_ele).filter("#id");
	assert_eq!(not_id.length(), 0);
	Ok(())
}

#[test]
fn test_selector_is() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	// is #id
	let is_id = id_ele.is("body #id");
	assert!(is_id);
	// is #id
	let is_id = id_divs.is("body > #id");
	assert!(is_id);
	// is #id
	let is_id = id_divs.is("div[id='id']");
	assert!(is_id);
	Ok(())
}

#[test]
fn test_selector_is_by() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	// is #id
	let is_id = id_divs.is_by(|_, node| {
		node
			.get_attribute("id")
			.map(|v| v.is_str("id"))
			.unwrap_or(false)
	});
	assert!(is_id);
	// is #id
	let is_id = id_ele.is_by(|_, node| node.get_attribute("id").is_some());
	assert!(is_id);
	// not [id]
	let not_has_id = !root
		.find("div:not([id])")
		.is_by(|_, node| node.get_attribute("id").is_some());
	assert!(not_has_id);
	Ok(())
}
#[test]
fn test_selector_is_in() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	// is #id
	let is_id = id_ele.is_in(&id_divs);
	assert!(is_id);
	// is #id
	let is_id = id_divs.is_in(&id_ele);
	assert!(is_id);
	// is #id
	let is_not_id = !id_divs.is_in(&root.find("div").not("[id]"));
	assert!(is_not_id);
	Ok(())
}

#[test]
fn test_selector_is_all() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	// is #id
	let is_all_id = id_ele.is_all("body #id");
	assert!(is_all_id);
	// is #id
	let is_not_all_id = !id_divs.is_all("body > #id");
	assert!(is_not_all_id);
	// is #id
	let is_not_all_id = !id_divs.is_all("div[id='id']");
	assert!(is_not_all_id);
	Ok(())
}

#[test]
fn test_selector_is_all_by() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	// is #id
	let is_id = id_ele.is_all_by(|index, _| index == 0);
	assert_eq!(is_id, true);
	// is #id
	let is_id = id_divs.is_all_by(|_, node| {
		node
			.get_attribute("id")
			.map(|v| v.is_str("id"))
			.unwrap_or(false)
	});
	assert_ne!(is_id, true);
	// is #id
	let is_id = id_divs.is_all_by(|_, node| node.tag_name() == "div");
	assert!(is_id);
	Ok(())
}

#[test]
fn test_selector_is_all_in() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	// is #id
	let is_id = id_ele.is_all_in(&id_divs);
	assert_eq!(is_id, true);
	// is #id
	let is_id = id_divs.is_all_in(&id_ele);
	assert_ne!(is_id, true);
	// is #id
	let is_id = id_divs.is_all_in(&root.find("div"));
	assert!(is_id);
	Ok(())
}

#[test]
fn test_selector_has() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	// #id
	let has_class_div = id_ele.has("div.class");
	assert_eq!(has_class_div.length(), 1);
	// #id
	let not_id_eles = id_divs.has("[class|='outer']");
	assert_eq!(not_id_eles.length() > 0, true);
	assert_eq!(not_id_eles.has("div.class").length(), 0);
	// is #id
	let be_id_ele = id_divs.has("div+p");
	assert!(be_id_ele.is_all_in(&id_ele));
	Ok(())
}

#[test]
fn test_content_text() -> Result {
	let root = Vis::load(HTML)?;
	// inner div 1-1
	let inner_div_1_1 = root.find("div.inner-div-1-1");
	let inner_div_1_1_text = inner_div_1_1.text();
	assert_eq!(inner_div_1_1_text, "inner-div-1-1");
	// inner div 1-2
	let inner_div_1_2 = root.find("div.inner-div-1-2");
	let inner_div_1_2_text = inner_div_1_2.text();
	assert!(inner_div_1_2.children("").length() > 0);
	assert_eq!(inner_div_1_2_text, "inner-div-1-2");
	// return
	Ok(())
}
