use std::error::Error;

use visdom::Vis;
fn main() -> Result<(), Box<dyn Error>> {
	Vis::init();
	let html = r##"
        <div id="hello" class="hello"><span>world</span></div>
        <div id="haha">
          <span class="hello other" id="inner">.hello</span><span>2</span><span>3</span><span id="a4">4</span>
        </div>
        <p>
          <a>haha</a>
          <span>5</span>
          <a>b</a>
          <span>6</span>
          <span>7</span>
          <span>8</span>
          <span class="great">.great</span>
          <span>10</span>
          <input type="checkbox" value="1" />
        </p>
    "##;
	let root = Vis::load(html)?;
	// let dom_hello = root.find("span:nth-last-child(2n + 1)");
	// println!("result:{:?}", dom_hello?.text());
	let result = root.find("span:nth-child(odd)")?;
	println!("result:{:?}", result.text());
	let filter = result.filter("div .hello,span + .great")?;
	println!("filter:{:?}", filter.text());
	println!("not:{:?}", filter.not(".hello,div.great")?.text());
	println!("is:{:?}", filter.is("p > span"));
	Ok(())
}
