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
	// special cases
	let html = r##"
  <nav id="lang">
    <a href="#" lang>boolean lang</a>
    <a href="#" lang="">empty lang</a>
  </nav>
  "##;
	let root = Vis::load(&html)?;
	let lang = root.find("#lang");
	assert_eq!(lang.find("a[lang^='']").length(), 0);
	assert_eq!(lang.find("a[lang$='']").length(), 0);
	assert_eq!(lang.find("a[lang*='']").length(), 0);
	assert_eq!(lang.find("a[lang~='']").length(), 0);
	assert_eq!(lang.find("a[lang^='a']").length(), 0);
	assert_eq!(lang.find("a[lang$='b']").length(), 0);
	assert_eq!(lang.find("a[lang*='c']").length(), 0);
	assert_eq!(lang.find("a[lang~='d']").length(), 0);
	assert_eq!(lang.find("a[lang!='']").length(), 0);
	assert_eq!(lang.find("a[lang!='anything']").length(), 2);
	assert_eq!(lang.find("a[lang='']").length(), 2);
	assert_eq!(lang.find("a[lang]").length(), 2);
	assert_eq!(lang.find("a[lang|='']").length(), 2);
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
	assert_eq!(link.filter("#lang #link").length(), 1);
	// nested
	let link = root.find("#lang #link");
	assert_eq!(link.length(), 1);
	// limit parent
	let link = root.find("nav #link");
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
    <em>no class selector</em>
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
fn test_selector_pseudo_empty() -> Result {
	let html = r#"<h1>abc</h1><div></div><p><!--comment--></p><b> </b>"#;
	let root = Vis::load(html)?;
	// empty
	let empty = root.find(":empty");
	assert_eq!(empty.length(), 2);
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
	assert_eq!(content.find("p:contains(\"Vis dom\")").length(), 0);
	// no quote
	assert_eq!(content.find("p:contains(Visdom)").length(), 2);
	// contains empty
	assert_eq!(
		content.find("p:contains()").length(),
		content.find("p").length()
	);
	Ok(())
}

#[test]
fn test_selector_pseudo_only_child() -> Result {
	let html = r#"
    <!doctype html>
    <html lang="en">
      <head>
        <meta charset="utf-8">
        <title>:only-child</title>
      </head>
    <body>
      <ul class="list1">
        <li>list1-item1</li>
      </ul>
      <ul class="list2">
        <li>list2-item1</li>
        <li>list2-item2</li>
      </ul>
      <ul class="list3">
        lists-text!
        <li>list3-item1</li>
      </ul>
    </body>
    </html>
  "#;
	let root = Vis::load(&html)?;
	// :only-child
	let only_child = root.find("li:only-child");
	assert_eq!(only_child.length(), 2);
	assert_eq!(only_child.eq(0).parent("").is(".list1"), true);
	assert_eq!(only_child.eq(1).parent("").is(".list3"), true);
	Ok(())
}

#[test]
fn test_selector_pseudo_first_child() -> Result {
	let html = r#"
    <!doctype html>
    <html lang="en">
      <head>
        <meta charset="utf-8">
        <title>:first-child</title>
      </head>
    <body>
      <ul>
        <li>item1</li>
        <li>item2</li>
        <li>item3</li>
        <li>item4</li>
        <li>item5</li>
        <li>item6</li>
        <li>item7</li>
        <li>item8</li>
        <li>item9</li>
      </ul>
    </body>
    </html>
  "#;
	let root = Vis::load(&html)?;
	// :first-child
	let first_child = root.find("li:first-child");
	assert_eq!(first_child.length(), 1);
	assert_eq!(first_child.text(), "item1");
	// prev :first-child
	let prev_first_child = first_child.prev_all(":first-child");
	assert_eq!(prev_first_child.length(), 0);
	// next :first-child
	let next_first_child = first_child.next_all(":first-child");
	assert_eq!(next_first_child.length(), 0);
	Ok(())
}

#[test]
fn test_selector_pseudo_last_child() -> Result {
	let html = r#"
    <!doctype html>
    <html lang="en">
      <head>
        <meta charset="utf-8">
        <title>:last-child</title>
      </head>
    <body>
      <ul>
        <li>item1</li>
        <li>item2</li>
        <li>item3</li>
        <li>item4</li>
        <li>item5</li>
        <li>item6</li>
        <li>item7</li>
        <li>item8</li>
        <li>item9</li>
      </ul>
    </body>
    </html>
  "#;
	let root = Vis::load(&html)?;
	// :last-child
	let last_child = root.find("li:last-child");
	assert_eq!(last_child.length(), 1);
	assert_eq!(last_child.text(), "item9");
	// prev :first-child
	let prev_last_child = last_child.prev_all(":last-child");
	assert_eq!(prev_last_child.length(), 0);
	// next :first-child
	let next_last_child = last_child.next_all(":last-child");
	assert_eq!(next_last_child.length(), 0);
	Ok(())
}

