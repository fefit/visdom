use std::error::Error;

use visdom::Vis;
fn main()->Result<(), Box<dyn Error>> {
    Vis::init();
    let html = r##"
        <div id="hello" class="hello">world</div>
        <div><span class="hello other"></span></div>
    "##;
    let root = Vis::load(html)?;
    let dom_hello = root.find("[class^='hello']");
    println!("结果：{:?}", dom_hello?.count());
    Ok(())
}
