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
	Ok(())
}
