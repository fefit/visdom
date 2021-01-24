use ntree::selector::interface::{IAttrValue, KindError};
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
	let is_position_ok = if let Some(v) = divs.eq(1)?.attr("class") {
		match v {
			IAttrValue::Value(v, _) => v == r#"inner-div-1-1"#,
			IAttrValue::True => false,
		}
	} else {
		false
	};
	assert!(is_position_ok);
	let sub_divs = divs.find("div")?;
	assert_eq!(sub_divs.length(), 4);
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
	let is_id_ele = match filter_id.attr("id") {
		Some(IAttrValue::Value(v, _)) => v == "id",
		_ => false,
	};
	assert!(is_id_ele);
	// filter #id
	let filter_id = id_divs.filter_by(|_, ele| Vis::dom(ele).is("#id").unwrap_or(false))?;
	assert_eq!(filter_id.length(), 1);
	let is_id_ele = match filter_id.attr("id") {
		Some(IAttrValue::Value(v, _)) => v == "id",
		_ => false,
	};
	assert!(is_id_ele);
	// filter #nested
	let filter_nested = id_divs.filter_by(|_, ele| {
		Vis::dom(ele)
			.has("[class|='outer']")
			.map(|v| !v.is_empty())
			.unwrap_or(false)
	})?;
	assert_eq!(filter_nested.length(), 1);
	let is_nested_ele = match filter_nested.attr("id") {
		Some(IAttrValue::Value(v, _)) => v == "nested",
		_ => false,
	};
	assert!(is_nested_ele);
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
	let is_id_ele = match filter_id.attr("id") {
		Some(IAttrValue::Value(v, _)) => v == "id",
		_ => false,
	};
	assert!(is_id_ele);
	// filter #nested
	let nested_ele = id_divs.not_in(&id_ele)?.eq(0)?;
	let filter_nested = id_divs.filter_in(&nested_ele)?;
	assert_eq!(filter_nested.length(), 1);
	let is_nested_ele = match filter_nested.attr("id") {
		Some(IAttrValue::Value(v, _)) => v == "nested",
		_ => false,
	};
	assert!(is_nested_ele);
	Ok(())
}
