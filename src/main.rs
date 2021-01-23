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
          </p>
        </div>
      </body>
    </html>
  "##;
	Vis::init();
	let root = Vis::load(html)?;
	let id = root.find("#id")?;
	let children = id.find("*")?;
	println!("children:{}", children.length());
	Ok(())
}
