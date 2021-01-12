use std::error::Error;

use visdom::Vis;
fn main() -> Result<(), Box<dyn Error>> {
	Vis::init();
	let html = r##"
        <div id="hello" class="hello">world</div>
        <div id="haha"><span class="hello other" id="inner">implement this</span></div>
    "##;
	let root = Vis::load(html)?;
	let dom_hello = root.find(".hello.other");
	println!("result:{:?}", dom_hello?.attr("id"));
	Ok(())
}
