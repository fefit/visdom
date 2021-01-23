use ntree::selector::interface::{IAttrValue, KindError};
use visdom::Vis;

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
fn test_selector_find() -> Result<(), KindError> {
	Vis::init();
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
