use std::error::Error;
use std::time::SystemTime;
use visdom::Vis;
fn main() -> Result<(), Box<dyn Error>> {
	let html = r##"
    <html>
      <head></head>
      <body>
        <div id="id">
          <div class="class">class-div</div>
          <p>
            p-tag
            <span>ooo</span>
            <span>
              <em>aaaa</em>
            </span>
          </p>
        </div>
      </body>
    </html>
  "##;
	Vis::init();
	let root = Vis::load(html)?;
	let id = root.find("#id")?;
	let children = id.find("*")?;
	let p_has = children.has_in(&id.find("p")?)?;
	println!("p_has:{}", p_has.text());
	Ok(())
}
