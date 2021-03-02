use std::error::Error;
use std::time::SystemTime;
use visdom::Vis;
const TIMES: u32 = 200;
const TOTAL: usize = 3000;
fn main() -> Result<(), Box<dyn Error>> {
	let html: String = format!(
		r##"
	    <ul>
	      {}
	    </ul>
	  "##,
		String::from("<li></li>").repeat(TOTAL)
	);

	let root = Vis::load(&html)?;
	let ul = root.children("ul");
	const SELECTOR: &str = ":nth-child(2n),:nth-child(3n),:nth-child(5n)";

	let start_time = SystemTime::now();
	for _ in 0..TIMES {
		ul.find(SELECTOR);
	}
	let end_time = SystemTime::now();
	let elapsed = end_time.duration_since(start_time)?;
	println!(
		"Elapsed: {:?}, Average Time: {:?}",
		elapsed,
		elapsed / TIMES
	);
	Ok(())
}
