use ntree::selector::interface::KindError;
use std::error::Error;
use std::thread;
use std::time::SystemTime;
use visdom::Vis;
fn main() -> Result<(), Box<dyn Error>> {
	let html = r##"
  <html>
    <head></head>
    <body>
      <div id="id" class="id">
        <div class="class">class-div</div>
        <p>
          p-tag
        </p>
      </div>
      <div id="nested" class="nested">
        <div class="outer-div-1">
          outer-1
          <div class="inner-div-1-1">outer-1-inner-1</div>
          <div class="inner-div-1-2">outer-1-inner-2</div>
        </div>
        <div class="outer-div-2">
          outer-2
          <div class="inner-div-2-1">outer-2-inner-1</div>
          <div class="inner-div-2-2">outer-2-inner-1</div>
        </div>
      </div>
    </body>
  </html>
  "##;
	let doc = Vis::load(html)?;
	let div_in_id = doc.find("[class|='outer'],[class|='inner']")?.sort();
	println!("{:?}", div_in_id.eq(1)?.attr("class"));
	Ok(())
}
