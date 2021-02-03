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
	println!("start....");
	let start_time = SystemTime::now();
	for _ in 0..TIMES {
		let list = root.children("ul");
		list.find(":nth-last-child(3n + 1)");
	}
	let end_time = SystemTime::now();
	let used_time = end_time.duration_since(start_time)?;
	println!(
		"take time:{:?}\navg time：{:?}",
		used_time,
		used_time / TIMES
	);
	Ok(())
}
