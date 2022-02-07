use std::result::Result as StdResult;
use visdom::types::{BoxDynError, Combinator, Elements, IAttrValue};
use visdom::Vis;
type Result = StdResult<(), BoxDynError>;

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

fn is_attr(node_list: &Elements, name: &str, value: &str) -> bool {
	match node_list.attr(name) {
		Some(IAttrValue::Value(v, _)) => v == value,
		_ => false,
	}
}
#[test]
fn test_method_find() -> Result {
	let root = Vis::load(HTML)?;
	let id_ele = root.find("div#id");
	assert_eq!(id_ele.length(), 1);
	let children = id_ele.find("> *");
	assert_eq!(children.length(), 2);
	let p_ele = id_ele.find("p");
	assert_eq!(p_ele.length(), 1);
	let ul_ele = id_ele.find("ul");
	assert_eq!(ul_ele.length(), 0);
	// nested should in
	assert_eq!(root.find("div~#nested").length(), 1);
	assert_eq!(root.find("div+#nested").length(), 1);
	assert_eq!(root.find("body>#nested").length(), 1);
	// should in
	let inner_div_1 = root.find(".outer-div-1");
	let inner_div_2_2 = inner_div_1.find("~div > .inner-div-2-2");
	assert_eq!(inner_div_2_2.length(), 1);
	let inner_div_2_2 = inner_div_1.find("+div > .inner-div-2-2");
	assert_eq!(inner_div_2_2.length(), 1);
	let outer_div_2 = root.find("#nested").find("div + .inner-div-2-2");
	assert_eq!(outer_div_2.length(), 1);
	let outer_div_2 = root.find("#nested").find("div ~ .inner-div-2-2");
	assert_eq!(outer_div_2.length(), 1);
	// unique selector
	let div = root.find("div");
	let inner_div_2_2 = div.find(".inner-div-2-2");
	assert_eq!(inner_div_2_2.length(), 1);
	let firsts = div.find(":nth-child(1)");
	assert_eq!(firsts.length(), 5);
	assert!(firsts.eq(3).is("span"));
	let after_firsts = div.find(":nth-child(n + 2)");
	assert_eq!(after_firsts.length(), 5);
	assert!(after_firsts.eq(0).is("p"));
	// complex selector
	let inner_div_2_2 = id_ele.find("~div .outer-div-1 + div > div.inner-div-2-2");
	assert_eq!(inner_div_2_2.length(), 1);
	assert!(inner_div_2_2.has_class("inner-div-2-2"));
	// complex selector
	let inner_div_2_2 = id_ele.find("+#nested .outer-div-1 ~ .outer-div-2 > div.inner-div-2-2");
	assert_eq!(inner_div_2_2.length(), 1);
	assert!(inner_div_2_2.has_class("inner-div-2-2"));
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
	assert!(is_attr(&outer_and_inner.eq(1), "class", "inner-div-1-1"));
	// find
	let inner_div = root.find("div .inner-div-2-2");
	assert_eq!(inner_div.length(), 1);
	// find
	let inner_div = root.find("div+.inner-div-2-2");
	assert_eq!(inner_div.length(), 1);
	// find
	let inner_div = root.find("div~.inner-div-2-2");
	assert_eq!(inner_div.length(), 1);
	Ok(())
}

