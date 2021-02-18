use mesdoc::interface::IDocumentTrait;
use std::thread;
use std::time::SystemTime;
use std::{collections::VecDeque, error::Error};
use visdom::Vis;

fn main() -> Result<(), Box<dyn Error>> {
	const HTML: &str = r##"
	  <html>
	    <head>
        <title><div> tag</title>
      </head>
	    <body>
	      <div id="id">
	        <div class="class">class-div</div>
	        <p>
	          p-tag
	        </p>
	      </div>
	      <div id="nested">
	        <div class="outer-div-1">
	          <div class="inner-div-1-1">inner-div-1-1</div>
	          <div class="inner-div-1-2">inner-div-<span>1</span>-<span>2</span></div>
	        </div>
	        <div class="outer-div-2">
	          <div class="inner-div-2-1"></div>
	          <div class="inner-div-2-2" id="inner"></div>
	        </div>
	      </div>
	    </body>
	  </html>
	"##;
	let root = Vis::load(HTML)?;

	let eles = root.find("body, #nested, #id");
	println!("eles:{:?}", eles.length());
	let doc = &eles.get(0).unwrap().owner_document().unwrap();
	println!("doc.title{:?}", doc.title());
	let tests = eles.find(".inner-div-2-1");
	println!("tests:{}", tests.length());
	// let inner = root.find("#nested > .outer-div-2 > #inner.inner-div-2-2");
	// println!("inner:{}", inner.length());
	// let divs = root.find("div");
	// let id_ele = divs.filter("#id");
	// assert_eq!(id_ele.length(), 1);
	// let div_in_id = divs.filter("#id > *");
	// assert_eq!(div_in_id.length(), 1);
	// let outer_div_in_nested = divs.filter("#nested > [class|='outer']");
	// assert_eq!(outer_div_in_nested.length(), 2);
	// let inner_div_in_nested = divs.filter("#nested > [class|='outer'] > [class|='inner']");
	// assert_eq!(inner_div_in_nested.length(), 4);
	// let id_not_ok_ele = divs.filter("div > #id");
	// assert_eq!(id_not_ok_ele.length(), 0);
	// let id_ok_ele = divs.filter("html body > #id");
	// assert_eq!(id_ok_ele.length(), 1);
	// const TOTAL: usize = 2000;
	// let html: String = format!(
	// 	r##"
	//     <dl>{}{}{}</dl>
	//   "##,
	// 	String::from("<dt><span></span></dt>").repeat(TOTAL),
	// 	String::from("<dd><span></span><b></b><c></c></dd>").repeat(TOTAL),
	// 	String::from("<li></li>")
	// );
	// const TIMES: u32 = 200;
	// let root = Vis::load(&html)?;
	// let ul = root.children("dl");
	// const SELECTOR: &str = "dl :empty-child";
	// println!(r#"html: <ul>{{"<li></li>".repeat({})}}</ul>"#, TOTAL);
	// println!(r#"查找：ul.children("{}")"#, SELECTOR);
	// let searchs = ul.children("dt,dd,li").siblings("dt");
	// println!("共找到节点数：{}", searchs.length());
	// // println!("{}", searchs.last().parent("").get(0).unwrap().index());
	// // println!(
	// // 	"{}",
	// // 	ul.children("").filter(SELECTOR).get(0).unwrap().index()
	// // );
	// println!("执行{}次求平均时间...", TIMES);
	// let start_time = SystemTime::now();
	// for _ in 0..TIMES {
	// 	let childs = ul.children("dt,dd,li").siblings("dt");
	// }
	// let end_time = SystemTime::now();
	// let used_time = end_time.duration_since(start_time)?;
	// println!(
	// 	"共消耗时间:{:?}，平均时间:{:?}",
	// 	used_time,
	// 	used_time / TIMES
	// );
	// let root = Vis::load(
	// 	r#"
	//     <div class="closest">
	//       <p>
	//         <a class="closest">aaa</a>
	//         <b class="closest">bbb</b>
	//         <c>ccc</c>
	//       </p>
	//       <a>top-aaaa</a>
	//     </div>
	// "#,
	// )?;
	// let a = root.find("b,c").add(root.find("a"));
	// let first = a.slice(0..1);
	// println!("first:{}", first.length());
	// println!("a:{}", a.length());
	// let closest = a.closest(".closest");
	// println!("length:{}", closest.length());
	// println!("closest:{}", closest.eq(0).outer_html());
	// let siblings = a.siblings("");
	// println!("siblings:{}", siblings.length());
	// println!("first:{}", siblings.eq(0).get(0).unwrap().tag_name());

	// 	let html = r##"
	//   <html>
	//   <head></head>
	//   <body>
	//     <div id="id">
	//       <div class="class">class-div</div>
	//       <p>
	//         p-tag
	//       </p>
	//     </div>
	//     <div id="nested">
	//       <div class="outer-div-1">
	//         <div class="inner-div-1-1">inner-div-1-1</div>
	//         <div class="inner-div-1-2">inner-div-<span>1</span>-<span>2</span></div>
	//       </div>
	//       <div class="outer-div-2">
	//         <div class="inner-div-2-1"></div>
	//         <div class="inner-div-2-2"></div>
	//       </div>
	//     </div>
	//   </body>
	// </html>
	//   "##;
	// 	let root = Vis::load(html)?;
	// 	let nested_divs = root.find("[class|='outer'],[class|='inner']");
	// 	println!("{}", nested_divs.length());
	// 	println!("{:?}", nested_divs.eq(5).attr("class"));
	// let root = Vis::load(
	// 	r#"
	//   <div><p></p><ul></ul><ol></ol></div>
	// "#,
	// )?;
	// let mut p = root.find("p");
	// println!("p:{}", p.get(0).unwrap().index());
	// let mut ul = root.find("ul");
	// println!("ul:{}", ul.get(0).unwrap().index());
	// p.remove();
	// println!("ul:{}", ul.get(0).unwrap().index());
	// let root = Vis::load(r##"<div class="parent"><div class="first_child"></div></div>"##)?;
	// let mut parent = root.find(".parent");
	// let first_child = parent.find(".first_child");
	// let mut new_elements =
	// 	Vis::load(r#"<div class="second-child"></div><div class="third-child"></div>"#)?;
	// println!("first_child:{}", first_child.get(0).unwrap().index());
	// parent.append(&mut new_elements);
	// println!("first_child:{}", first_child.get(0).unwrap().index());
	// let childs = parent.children("");
	// println!("childs:{}", childs.length());
	// let last_child = childs.eq(childs.length() - 1);
	// println!("last_child:{}", last_child.get(0).unwrap().index());
	// let mut new_elements = Vis::load(r#"<div class="parent-1"></div><div class="parent-2"></div>"#)?;
	// new_elements.insert_before(&mut parent);
	// println!("parent:{}", parent.get(0).unwrap().index());
	// println!("root:{}", root.outer_html());
	// let mut new_elements = Vis::load(r#"<div class="zero-child"></div>"#)?;
	// new_elements.prepend_to(&mut parent);
	// println!("root:{}", root.outer_html());
	// println!("first_child:{}", first_child.get(0).unwrap().index());
	// first_child.remove();
	// let childs = parent.children("");
	// let last_child = childs.eq(childs.length() - 1);
	// println!("last_child:{}", last_child.get(0).unwrap().index());
	// println!("root:{}", root.outer_html());

	// let root = Vis::load(r##"<div class="parent">某段文字<span>span</span>再一段文字</div>"##)?;
	// let mut texts = root.find(".parent").texts(1);
	// texts.for_each(|index, ele| {
	// 	if index == 0 {
	// 		println!("ele:{}", ele.text());
	// 		ele.set_html("<span>a</span><span>b</span>");
	// 	}
	// 	true
	// });
	// println!("{}", root.outer_html());
	Ok(())
}
