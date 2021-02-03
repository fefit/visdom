use mesdoc::interface::IDocumentTrait;
use std::error::Error;
use std::thread;
use std::time::SystemTime;
use visdom::Vis;
fn main() -> Result<(), Box<dyn Error>> {
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
  <style>
    .a{color:red;}
  </style>
  "##;
	let root = Vis::load_catch(
		html,
		Box::new(|e| {
			println!("error is:{}", e.to_string());
		}),
	);
	let id_term_2 = root.find("#term-2");
	let dd_after_term_2 = id_term_2.next_until("dt", "", false);
	assert_eq!(dd_after_term_2.length(), 3);
	let mut style_ele = root.find("style");
	let mut texts = style_ele.texts(1);
	println!("texts:{}", texts.length());
	let mut dl = root.find("dl");
	let mut style_ele = Vis::load("<style>.me{color:red}</style>")?;
	style_ele.insert_before(&mut dl);
	println!("outer html:{}", root.outer_html());
	Ok(())
}