#[test]
fn test_selector_pseudo_nth_child() -> Result {
	let html = r#"
  <!doctype html>
  <html lang="en">
    <head>
      <meta charset="utf-8">
      <title>:nth-child</title>
    </head>
  <body>
    <ul>
      <li>item1</li>
      <li>item2</li>
      <li>item3</li>
      <li>item4</li>
      <li>item5</li>
      <li>item6</li>
      <li>item7</li>
      <li>item8</li>
      <li>item9</li>
    </ul>
  </body>
  </html>
"#;
	let root = Vis::load(&html)?;
	let ul = root.find("ul");
	// :nth-child(0)
	let child = ul.children("li:nth-child(0)");
	assert_eq!(child.length(), 0);
	// :nth-child(-2n + 3)
	let child = ul.children("li:nth-child(-2n + 3)");
	assert_eq!(child.length(), 2);
	assert_eq!(child.text(), "item1item3");
	// :nth-child(1)
	let child = ul.children("li:nth-child(1)");
	assert_eq!(child.length(), 1);
	assert_eq!(child.text(), "item1");
	// :nth-child(odd)
	let odd_childs = ul.children("li:nth-child(odd)");
	assert_eq!(odd_childs.length(), 5);
	assert_eq!(odd_childs.text(), "item1item3item5item7item9");
	// :nth-child(even)
	let even_childs = ul.children("li:nth-child( even )");
	assert_eq!(even_childs.length(), 4);
	assert_eq!(even_childs.text(), "item2item4item6item8");
	// :nth-child(3n)
	let childs_3n = ul.children("li:nth-child(3n)");
	assert_eq!(childs_3n.length(), 3);
	assert_eq!(childs_3n.text(), "item3item6item9");
	// filter
	let childs_3n_2n = childs_3n.filter(":nth-child(2n)");
	assert_eq!(childs_3n_2n.length(), 1);
	assert_eq!(childs_3n_2n.text(), "item6");
	Ok(())
}

#[test]
fn test_selector_pseudo_nth_last_child() -> Result {
	let html = r#"
    <!doctype html>
    <html lang="en">
      <head>
        <meta charset="utf-8">
        <title>:nth-last-child</title>
      </head>
    <body>
      <ul>
        <li>item1</li>
        <li>item2</li>
        <li>item3</li>
        <li>item4</li>
        <li>item5</li>
        <li>item6</li>
        <li>item7</li>
        <li>item8</li>
        <li>item9</li>
      </ul>
    </body>
    </html>
  "#;
	let root = Vis::load(&html)?;
	let ul = root.find("ul");
	// :nth-last-child(1)
	let child = ul.children("li:nth-last-child(1)");
	assert_eq!(child.length(), 1);
	assert_eq!(child.text(), "item9");
	// :nth-last-child(odd)
	let odd_last_childs = ul.children("li:nth-last-child(odd)");
	assert_eq!(odd_last_childs.length(), 5);
	assert_eq!(odd_last_childs.text(), "item1item3item5item7item9");
	// :nth-last-child(3n)
	let childs_last_3n = ul.children("li:nth-last-child(3n)");
	assert_eq!(childs_last_3n.length(), 3);
	assert_eq!(childs_last_3n.text(), "item1item4item7");
	// :nth-last-child(3n):nth-last-child(2n)
	let childs_last_3n_2n = childs_last_3n.filter(":nth-last-child(2n)");
	assert_eq!(childs_last_3n_2n.length(), 1);
	assert_eq!(childs_last_3n_2n.text(), "item4");
	Ok(())
}

#[test]
fn test_selector_pseudo_only_of_type() -> Result {
	let html = r#"
    <!doctype html>
    <html lang="en">
      <head>
        <meta charset="utf-8">
        <title>:only-of-type</title>
      </head>
    <body>
      <div id="content">
        <strong>only strong</strong>
        This is <span>span1</span>, this is a <b>only b</b>, this is another <span>span2</span>
      </div>
    </body>
    </html>
  "#;
	let root = Vis::load(&html)?;
	let content = root.find("#content");
	// :only-of-type
	let only_of_type = content.children(":only-of-type");
	assert_eq!(only_of_type.length(), 2);
	assert_eq!(only_of_type.text(), "only strongonly b");
	// prev_all
	let prevs_only_of_type = content.find("b").prev_all(":only-of-type");
	assert_eq!(prevs_only_of_type.length(), 1);
	assert_eq!(prevs_only_of_type.text(), "only strong");
	Ok(())
}

