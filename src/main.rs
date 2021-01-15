use std::error::Error;

use visdom::Vis;
fn main() -> Result<(), Box<dyn Error>> {
	Vis::init();
	let html = r##"
        <div id="hello" class="hello">world</div>
        <div id="haha">
          <span class="hello other" id="inner">1</span><span>2</span><span>3</span><span>4</span>
        </div>
        <p>
          <span>5</span>
          <span>6</span>
          <span>7</span>
          <span>8</span>
          <span>9</span>
          <span>10</span>
        </p>
    "##;
	let root = Vis::load(html)?;
	let dom_hello = root.find("span:last-of-type");
	println!("result:{:?}", dom_hello?.text());
	Ok(())
}
