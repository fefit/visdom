use mesdoc::interface::IDocumentTrait;
use std::error::Error;
use std::thread;
use std::time::SystemTime;
use visdom::Vis;

fn main() -> Result<(), Box<dyn Error>> {
	// const TOTAL: usize = 4000;
	// let html: String = format!(
	// 	r##"
	//     <ul>
	//       {}
	//     </ul>
	//   "##,
	// 	String::from("<li></li>").repeat(TOTAL)
	// );
	// const TIMES: u32 = 200;
	// let root = Vis::load(&html)?;
	// let ul = root.children("ul");
	// const SELECTOR: &str = ":nth-child(2n),:nth-child(3n),:nth-child(5n)";
	// println!(r#"html: <ul>{{"<li></li>".repeat({})}}</ul>"#, TOTAL);
	// println!(r#"search：ul.children("{}")"#, SELECTOR);
	// println!("find nodes：{}", ul.children(SELECTOR).length());
	// println!("execute {} times to get average time...", TIMES);
	// let start_time = SystemTime::now();
	// for _ in 0..TIMES {
	// 	let childs = ul.children(SELECTOR);
	// }
	// let end_time = SystemTime::now();
	// let used_time = end_time.duration_since(start_time)?;
	// println!(
	// 	"total take time:{:?}\naverage time:{:?}",
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
    <div id="nested">
      <div class="outer-div-1">
        <div class="inner-div-1-1">inner-div-1-1</div>
        <div class="inner-div-1-2">inner-div-<span>1</span>-<span>2</span></div>
      </div>
      <div class="outer-div-2">
        <div class="inner-div-2-1"></div>
        <div class="inner-div-2-2"></div>
      </div>
    </div>
  </body>
</html>
  "##;
	let root = Vis::load(html)?;
	let nested_divs = root.find("[class|='outer'],[class|='inner']");
	println!("{}", nested_divs.length());
	println!("{:?}", nested_divs.eq(1).attr("class"));
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
