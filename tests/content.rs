use std::result::Result as StdResult;
#[cfg(feature = "text")]
use visdom::types::INodeType;
use visdom::{types::BoxDynError, Vis};
type Result = StdResult<(), BoxDynError>;

#[test]
fn test_val() -> Result {
	// ---- input ----
	let html = r#"
  <input type="text" value="textinput" />
  <input type="radio" name="radioinput" value="radio1" />
  <input type="radio" name="radioinput" value="radio2" checked="checked" />
  <input type="radio" name="radioinput" value="radio3" />
  <input type="checkbox" name="checkboxinput" value="checkbox1" />
  <input type="checkbox" name="checkboxinput" value="checkbox2" checked="checked" />
  <input type="checkbox" name="checkboxinput" value="checkbox3" checked="checked" />
  <p value="abc"></p>
  <input type="text" name="novalue" />
  "#;
	let root = Vis::load(html)?;
	let inputs = root.children("input");
	assert_eq!(inputs.val().to_string(), "textinput");
	assert_eq!(inputs.filter("[type='radio']").length(), 3);
	assert_eq!(inputs.filter("[type='radio']:checked").length(), 1);
	assert_eq!(
		inputs.filter("[type='radio']:checked").val().to_string(),
		"radio2"
	);
	assert_eq!(
		inputs.filter("[name='radioinput']").eq(0).val().to_string(),
		"radio1"
	);
	assert_eq!(inputs.filter("[type='checkbox']").length(), 3);
	assert_eq!(inputs.filter("[type='checkbox']:checked").length(), 2);
	assert_eq!(
		inputs.filter("[type='checkbox']:checked").val().to_string(),
		"checkbox2"
	);
	assert_eq!(root.find("p").length(), 1);
	assert_eq!(root.find("p").val().to_string(), "");
	assert_eq!(root.find("div").val().to_string(), "");
	assert_eq!(root.find("input[name='novalue']").length(), 1);
	assert_eq!(root.find("input[name='novalue']").val().to_string(), "");
	// ---- textarea ----
	let textarea_content = r#"<div>This is the content in textarea</div>"#;
	let html = format!("<textarea>{}</textarea>", textarea_content);
	let root = Vis::load(&html)?;
	let textarea = root.children("textarea");
	assert_eq!(textarea.val().to_string(), textarea_content);
	// ---- select ----
	// select without selected option
	let html = r#"
  <select>
    <option value="1">1</option>
    <option value="2">2</option>
    <option value="3">3</option>
  </select>
  "#;
	let root = Vis::load(html)?;
	let select = root.children("select");
	assert_eq!(select.find("option:checked").length(), 1);
	assert_eq!(select.find("option:checked").val().to_string(), "1");
	assert_eq!(select.val().to_string(), "1");
	// select without selected option, but in optgroup
	let html = r#"
  <select>
    <optgroup>
      <option value="1">1</option>
      <option value="2">2</option>
      <option value="3">3</option>
    </optgroup>
  </select>
  "#;
	let root = Vis::load(html)?;
	let select = root.children("select");
	assert_eq!(select.find("option:checked").length(), 0);
	assert_eq!(select.val().to_string(), "");
	// select with selected option
	let html = r#"
  <select>
    <option value="1">1</option>
    <option value="2" selected="selected">2</option>
    <option value="3">3</option>
  </select>
  "#;
	let root = Vis::load(html)?;
	let select = root.children("select");
	assert_eq!(select.find("option:checked").length(), 1);
	assert_eq!(select.val().to_string(), "2");
	// select with selected option in optgroup
	let html = r#"
  <select>
    <optgroup>
      <option value="1">1</option>
      <option value="2" selected="selected">2</option>
      <option value="3">3</option>
    </optgroup>
  </select>
  "#;
	let root = Vis::load(html)?;
	let select = root.children("select");
	assert_eq!(select.find("option:checked").length(), 1);
	assert_eq!(select.val().to_string(), "2");
	// multiple select with selected option
	let html = r#"
  <select multiple>
    <option value="1">1</option>
    <option value="2">2</option>
    <option value="3">3</option>
  </select>
  "#;
	let root = Vis::load(html)?;
	let select = root.children("select");
	assert_eq!(select.find("option:checked").length(), 0);
	assert_eq!(select.val().to_string(), "");
	// multiple select with selected option in optgroup
	let html = r#"
  <select multiple>
    <optgroup>
      <option value="1">1</option>
      <option value="2">2</option>
      <option value="3">3</option>
    </optgroup>
  </select>
  "#;
	let root = Vis::load(html)?;
	let select = root.children("select");
	assert_eq!(select.find("option:checked").length(), 0);
	assert_eq!(select.val().to_string(), "");
	// multiple select with selected option in optgroup
	let html = r#"
  <select multiple>
    <optgroup>
      <option value="1">1</option>
      <option value="2" selected>2</option>
      <option value="3" selected>3</option>
    </optgroup>
    <optgroup>
      <option value="4">4</option>
      <option value="5" selected>5</option>
      <option value="6">6</option>
    </optgroup>
    <option value="7" selected>7</option>
  </select>
  "#;
	let root = Vis::load(html)?;
	let select = root.children("select");
	assert_eq!(select.find("option:checked").length(), 4);
	assert_eq!(select.val().to_string(), "2,3,5,7");
	assert_eq!(select.val().into_iter().collect::<String>(), "2357");
	Ok(())
}

