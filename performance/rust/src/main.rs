use std::error::Error;
use std::time::SystemTime;
use visdom::Vis;
fn main() -> Result<(), Box<dyn Error>> {
	const TOTAL: usize = 3000;
	let html: String = format!(
		r##"
	    <ul>
	      {}
	    </ul>
	  "##,
		String::from("<li></li>").repeat(TOTAL)
	);
	const TIMES: u32 = 200;
	let root = Vis::load(&html)?;
	let ul = root.children("ul");
	const SELECTOR: &str = ":nth-child(2n),:nth-child(3n),:nth-child(5n)";
	println!(r#"Html: <ul>{{"<li></li>".repeat({})}}</ul>"#, TOTAL);
	println!(r#"Query: ul.children("{}")"#, SELECTOR);
	println!("Find matched: {}", ul.children(SELECTOR).length());
	println!("Execute {} times to get average time:", TIMES);
	let start_time = SystemTime::now();
	for _ in 0..TIMES {
		let _childs = ul.children(SELECTOR);
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
