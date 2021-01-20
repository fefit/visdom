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
          <span class="span-1">span-1
            <strong>
              <span>strong-span</span>
            </strong>
          </span>
          <span class="span0">span0</span>
          <span class="span1">
            outer span1
            <span>
              innser span1
            </span>
          </span>
          <span>
            outer span2
            <span>
              innser span2
            </span>
          </span>
          <span class="single-span">
            span3
          </span>
          <span class="single-span">
            span4
          </span>
        </div>
      </body>
      </html>
  "##;
	let root = Vis::load(html)?;
	let mut test = root.find(".test")?;
	// println!("test:{}", test.length());
	// println!("root:{}", root.outer_html());
	// let childs = Vis::load("多分几份辣椒酱")?;
	// childs.prepend_to(&mut test);
	// childs.find("div")?.set_attr("readonly", Some("readonly"));
	// println!("test:{}", root.outer_html());
	// println!("children:{}", test.find(".inner")?.length());
	// let lazy = root.find("#lazy")?;
	// let children = lazy.children(">span")?;
	// println!("test:{}", children.text());
	// let span1 = lazy.find(".span1")?;
	// println!("span1:{}",span1.text());
	// println!("span1_length:{}",span1.length());
	// let next = span1.next("")?;
	// println!("span1_next.length:{}", next.length());
	// println!("span1_next:{}", next.text());
	// let next_all = span1.next_all("")?;
	// println!("span1_next.length:{}", next_all.length());
	// println!("span1_next:{}", next_all.text());
	// let next_all_selector = span1.next_all(".single-span")?;
	// println!("span1_next_selector.length:{}", next_all_selector.length());
	// println!("span1_next_selector:{}", next_all_selector.text());
	// let prev = span1.prev("")?;
	// println!("prev.length:{}", prev.length());
	// println!("prev:{}", prev.text());
	// let prev_selector = span1.prev(".span0")?;
	// println!("prev_selector.length:{}", prev_selector.length());
	// println!("prev_selector:{}", prev_selector.text());
	// let prev_all = span1.prev_all("")?;
	// println!("prev_all.length:{}", prev_all.length());
	// println!("prev_all:{}", prev_all.text());
	// let prev_all_selector = span1.prev_all(".span-1")?;
	// println!("prev_all_selector.length:{}", prev_all_selector.length());
	// println!("prev_all__selector:{}", prev_all_selector.text());
	// let siblings = span1.siblings("")?;
	// println!("siblings.length:{}", siblings.length());
	// println!("siblings:{}", siblings.text());
	// let siblings_selector = span1.siblings(".span0,.single-span")?;
	// println!("siblings_selector.length:{}", siblings_selector.length());
	// println!("siblings_selector:{}", siblings_selector.text());
	// let children = lazy.find("span")?;
	// println!("span.length: {}", children.length());
	// println!("span: {}", children.text());
	let not = test.filter(":not(:not(.a))")?;
	println!("length:{}", not.length());
	println!("text:{}", not.text());
	Ok(())
}
