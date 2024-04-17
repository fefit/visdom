#![allow(clippy::unnecessary_wraps)]
use std::thread;
use std::time::SystemTime;
use std::{collections::VecDeque, error::Error};
use visdom::types::{BoxDynError, IDocumentTrait};
use visdom::types::{BoxDynNode, INodeType};
use visdom::Vis;

fn main() -> Result<(), BoxDynError> {
	// let html = format!("<ul>{}</ul>", "<li></li>".repeat(9));
	// let root = Vis::load(&html)?;
	// let ul = root.find("ul");
	// let nth_2n_child = ul.find(":nth-child(2n),:nth-child(1),:nth-child(n+8)");
	// println!("2n:{}", nth_2n_child.length());
	// 	let html = r##"
	//   <!doctype html>
	//   <html lang="utf-8">
	//     <head></head>
	//     <body>
	//       <div id="id" name="#id">
	//         <div class="class" name="#id .class">class-div</div>
	//         <p>
	//           p-tag
	//         </p>
	//       </div>
	//       <div id="nested" name="#nested">
	//         <div class="outer-div-1" name="#nested .outer-div-1">
	//           <div class="inner-div-1-1" name="#nested .outer-div-1 .inner-div-1-1">inner-div-1-1</div>
	//           <div class="inner-div-1-2" name="#nested .outer-div-1 .inner-div-1-2">inner-div-<span>1</span>-<span>2</span></div>
	//         </div>
	//         <div class="outer-div-2" name="#nested .outer-div-2">
	//           <div class="inner-div-2-1" name="#nested .outer-div-2 .inner-div-2-1"></div>
	//           <div class="inner-div-2-2" name="#nested .outer-div-2 .inner-div-2-1"></div>
	//         </div>
	//       </div>
	//     </body>
	//   </html>
	// "##;
	// let div = root.find("div");
	// println!("div:{}", div.length());
	// let child_divs = div.find("div:nth-child(1)");
	// println!("{}", child_divs.length());
	// child_divs.map(|_, ele| {
	// 	println!("{:?}:{}", ele.get_attribute("name"), ele.tag_name());
	// });
	// println!(
	// 	"{}",
	// 	ul.find(":nth-child(3n),:nth-child(2n),:nth-child(6n)")
	// 		.length()
	// );
	// println!("{:?}", nth_2n_child.length());
	// 	let html = format!(
	// 		r##"<!doctype html>
	//   <html lang="en">
	//     <head>
	//       <meta charset="utf-8">
	//       <title>:nth-last-of-type</title>
	//     </head>
	//   <body>
	//     <dl>
	//       {}
	//       <dt>dt1</dt>
	//         <dd>dd1</dd>
	//         <dd>dd2</dd>
	//         <dd>dd3</dd>
	//       <dt id="dt2">dt2</dt>
	//         <dd>dd4</dd>
	//       <dt>dt3</dt>
	//         <dd>dd5</dd>
	//         <dd>dd6</dd>
	//     </dl>
	//   </body>
	//   </html>"##,
	// 		"<dt></dt>".repeat(3000)
	// 	);
	// 	let root = Vis::load_catch(
	// 		&html,
	// 		Box::new(|e| {
	// 			println!("e:{:?}", e);
	// 		}),
	// 	);
	// 	let start_time = SystemTime::now();
	// 	let dl = root.find("dl");
	// 	let child = dl.find(":nth-child(5)");
	// 	println!("child.length():{}", child.length());
	// 	println!(
	// 		"消耗时间：{:?}",
	// 		SystemTime::now().duration_since(start_time)? / 1
	// 	);
	// let root = Vis::load(&html)?;
	// let dl = root.find("dl");
	// // :last-of-type
	// println!("---------------:last-of-type-------------");
	// let last_type_child = dl.children(":last-of-type");
	// println!("last_type_child.length() = {}", last_type_child.length());
	// println!("last_type_child.text() = {:?}", last_type_child.text());
	// let html: &str = r#"<input type="text" READONly /></div>"#;
	// let root = Vis::load(html)?;
	// let input = root.children("[readOnly]");
	// println!("input:{}", input.length());
	// let texts = content
	// 	.texts(0)
	// 	.filter_by(|_, e| !matches!(e.node_type(), INodeType::Element));
	// println!("texts{}", texts.length());
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
	// let html = r#"
	// <!doctype html>
	// <html>
	//   <body>
	//    <dl>
	//      <dt>Title</dt>
	//      <dd><span>item1</span></dd>
	//      <dd class="item2"><span>item2</span></dd>
	//      <dd class="item3">item3</dd>
	//    </dl>

	//   </body>
	// </html>
	// "#;
	// let root = Vis::load(html)?;
	// let items = root.find("dl > *");
	// println!(
	// 	"items:{}, items:hasnotspan:{}",
	// 	items.length(),
	// 	items.has(":not(span)").length()
	// );
	// first.map(|_, ele| {
	// 	println!("{:?}", ele.tag_name());
	// });
	// println!("{}", first.get(0).unwrap().tag_name());
	// println!("root:{}", root.find("div").length());
	// println!("root:{:?}", root.find("#content").length());
	let html = r#"
  <!--注释-->
  <!doctype html>
  <html>
    <body>
	   <div id="content">
      <!--content注释-->
      这是一些测试数据
      <script>/*js*/var a = 1;var b = 2;</script>
      内容
      <svg xmlns="http://www.w3.org/2000/svg" version="1.1">
        <text x="0" y="15" fill="red" transform="rotate(30 20,40)">I love SVG</text>
      </svg>
      <div id="test_clone">
        <div class="abc">hello</div>
        great!
        <p class="def">world!</div>
      </div>
     </div>
    </body>
  </html>
  "#;
	let root = Vis::load(html)?;
	// let div_no_has_p = root.find("div:not(:has(p))");
	// println!("div_no_has_p: {}", div_no_has_p.text());
	// let divs = root.find("div");
	// let div_has_p = divs.has("p");
	// println!("div_has_p: {}", div_has_p.text());
	// let div_no_has_p = divs.not(":has(p)");
	// println!("div_no_has_p: {}", div_no_has_p.text());
	let content = root.find("#content");
	content.texts_by_rec(
		0,
		Box::new(|_, text_node| {
			println!("{}", text_node.text());
			true
		}),
		Box::new(|ele| {
			let tag_name = ele.tag_name();
			true
		}),
	);
	let mut childs = content.get(0).unwrap().child_nodes();
	for child in childs.iter_mut() {
		if matches!(child.node_type(), INodeType::Comment) {
			child.set_text("abc");
		}
	}
	let pseduo_root = root.find(":root");
	let child_nodes = root.get(0).unwrap().child_nodes();
	println!("{}", child_nodes[1].text_content());
	println!("{}", content.html());
	let mut test_clone = root.find("#test_clone");
	println!("test_clone:{:?}", test_clone.html());
	let test_clone_new = test_clone.clone();
	test_clone_new.find(".abc").set_text("哈哈哈");
	println!("test_clone:{:?}", test_clone.html());
	println!("test_clone_new:{:?}", test_clone_new.html());
	test_clone_new.find(".abc").append_to(&mut test_clone);
	println!("test_clone:{:?}", test_clone.html());
	println!("test_clone_new:{:?}", test_clone_new.html());
	Ok(())
}
