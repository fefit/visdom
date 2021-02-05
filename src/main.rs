use mesdoc::interface::IDocumentTrait;
use std::error::Error;
use std::thread;
use std::time::SystemTime;
use visdom::Vis;
fn main() -> Result<(), Box<dyn Error>> {
	let html: String = format!(
		r##"
      <ul>
        {}
      </ul>
    "##,
		String::from("<li></li>").repeat(1000)
	);
	const TIMES: u32 = 200;
	let root = Vis::load(&html)?;

	// println!("start....");
	// let start_time = SystemTime::now();
	// for _ in 0..TIMES {
	// 	let list = root.children("ul");
	// 	let some_child = list.children("").eq(100);
	// 	println!("{}", some_child.prev_all("").get(99).unwrap().index());
	// }
	// let end_time = SystemTime::now();
	// let used_time = end_time.duration_since(start_time)?;
	// println!(
	// 	"take time:{:?}\navg time：{:?}",
	// 	used_time,
	// 	used_time / TIMES
	// );
	let root = Vis::load(
		r#"
    <div><p></p><ul></ul><ol></ol></div>
  "#,
	)?;
	let mut p = root.find("p");
	println!("p:{}", p.get(0).unwrap().index());
	let mut ul = root.find("ul");
	println!("ul:{}", ul.get(0).unwrap().index());
	p.remove();
	println!("ul:{}", ul.get(0).unwrap().index());
	let root = Vis::load(r##"<div class="parent"><div class="first_child"></div></div>"##)?;
	let mut parent = root.find(".parent");
	let first_child = parent.find(".first_child");
	let mut new_elements =
		Vis::load(r#"<div class="second-child"></div><div class="third-child"></div>"#)?;
	println!("first_child:{}", first_child.get(0).unwrap().index());
	parent.append(&mut new_elements);
	println!("first_child:{}", first_child.get(0).unwrap().index());
	let childs = parent.children("");
	println!("childs:{}", childs.length());
	let last_child = childs.eq(childs.length() - 1);
	println!("childs:{}", last_child.get(0).unwrap().index());
	let mut new_elements = Vis::load(r#"<div class="parent-1"></div><div class="parent-2"></div>"#)?;
	new_elements.insert_before(&mut parent);
	println!("parent:{}", parent.get(0).unwrap().index());
	Ok(())
}