#[test]
fn test_method_filter() -> Result {
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
fn test_method_filter_by() -> Result {
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
fn test_method_filter_in() -> Result {
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
fn test_method_not() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	// div is not p
	assert_eq!(id_divs.not("p").length(), id_divs.length());
	// not #id
	let not_id = id_ele.not("#id");
	assert_eq!(not_id.length(), 0);
	// not [id]
	let not_has_id = id_divs.not("[id]");
	assert_eq!(not_has_id.length(), 0);
	Ok(())
}

#[test]
fn test_method_not_by() -> Result {
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
fn test_method_not_in() -> Result {
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
fn test_method_is() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	let nested = id_divs.filter("#nested");
	assert!(nested.is("#id~#nested"));
	assert!(nested.is("div+#nested"));
	assert!(nested.is("body > #nested"));
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
fn test_method_is_by() -> Result {
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
fn test_method_is_in() -> Result {
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
fn test_method_is_all() -> Result {
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
fn test_method_is_all_by() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	// is #id
	let is_id = id_ele.is_all_by(|index, _| index == 0);
	assert!(is_id);
	// is #id
	let is_id = id_divs.is_all_by(|_, node| {
		node
			.get_attribute("id")
			.map(|v| v.is_str("id"))
			.unwrap_or(false)
	});
	assert!(!is_id);
	// is #id
	let is_id = id_divs.is_all_by(|_, node| node.tag_name() == "DIV");
	assert!(is_id);
	Ok(())
}

#[test]
fn test_method_is_all_in() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	// all is div[id]
	let is_all_has_id = id_ele.is_all_in(&id_divs);
	assert!(is_all_has_id);
	// all is #id
	let is_all_id = id_divs.is_all_in(&id_ele);
	assert!(!is_all_id);
	// all is div
	let is_all_div = id_divs.is_all_in(&root.find("div"));
	assert!(is_all_div);
	// not contains all id
	assert!(!id_divs.is_all_in(&root.find("#nested, #nested div")));
	Ok(())
}

#[test]
fn test_method_has() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	// #id
	let has_class_div = id_ele.has("div.class");
	assert_eq!(has_class_div.length(), 1);
	// #id
	let nested = id_divs.has("[class|='outer']");
	assert_eq!(nested.length(), 1);
	assert_eq!(nested.has("div.class").length(), 0);
	let nested = id_divs.has("[class|='inner']");
	assert_eq!(nested.length(), 1);
	// is #id
	let be_id_ele = id_divs.has("div+p");
	assert!(be_id_ele.is_all_in(&id_ele));
	Ok(())
}

#[test]
fn test_method_has_in() -> Result {
	let root = Vis::load(HTML)?;
	let id_divs = root.find("div[id]");
	let id_ele = id_divs.filter("#id");
	assert_eq!(id_divs.length(), 2);
	// #id
	let has_class_div = id_ele.has_in(&root.find("div.class"));
	assert_eq!(has_class_div.length(), 1);
	// #id
	let nested = id_divs.has_in(&root.find("[class|='outer']"));
	assert_eq!(nested.length(), 1);
	assert_eq!(nested.has_in(&root.find("div.class")).length(), 0);
	// iterator
	let nested = id_divs.has_in(&root.find("[class|='inner']"));
	assert_eq!(nested.length(), 1);
	// is #id
	let be_id_ele = id_divs.has_in(&root.find("div+p"));
	assert!(be_id_ele.is_all_in(&id_ele));
	Ok(())
}

#[test]
fn test_method_children() -> Result {
	let root = Vis::load(HTML)?;
	let id_ele = root.find("#id");
	// childs
	let childs = id_ele.children("");
	assert_eq!(childs.length(), 2);
	// child div
	let child_divs = id_ele.children("div");
	assert_eq!(child_divs.length(), 1);
	// child p
	let child_p = id_ele.children("~p");
	assert_eq!(child_p.length(), 1);
	// child p
	let child_p = id_ele.children("+p");
	assert_eq!(child_p.length(), 1);
	// child div
	let child_div = id_ele.children("~div");
	assert_eq!(child_div.length(), 0);
	// nested
	let nested = root.find("#nested");
	let nested_childs = nested.children("");
	assert_eq!(nested_childs.length(), 2);
	let nested_sub_childs = nested.children("div > div");
	assert_eq!(nested_sub_childs.length(), 4);
	Ok(())
}

#[test]
fn test_method_parent() -> Result {
	let root = Vis::load(HTML)?;
	let id_ele = root.find("#id");
	// childs
	let childs = id_ele.children("");
	// parent
	let parent = childs.parent("");
	assert_eq!(parent.length(), 1);
	// parent filter
	let parent = childs.parent("#notId");
	assert_eq!(parent.length(), 0);
	let still_div = childs.parent("#id > div");
	assert_eq!(still_div.length(), 1);
	Ok(())
}

#[test]
fn test_method_parents_until() -> Result {
	let html = r##"
  <ul id="one" class="level-1">
    <li class="item-i">I</li>
    <li id="ii" class="item-ii">II
      <ul class="level-2">
        <li class="item-a">A</li>
        <li class="item-b">B
          <ul class="level-3">
            <li class="item-1">1</li>
            <li class="item-2">2</li>
            <li class="item-3">3</li>
          </ul>
        </li>
        <li class="item-c">C</li>
      </ul>
    </li>
    <li class="item-iii">III</li>
  </ul>
  "##;
	let root = Vis::load(html)?;
	let item_1 = root.find(".item-1");
	// parents until
	let to_level_3 = item_1.parents_until(".level-3", "", false);
	assert_eq!(to_level_3.length(), 0);
	// parents until
	let to_level_3_contains = item_1.parents_until(".level-3", "", true);
	assert_eq!(to_level_3_contains.length(), 1);
	// parents until to level 1
	let to_level_1 = item_1.parents_until(".level-1", "", false);
	assert_eq!(to_level_1.length(), 4);
	assert!(to_level_1.eq(0).has_class("item-ii"));
	// parents  until to level 1, but only "li" tags
	let to_level_1_items = item_1.parents_until(".level-1", "li", false);
	assert_eq!(to_level_1_items.length(), 2);
	Ok(())
}

#[test]
fn test_method_parents() -> Result {
	let root = Vis::load(HTML)?;
	let id_ele = root.find("#id");
	// childs
	let childs = id_ele.children("");
	// parent
	let body = childs.parents("body");
	assert_eq!(body.length(), 1);
	Ok(())
}

#[test]
fn test_method_prev() -> Result {
	let html = r##"
  <dl>
    <dt id="term-1">term 1</dt>
      <dd>definition 1-a</dd>
      <dd>definition 1-b</dd>
      <dd>definition 1-c</dd>
      <dd>definition 1-d</dd>
    <dt id="term-2">term 2</dt>
      <dd>definition 2-a</dd>
      <dd>definition 2-b</dd>
      <dd>definition 2-c</dd>
    <dt id="term-3">term 3</dt>
      <dd>definition 3-a</dd>
      <dd>definition 3-b</dd>
  </dl>
  "##;
	let root = Vis::load(html)?;
	let term_with_id = root.find("[id^='term']");
	assert_eq!(term_with_id.length(), 3);
	// prev
	let term_with_id_prev = term_with_id.prev("");
	assert_eq!(term_with_id_prev.length(), 2);
	// prev dt
	let term_with_id_prev = term_with_id.prev("dt");
	assert_eq!(term_with_id_prev.length(), 0);
	Ok(())
}

#[test]
fn test_method_next() -> Result {
	let html = r##"
  <dl>
    <dt id="term-1">term 1</dt>
      <dd>definition 1-a</dd>
      <dd>definition 1-b</dd>
      <dd>definition 1-c</dd>
      <dd>definition 1-d</dd>
    <dt id="term-2">term 2</dt>
      <dd>definition 2-a</dd>
      <dd>definition 2-b</dd>
      <dd>definition 2-c</dd>
    <dt id="term-3">term 3</dt>
      <dd>definition 3-a</dd>
      <dd>definition 3-b</dd>
  </dl>
  "##;
	let root = Vis::load(html)?;
	let term_with_id = root.find("[id^='term']");
	assert_eq!(term_with_id.length(), 3);
	// next
	let term_with_id_next = term_with_id.next("");
	assert_eq!(term_with_id_next.length(), 3);
	// next dd
	let term_with_id_next = term_with_id.next("dd");
	assert_eq!(term_with_id_next.length(), 3);
	// next dt
	let term_with_id_next = term_with_id.next("dt");
	assert_eq!(term_with_id_next.length(), 0);
	Ok(())
}
#[test]
fn test_method_next_all() -> Result {
	let html = r##"
  <dl>
    <dt id="term-1">term 1</dt>
      <dd>definition 1-a</dd>
      <dd>definition 1-b</dd>
      <dd>definition 1-c</dd>
      <dd>definition 1-d</dd>
    <dt id="term-2">term 2</dt>
      <dd>definition 2-a</dd>
      <dd>definition 2-b</dd>
      <dd>definition 2-c</dd>
    <dt id="term-3">term 3</dt>
      <dd>definition 3-a</dd>
      <dd>definition 3-b</dd>
  </dl>
  "##;
	let root = Vis::load(html)?;
	let id_term_2 = root.find("#term-2");
	// next all
	let item_after_term_2 = id_term_2.next_all("");
	assert_eq!(item_after_term_2.length(), 6);
	// next all dd
	let dd_after_term_2 = id_term_2.next_all("dd");
	assert_eq!(dd_after_term_2.length(), 5);
	// next all dt
	let dt_after_term_2 = id_term_2.next_all("dt");
	assert_eq!(dt_after_term_2.length(), 1);
	Ok(())
}

#[test]
fn test_method_prev_all() -> Result {
	let html = r##"
  <dl>
    <dt id="term-1">term 1</dt>
      <dd>definition 1-a</dd>
      <dd>definition 1-b</dd>
      <dd>definition 1-c</dd>
      <dd>definition 1-d</dd>
    <dt id="term-2">term 2</dt>
      <dd>definition 2-a</dd>
      <dd>definition 2-b</dd>
      <dd>definition 2-c</dd>
    <dt id="term-3">term 3</dt>
      <dd>definition 3-a</dd>
      <dd>definition 3-b</dd>
  </dl>
  "##;
	let root = Vis::load(html)?;
	let id_term_2 = root.find("#term-2");
	// prev all
	let item_before_term_2 = id_term_2.prev_all("");
	assert_eq!(item_before_term_2.length(), 5);
	// prev all dd
	let dd_before_term_2 = id_term_2.prev_all("dd");
	assert_eq!(dd_before_term_2.length(), 4);
	// prev all dt
	let dt_before_term_2 = id_term_2.prev_all("dt");
	assert_eq!(dt_before_term_2.length(), 1);
	Ok(())
}

#[test]
fn test_method_prev_until() -> Result {
	let html = r##"
  <dl>
    <dt id="term-1">term 1</dt>
      <dd>definition 1-a</dd>
      <dd>definition 1-b</dd>
      <dd>definition 1-c</dd>
      <dd>definition 1-d</dd>
    <dt id="term-2">term 2</dt>
      <dd>definition 2-a</dd>
      <dd>definition 2-b</dd>
      <dd>definition 2-c</dd>
    <dt id="term-3">term 3</dt>
      <dd>definition 3-a</dd>
      <dd>definition 3-b</dd>
  </dl>
  "##;
	let root = Vis::load(html)?;
	let id_term_2 = root.find("#term-2");
	// until meet the dt
	let dd_before_term_2 = id_term_2.prev_until("dt", "", false);
	assert_eq!(dd_before_term_2.length(), 4);
	assert_eq!(dd_before_term_2.eq(0).text(), "definition 1-a");
	// until meet the dt, but contains dt
	let dd_and_self_before_term_2 = id_term_2.prev_until("dt", "", true);
	assert_eq!(dd_and_self_before_term_2.length(), 5);
	// until with filter
	let id_term_3 = root.find("#term-3");
	let filter_before_term_3 = id_term_3.prev_until("#term-1", ":contains('1')", true);
	assert_eq!(filter_before_term_3.length(), 5);
	Ok(())
}
#[test]
fn test_method_next_until() -> Result {
	let html = r##"
  <dl>
    <dt id="term-1">term 1</dt>
      <dd>definition 1-a</dd>
      <dd>definition 1-b</dd>
      <dd>definition 1-c</dd>
      <dd>definition 1-d</dd>
    <dt id="term-2">term 2</dt>
      <dd>definition 2-a</dd>
      <dd>definition 2-b</dd>
      <dd>definition 2-c</dd>
    <dt id="term-3">term 3</dt>
      <dd>definition 3-a</dd>
      <dd>definition 3-b</dd>
  </dl>
  "##;
	let root = Vis::load(html)?;
	let id_term_2 = root.find("#term-2");
	// until wrong selector
	let wrong_dd_after_term_2 = id_term_2.next_until(":dt", "", false);
	assert_eq!(wrong_dd_after_term_2.length(), 0);
	// until meet the dt
	let dd_after_term_2 = id_term_2.next_until("dt", "", false);
	assert_eq!(dd_after_term_2.length(), 3);
	// until meet the dt, but contains dt
	let dd_and_self_after_term_2 = id_term_2.next_until("dt", "", true);
	assert_eq!(dd_and_self_after_term_2.length(), 4);
	// until with filter
	let id_term_1 = root.find("#term-1");
	let filter_after_term_1 = id_term_1.next_until("#term-3", ":contains('2')", false);
	assert_eq!(filter_after_term_1.length(), 4);
	// until wrong filter
	let wrong_filter_after_term_1 = id_term_1.next_until("#term-3", ":gt('2')", false);
	assert_eq!(wrong_filter_after_term_1.length(), 0);
	Ok(())
}

#[test]
fn test_method_find_closest() -> Result {
	let html = r##"
  <ul id="one" class="level-1">
    <li class="item-i">I</li>
    <li id="ii" class="item-ii">II
      <ul class="level-2">
        <li class="item-a">A</li>
        <li class="item-b">B
          <ul class="level-3">
            <li class="item-1">1</li>
            <li class="item-2">2</li>
            <li class="item-3">3</li>
          </ul>
        </li>
        <li class="item-c">C</li>
      </ul>
    </li>
    <li class="item-iii">III</li>
  </ul>
  "##;
	let root = Vis::load(html)?;
	let closest_ul = root.find("li.item-a").closest("ul");
	assert!(is_attr(&closest_ul, "class", "level-2"));
	let closest_self = root.find("li.item-a").closest("li");
	assert!(is_attr(&closest_self, "class", "item-a"));
	Ok(())
}

#[test]
fn test_method_closest() -> Result {
	let root = Vis::load(
		r#"
	    <div class="closest">
	      <p>
	        <a class="closest">aaa</a>
          <b class="closest">bbb</b>
          <c>ccc</c>
	      </p>
	      <a>top-aaaa</a>
	    </div>
	"#,
	)?;
	let abc = root.find("a,b,c");
	assert_eq!(abc.length(), 4);
	// wrong selector :first
	assert_eq!(abc.closest(":first").length(), 0);
	// empty selector, always return empty elements
	assert_eq!(abc.closest("").length(), 0);
	let closest = abc.closest(".closest");
	assert_eq!(closest.length(), 3);
	assert_eq!(closest.eq(0).get(0).unwrap().tag_name(), "DIV");
	Ok(())
}

#[test]
fn test_method_siblings() -> Result {
	// siblings
	let root = Vis::load(HTML)?;
	let divs = root.find("div");
	assert_eq!(divs.length(), 9);
	let siblings = divs.siblings("div");
	assert_eq!(siblings.length(), 8);
	// more cases
	let root = Vis::load(
		r#"
	    <div class="closest">
	      <p><a class="closest">aaa</a><b class="closest">bbb</b><c>ccc</c></p>
	      <a>top-aaaa</a>
	    </div>
	"#,
	)?;
	let abc = root.find("a,b,c");
	// siblings
	let ele_c = abc.filter("c");
	assert_eq!(ele_c.siblings("").length(), 2);
	// siblings
	let ele_a = abc.filter("a");
	assert_eq!(ele_a.siblings("").length(), 3);
	// empty selector
	let siblings = abc.siblings("");
	assert_eq!(siblings.length(), 4);
	assert_eq!(siblings.eq(0).get(0).unwrap().tag_name(), "P");
	// wrong siblings selecotr
	assert_eq!(abc.siblings(":nono").length(), 0);
	// siblings
	let siblings = abc.siblings(".closest");
	assert_eq!(siblings.length(), 2);
	assert_eq!(siblings.eq(0).get(0).unwrap().tag_name(), "A");
	// big count childs
	let html = format!("<ul>{}</ul>", "<li></li>".repeat(3000));
	let root = Vis::load(&html)?;
	let ul = root.find("ul");
	let nth_2n_child = ul.find(":nth-child(2n)");
	assert_eq!(nth_2n_child.siblings("").length(), 3000);
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

#[test]
fn test_method_eq() -> Result {
	let html = r##"
  <dl>
    <dt id="term-1">term 1</dt>
      <dd>definition 1-a</dd>
      <dd>definition 1-b</dd>
      <dd>definition 1-c</dd>
      <dd>definition 1-d</dd>
    <dt id="term-2">term 2</dt>
      <dd>definition 2-a</dd>
      <dd>definition 2-b</dd>
      <dd>definition 2-c</dd>
    <dt id="term-3">term 3</dt>
      <dd>definition 3-a</dd>
      <dd>definition 3-b</dd>
  </dl>
  "##;
	let root = Vis::load(html)?;
	let term_with_id = root.find("[id^='term']");
	assert_eq!(term_with_id.length(), 3);
	// eq
	let term_id_1 = term_with_id.eq(0);
	assert_eq!(term_id_1.length(), 1);
	assert!(term_id_1.is("#term-1"));
	assert!(term_id_1.is_in(&term_with_id.first()));
	// more
	assert!(term_with_id.eq(2).is("#term-3"));
	assert!(term_with_id.eq(2).is_in(&term_with_id.last()));
	assert!(term_with_id.eq(3).is_empty());
	Ok(())
}

#[test]
fn test_method_slice() -> Result {
	let html = r##"
  <dl>
    <dt id="term-1">term 1</dt>
      <dd>definition 1-a</dd>
      <dd>definition 1-b</dd>
      <dd>definition 1-c</dd>
      <dd>definition 1-d</dd>
    <dt id="term-2">term 2</dt>
      <dd>definition 2-a</dd>
      <dd>definition 2-b</dd>
      <dd>definition 2-c</dd>
    <dt id="term-3">term 3</dt>
      <dd>definition 3-a</dd>
      <dd>definition 3-b</dd>
  </dl>
  "##;
	let root = Vis::load(html)?;
	let term_with_id = root.find("[id^='term']");
	assert_eq!(term_with_id.length(), 3);
	// slice
	let term_id_slice = term_with_id.slice(1..);
	assert_eq!(term_id_slice.length(), 2);
	// slice
	let term_id_slice = term_with_id.slice(1..5);
	assert_eq!(term_id_slice.length(), 2);
	// slice
	let term_id_slice = term_with_id.slice(..3);
	assert_eq!(term_id_slice.length(), 3);
	// slice
	let term_id_slice = term_with_id.slice(..5);
	assert_eq!(term_id_slice.length(), 3);
	// slice
	let term_id_slice = term_with_id.slice(3..);
	assert_eq!(term_id_slice.length(), 0);
	Ok(())
}

#[test]
fn test_method_add() -> Result {
	let html = r##"
  <dl>
    <dt id="term-1">term 1</dt>
      <dd>definition 1-a</dd>
      <dd>definition 1-b</dd>
      <dd>definition 1-c</dd>
      <dd>definition 1-d</dd>
    <dt id="term-2">term 2</dt>
      <dd>definition 2-a</dd>
      <dd>definition 2-b</dd>
      <dd>definition 2-c</dd>
    <dt id="term-3">term 3</dt>
      <dd>definition 3-a</dd>
      <dd>definition 3-b</dd>
  </dl>
  "##;
	let root = Vis::load(html)?;
	let dl = root.find("dl");
	let dt = dl.children("dt");
	let dd = dl.children("dd");
	let dl_childs = dt.add(dd);
	assert_eq!(dl.children("").length(), dl_childs.length());
	assert!(dl_childs.eq(0).is("dt") && dl_childs.eq(0).attr("id").unwrap().is_str("term-1"));
	assert!(dl_childs.eq(1).is("dd") && dl_childs.eq(1).text().contains("1-a"));
	assert!(dl_childs.last().is("dd") && dl_childs.last().text().contains("3-b"));
	// clone self
	let another_dl_childs = dl_childs.add(Elements::new());
	assert_eq!(another_dl_childs.length(), dl_childs.length());
	Ok(())
}

#[test]
fn test_method_for_root() -> Result {
	let html = r##"
  <dl>
    <dt id="term-1">term 1</dt>
      <dd>definition 1-a</dd>
      <dd>definition 1-b</dd>
      <dd>definition 1-c</dd>
      <dd>definition 1-d</dd>
    <dt id="term-2">term 2</dt>
      <dd>definition 2-a</dd>
      <dd>definition 2-b</dd>
      <dd>definition 2-c</dd>
    <dt id="term-3">term 3</dt>
      <dd>definition 3-a</dd>
      <dd>definition 3-b</dd>
  </dl>
  "##;
	let root = Vis::load(html)?;
	assert_eq!(root.prev_all("").length(), 0);
	assert_eq!(root.next_all("").length(), 0);
	assert_eq!(root.get(0).unwrap().siblings().length(), 0);
	assert_eq!(root.parent("").length(), 0);
	Ok(())
}

#[test]
fn test_method_contains() -> Result {
	let html = r##"
    <dl>
      <dt id="term-1">term 1</dt>
        <dd>definition 1-a</dd>
        <dd>definition 1-b</dd>
        <dd>definition 1-c</dd>
        <dd>definition 1-d</dd>
      <dt id="term-2">term 2</dt>
        <dd>definition 2-a</dd>
        <dd>definition 2-b</dd>
        <dd>definition 2-c</dd>
      <dt id="term-3">term 3</dt>
        <dd>definition 3-a</dd>
        <dd>definition 3-b</dd>
    </dl>
  "##;
	let root = Vis::load(html)?;
	let dl = root.find("dl");
	let childs = dl.children("");
	assert!(dl.contains(childs.get(0).unwrap(), &Combinator::Children));
	assert!(dl.contains(childs.get(0).unwrap(), &Combinator::ChildrenAll));
	assert!(childs
		.eq(0)
		.contains(childs.get(1).unwrap(), &Combinator::Next));
	assert!(childs
		.eq(0)
		.contains(childs.get(0).unwrap(), &Combinator::Chain));
	assert!(childs
		.eq(0)
		.contains(childs.get(2).unwrap(), &Combinator::NextAll));
	assert!(!childs
		.eq(0)
		.contains(childs.get(2).unwrap(), &Combinator::Next));
	Ok(())
}
