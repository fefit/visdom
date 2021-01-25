use ntree::selector::interface::{IAttrValue, KindError, NodeList};
use std::result::Result as StdResult;
use visdom::Vis;
type Result = StdResult<(), KindError>;
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
          <div class="inner-div-1-1"></div>
          <div class="inner-div-1-2"></div>
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
	let doc = Vis::load(HTML)?;
	let id_ele = doc.find("div#id")?;
	assert_eq!(id_ele.length(), 1);
	let children = id_ele.find("> *")?;
	assert_eq!(children.length(), 2);
	let p_ele = id_ele.find("p")?;
	assert_eq!(p_ele.length(), 1);
	let ul_ele = id_ele.find("ul")?;
	assert_eq!(ul_ele.length(), 0);
	// nested
	let nested = doc.find("div#id ~ div#nested")?;
	assert_eq!(nested.length(), 1);
	let divs = nested.find("div")?;
	assert_eq!(divs.length(), 6);
	let is_position_ok = is_attr(&divs.eq(1)?, "class", "inner-div-1-1");
	assert!(is_position_ok);
	let sub_divs = divs.find("div")?;
	assert_eq!(sub_divs.length(), 4);
	// group
	let outer_and_inner = nested.find("[class|='outer'],[class|='inner']")?;
	assert_eq!(outer_and_inner.length(), 6);
	// assert!(is_attr(&outer_and_inner.eq(1)?, "class", "inner-div-1-1"));
	Ok(())
}

#[test]
fn test_selector_filter() -> Result {
	let doc = Vis::load(HTML)?;
	let divs = doc.find("div")?;
	let id_ele = divs.filter("#id")?;
	assert_eq!(id_ele.length(), 1);
	let div_in_id = divs.filter("#id > *")?;
	assert_eq!(div_in_id.length(), 1);
	let outer_div_in_nested = divs.filter("#nested > [class|='outer']")?;
	assert_eq!(outer_div_in_nested.length(), 2);
	let inner_div_in_nested = divs.filter("#nested > [class|='outer'] > [class|='inner']")?;
	assert_eq!(inner_div_in_nested.length(), 4);
	let id_not_ok_ele = divs.filter("div > #id")?;
	assert_eq!(id_not_ok_ele.length(), 0);
	let id_ok_ele = divs.filter("html body > #id")?;
	assert_eq!(id_ok_ele.length(), 1);
	Ok(())
}

#[test]
fn test_selector_filter_by() -> Result {
	let doc = Vis::load(HTML)?;
	let id_divs = doc.find("div[id]")?;
	assert_eq!(id_divs.length(), 2);
	// filter #id
	let filter_id = id_divs.filter_by(|index, _| index == 0)?;
	assert_eq!(filter_id.length(), 1);
	assert!(is_attr(&filter_id, "id", "id"));
	// filter #id
	let filter_id = id_divs.filter_by(|_, ele| Vis::dom(ele).is("#id").unwrap_or(false))?;
	assert_eq!(filter_id.length(), 1);
	assert!(is_attr(&filter_id, "id", "id"));
	// filter #nested
	let filter_nested = id_divs.filter_by(|_, ele| {
		Vis::dom(ele)
			.has("[class|='outer']")
			.map(|v| !v.is_empty())
			.unwrap_or(false)
	})?;
	assert_eq!(filter_nested.length(), 1);
	assert!(is_attr(&filter_nested, "id", "nested"));
	Ok(())
}

#[test]
fn test_selector_filter_in() -> Result {
	let doc = Vis::load(HTML)?;
	let id_divs = doc.find("div[id]")?;
	let id_ele = id_divs.filter("#id")?;
	// filter #id
	let filter_id = id_divs.filter_in(&id_ele)?;
	assert_eq!(filter_id.length(), 1);
	assert!(is_attr(&filter_id, "id", "id"));
	// filter #nested
	let nested_ele = id_divs.not_in(&id_ele)?.eq(0)?;
	let filter_nested = id_divs.filter_in(&nested_ele)?;
	assert_eq!(filter_nested.length(), 1);
	assert!(is_attr(&filter_nested, "id", "nested"));
	Ok(())
}

#[test]
fn test_selector_not() -> Result {
	let doc = Vis::load(HTML)?;
	let id_divs = doc.find("div[id]")?;
	let id_ele = id_divs.filter("#id")?;
	// not #id
	let not_id = id_ele.not("#id")?;
	assert_eq!(not_id.length(), 0);
	// not [id]
	let not_has_id = id_divs.not("[id]")?;
	assert_eq!(not_has_id.length(), 0);
	Ok(())
}