#[test]
fn test_set_html() -> Result {
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
	Ok(())
}

#[test]
#[cfg(feature = "text")]
fn test_text_set_html() -> Result {
	// text node
	let text = "This is a test!";
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
fn test_set_text() -> Result {
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
	// pre tag
	let html: &str = r#"<pre class="parent"></pre>"#;
	let root = Vis::load(html)?;
	let mut parent = root.children(".parent");
	parent.set_text(setted);
	assert_eq!(parent.html(), encoded_setted);
	assert_eq!(parent.text(), setted);
	assert_eq!(parent.children("strong").length(), 0);
	parent.set_text("");
	assert!(parent.text().is_empty());
	// script tag
	let inner_script = "var a = 1;";
	let html = format!(r#"<script>{}</script>"#, inner_script);
	let root = Vis::load(&html)?;
	let mut script = root.find("script");
	assert_eq!(script.length(), 1);
	assert_eq!(script.text(), inner_script);
	// set text
	let inner_script = "var b = 2;";
	script.set_text(inner_script);
	assert_eq!(script.text(), inner_script);
	// style tag
	let root = Vis::load("<style></style>")?;
	let mut style = root.find("style");
	assert_eq!(style.length(), 1);
	assert_eq!(style.text(), "");
	let inner_style = "body{background:blue;}";
	style.set_html(inner_style);
	assert_eq!(style.text(), inner_style);
	assert_eq!(style.html(), inner_style);
	Ok(())
}

#[test]
fn test_text_content() {}

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
fn test_inner_htmls() -> Result {
	let inner_html = "abc<span>def</span>ghj";
	let code = format!("<div>{}</div><div>{}</div>", inner_html, inner_html);
	let root = Vis::load(&code)?;
	assert_eq!(root.find("div").eq(0).htmls(), inner_html);
	assert_eq!(
		root.find("div").htmls(),
		format!("{}{}", inner_html, inner_html)
	);
	assert_eq!(root.find("p").htmls(), "");
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

#[test]
fn test_outer_htmls() -> Result {
	let inner_html = "abc<span>def</span>ghj";
	let code = format!("<div>{}</div><div>{}</div>", inner_html, inner_html);
	let root = Vis::load(&code)?;
	assert_eq!(
		root.find("div").eq(0).outer_htmls(),
		format!("<div>{}</div>", inner_html)
	);
	assert_eq!(root.find("div").outer_htmls(), code);
	assert_eq!(root.find("p").outer_htmls(), "");
	Ok(())
}

#[test]
#[cfg(feature = "text")]
fn test_texts() -> Result {
	let html = r##"
    <div id="content">FIRST-ABC<div>SECOND-ABC<style>.a{{color:red}}</style>SECOND-DEF</div><script>var a = 1;</script>FIRST-DEF</div>
  "##;
	let root = Vis::load(html)?;
	let content = root.find("#content");
	let texts = content.texts(0);
	assert_eq!(texts.length(), 6);
	// top childs
	let texts_limit = content.texts(1);
	assert_eq!(texts_limit.length(), 3);
	// filters, ignore content nodes such as style/script
	let texts_filter = content.texts_by(
		0,
		Box::new(|_, node| !matches!(node.node_type(), INodeType::Element)),
	);
	assert_eq!(texts_filter.length(), 4);
	// filter also with limit depth
	let texts_filter = content.texts_by(
		1,
		Box::new(|_, node| !matches!(node.node_type(), INodeType::Element)),
	);
	assert_eq!(texts_filter.length(), 2);
	// just content tags
	let html = r##"<script>var a = 1;</script>"##;
	let root = Vis::load(html)?;
	let script = root.find("script");
	let mut texts = script.texts(0);
	assert_eq!(texts.length(), 1);
	texts.for_each(|_, node| {
		assert_eq!(node.text(), "var a = 1;");
		true
	});
	// filter content tags
	let texts = script.texts_by(
		0,
		Box::new(|_, node| !matches!(node.node_type(), INodeType::Element)),
	);
	assert_eq!(texts.length(), 0);
	// filter content tags and other tags
	let html = r##"<div id="text">abc<script>var a = 1;</script><svg xmlns="http://www.w3.org/2000/svg" version="1.1"><text x="0" y="15" fill="red" transform="rotate(30 20,40)">I love SVG</text></svg></div>"##;
	let root = Vis::load(html)?;
	let text_div = root.find("#text");
	assert_eq!(text_div.texts(0).length(), 3);
	assert_eq!(
		text_div
			.texts_by(
				0,
				Box::new(|_, text_node| { !matches!(text_node.node_type(), INodeType::Element) })
			)
			.length(),
		2
	);
	assert_eq!(
		text_div
			.texts_by_rec(
				0,
				Box::new(|_, text_node| { !matches!(text_node.node_type(), INodeType::Element) }),
				Box::new(|ele| {
					let tag_name = ele.tag_name();
					tag_name != "SVG"
				})
			)
			.length(),
		1
	);
	Ok(())
}
