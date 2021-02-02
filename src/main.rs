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
  "##;
	let root = Vis::load(html)?;
	let id_term_2 = root.find("#term-2");
	let dd_after_term_2 = id_term_2.next_until("dt", "", true);
	assert_eq!(dd_after_term_2.length(), 3);
	Ok(())
}
