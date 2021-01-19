use std::error::Error;
use std::time::SystemTime;
use visdom::Vis;
fn main() -> Result<(), Box<dyn Error>> {
	Vis::init();
	let html = r##"
      <!DOCTYPE html>
      <!--[if lt IE 7 ]><html class="ie6"><![endif]-->
      <!--[if IE 7 ]><html class="ie7"><![endif]-->
      <!--[if IE 8 ]><html class="ie8"><![endif]-->
      <!--[if IE 9 ]><html class="ie9"><![endif]-->
      <!--[if (gt IE 9)|!(IE)]><!--><html><!--<![endif]-->
      <head>
        <meta charset="utf-8">
        <meta content="always" name="referrer">
        <meta http-equiv="X-UA-Compatible" content="IE=edge,chrome=1">
        <title></title>
      </head>
      <body>
        <div id="test" class="test">
          test1  
          <a>a</a>
          <div class="inner">
            <b>b</b>
            <span class="inner"></span>
          </div>
        </div>
        <div class="test">
          test2
        </div>
        <div id="lazy">
        </div>
      </body>
      </html>
  "##;
	let  root = Vis::load(html)?;
  let mut test = root.find(".test")?;
  println!("test:{}", test.length());
  println!("root:{}", root.outer_html());
  let childs = Vis::load("多分几份辣椒酱")?;
  childs.prepend_to(&mut test);
  childs.find("div")?.set_attr("readonly", Some("readonly"));
  println!("test:{}", root.outer_html());
  println!("children:{}", test.find(".inner")?.length());
  let mut lazy = root.find("#lazy")?;
  lazy.append(&test);
  println!("test:{}", root.outer_html());
	Ok(())
}