#[test]
fn test_selector_pseudo_first_of_type() -> Result {
	let html = r#"
    <!doctype html>
    <html lang="en">
      <head>
        <meta charset="utf-8">
        <title>:first-of-type</title>
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
	// :first-of-type
	let type_child = dl.children(":first-of-type");
	assert_eq!(type_child.length(), 2);
	assert_eq!(type_child.text(), "dt1dd1");
	// prevs
	let type_child_prevs = type_child.prev_all(":first-of-type");
	assert_eq!(type_child_prevs.length(), 1);
	assert_eq!(type_child_prevs.text(), "dt1");
	// nexts
	let type_child_nexts = type_child.next_all(":first-of-type");
	assert_eq!(type_child_nexts.length(), 1);
	assert_eq!(type_child_nexts.text(), "dd1");
	Ok(())
}

#[test]
fn test_selector_pseudo_last_of_type() -> Result {
	let html = r#"
    <!doctype html>
    <html lang="en">
      <head>
        <meta charset="utf-8">
        <title>:last-of-type</title>
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
	// :last-of-type
	let type_child = dl.children(":last-of-type");
	assert_eq!(type_child.length(), 2);
	assert_eq!(type_child.text(), "dt3dd6");
	// prevs
	let type_child_prevs = type_child.prev_all(":last-of-type");
	assert_eq!(type_child_prevs.length(), 1);
	assert_eq!(type_child_prevs.text(), "dt3");
	// nexts
	let type_child_nexts = type_child.next_all(":last-of-type");
	assert_eq!(type_child_nexts.length(), 1);
	assert_eq!(type_child_nexts.text(), "dd6");
	Ok(())
}

#[test]
fn test_selector_pseudo_nth_of_type() -> Result {
	let html = r#"
    <!doctype html>
    <html lang="en">
      <head>
        <meta charset="utf-8">
        <title>:nth-of-type</title>
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
	// :nth-of-type(0)
	let type_child = dl.children(":nth-of-type(0)");
	assert_eq!(type_child.length(), 0);
	// :nth-of-type(1)
	let type_child = dl.children(":nth-of-type(1)");
	assert_eq!(type_child.length(), 2);
	assert_eq!(type_child.text(), "dt1dd1");
	// :nth-of-type(odd)
	let odd_type_childs = dl.children(":nth-of-type(odd)");
	assert_eq!(odd_type_childs.length(), 5);
	assert_eq!(odd_type_childs.text(), "dt1dd1dd3dt3dd5");
	// :nth-of-type(3n)
	let childs_type_3n = dl.children(":nth-of-type(3n)");
	assert_eq!(childs_type_3n.length(), 3);
	assert_eq!(childs_type_3n.text(), "dd3dt3dd6");
	// :nth-of-type(3n):nth-of-type(2n)
	let childs_type_3n_2n = childs_type_3n.filter(":nth-of-type(2n)");
	assert_eq!(childs_type_3n_2n.length(), 1);
	assert_eq!(childs_type_3n_2n.text(), "dd6");
	// prevs
	let childs_type_3n_2n_prevs = childs_type_3n_2n.prev_all(":nth-of-type(3n)");
	assert_eq!(childs_type_3n_2n_prevs.length(), 2);
	assert_eq!(childs_type_3n_2n_prevs.text(), "dd3dt3");
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
	// :nth-last-of-type(3n):nth-last-of-type(2n)
	let childs_type_last_3n_2n = childs_type_last_3n.filter(":nth-last-of-type(2n)");
	assert_eq!(childs_type_last_3n_2n.length(), 1);
	assert_eq!(childs_type_last_3n_2n.text(), "dd1");
	// prevs
	let childs_type_last_3n_2n_prevs = childs_type_last_3n_2n.prev_all(":nth-last-of-type(3n)");
	assert_eq!(childs_type_last_3n_2n_prevs.length(), 1);
	assert_eq!(childs_type_last_3n_2n_prevs.text(), "dt1");
	// nexts
	let childs_type_last_3n_2n_nests = childs_type_last_3n_2n.next_all(":nth-last-of-type(3n)");
	assert_eq!(childs_type_last_3n_2n_nests.length(), 1);
	assert_eq!(childs_type_last_3n_2n_nests.text(), "dd4");
	Ok(())
}

#[test]
fn test_selector_pseudo_not() -> Result {
	let html = r#"
    <!doctype html>
    <html lang="en">
      <head>
        <meta charset="utf-8">
        <title>:not</title>
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
	// not dt
	let not_dt = dl.children(":not(dt)");
	assert_eq!(not_dt.length(), 6);
	// not dt
	let not_dt_first = dl.children(":not(dt:nth-child(-n + 1))");
	assert_eq!(not_dt_first.length(), 8);
	assert_eq!(not_dt_first.eq(0).text(), "dd1");
	Ok(())
}
