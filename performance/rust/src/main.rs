use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;
use visdom::Vis;
type GenResult<T> = Result<T, Box<dyn Error>>;
type UniResult = GenResult<()>;
type TotalInfo = (&'static str, String);
type RunResult = GenResult<TotalInfo>;
const LOOPTIMES: u32 = 200;
const NODECOUNT: usize = 3000;

fn get_file_content(cur_file: &str) -> GenResult<String> {
	let mut file = File::open(cur_file)?;
	let mut content = String::new();
	file.read_to_string(&mut content)?;
	Ok(content)
}

fn load_html() -> RunResult {
	let content = get_file_content("../data/index.html")?;
	let start_time = SystemTime::now();
	for _ in 0..LOOPTIMES {
		Vis::load_options(&content, Default::default())?;
	}
	let elapsed = start_time.elapsed()?;
	Ok(("", format!("{:?}", elapsed / LOOPTIMES)))
}

fn exec_times_avg<F>(cb: F) -> String
where
	F: Fn(),
{
	let start_time = SystemTime::now();
	for _ in 0..LOOPTIMES {
		cb();
	}
	let elapsed = start_time.elapsed().unwrap();
	format!("{:?}", elapsed / LOOPTIMES)
}

fn find_id() -> RunResult {
	let html: String = format!(
		r##"
	    <ul>
	      {}{}
	    </ul>
	  "##,
		String::from("<li></li>").repeat(NODECOUNT),
		"<li id='target'></li>"
	);
	const SELECTOR: &str = "#target";
	let root = Vis::load(&html)?;
	let ul = root.children("ul");
	println!("Finded: {:?}", ul.find(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		ul.find(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn find_class() -> RunResult {
	let html: String = format!(
		r##"
	    <ul>
	      {}{}
	    </ul>
	  "##,
		String::from("<li></li>").repeat(NODECOUNT),
		"<li class='target'></li>"
	);
	const SELECTOR: &str = ".target";
	let root = Vis::load(&html)?;
	let ul = root.children("ul");
	println!("Finded: {:?}", ul.find(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		ul.find(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn find_name() -> RunResult {
	let html: String = format!(
		r##"
	    <dl>{}</dl>
	  "##,
		String::from("<dt></dt><dd></dd>").repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = "dt";
	let root = Vis::load(&html)?;
	let dl = root.children("dl");
	println!("Finded: {:?}", dl.find(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		dl.find(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn find_attr() -> RunResult {
	let html: String = format!(
		r##"
	    <dl>{}</dl>
	  "##,
		String::from("<dt></dt><dd contenteditable></dd>").repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = "[contenteditable]";
	let root = Vis::load(&html)?;
	let dl = root.children("dl");
	println!("Finded: {:?}", dl.find(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		dl.find(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn find_prev() -> RunResult {
	let html: String = format!(
		r##"
	    <dl>{}</dl>
	  "##,
		String::from("<dt></dt><dd></dd>").repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = "dd";
	let root = Vis::load(&html)?;
	let dt = root.children("dl dt");
	println!("Finded: {:?}", dt.prev(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		dt.prev(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn find_prev_all() -> RunResult {
	let html: String = format!(
		r##"
	    <dl>{}</dl>
	  "##,
		String::from("<dt></dt><dd></dd>").repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = "dd";
	let root = Vis::load(&html)?;
	let dt = root.children("dl dt");
	println!("Finded: {:?}", dt.prev_all(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		dt.prev_all(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn find_next() -> RunResult {
	let html: String = format!(
		r##"
	    <dl>{}</dl>
	  "##,
		String::from("<dt></dt><dd></dd>").repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = "dd";
	let root = Vis::load(&html)?;
	let dt = root.children("dl dt");
	println!("Finded: {:?}", dt.next(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		dt.next(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn find_next_all() -> RunResult {
	let html: String = format!(
		r##"
	    <dl>{}</dl>
	  "##,
		String::from("<dt></dt><dd></dd>").repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = "dd";
	let root = Vis::load(&html)?;
	let dt = root.children("dl dt");
	println!("Finded: {:?}", dt.next_all(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		dt.next_all(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn empty() -> RunResult {
	let html: String = format!(
		r##"
	    <ul>{}</ul>
	  "##,
		String::from("<li></li><li>a</li>").repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = ":empty";
	let root = Vis::load(&html)?;
	let ul = root.children("ul");
	println!("Finded: {:?}", ul.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		ul.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn contains() -> RunResult {
	let html: String = format!(
		r##"
	    <ul>{}</ul>
	  "##,
		String::from("<li></li><li>abcdefghijklmnopqrstuvwxyz&amp;abcdefghijklmnopqrstuvwxy</li>")
			.repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = ":contains('z&a')";
	let root = Vis::load(&html)?;
	let ul = root.children("ul");
	println!("Finded: {:?}", ul.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		ul.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn first_child() -> RunResult {
	let html: String = format!(
		r##"
	    <ul>{}</ul>
	  "##,
		String::from("<li></li>").repeat(NODECOUNT)
	);
	const SELECTOR: &str = ":first-child";
	let root = Vis::load(&html)?;
	let ul = root.children("ul");
	println!("Finded: {:?}", ul.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		ul.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn last_child() -> RunResult {
	let html: String = format!(
		r##"
	    <ul>
	      {}
	    </ul>
	  "##,
		String::from("<li></li>").repeat(NODECOUNT)
	);
	const SELECTOR: &str = ":last-child";
	let root = Vis::load(&html)?;
	let ul = root.children("ul");
	println!("Finded: {:?}", ul.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		ul.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn first_of_type() -> RunResult {
	let html: String = format!(
		r##"
	    <dl>{}</dl>
	  "##,
		String::from("<dt></dt><dd></dd>").repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = ":first-of-type";
	let root = Vis::load(&html)?;
	let dl = root.children("dl");
	println!("Finded: {:?}", dl.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		dl.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn last_of_type() -> RunResult {
	let html: String = format!(
		r##"
	    <dl>{}</dl>
	  "##,
		String::from("<dt></dt><dd></dd>").repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = ":last-of-type";
	let root = Vis::load(&html)?;
	let dl = root.children("dl");
	println!("Finded: {:?}", dl.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		dl.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn nth_child() -> RunResult {
	let html: String = format!(
		r##"
	    <ul>
	      {}
	    </ul>
	  "##,
		String::from("<li></li>").repeat(NODECOUNT)
	);
	const SELECTOR: &str = ":nth-child(2n),:nth-child(3n),:nth-child(5n)";
	let root = Vis::load(&html)?;
	let ul = root.children("ul");
	println!("Finded: {:?}", ul.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		ul.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn nth_child_10() -> RunResult {
	let html: String = format!(
		r##"
	    <ul>{}</ul>
	  "##,
		String::from("<li></li>").repeat(NODECOUNT)
	);
	const SELECTOR: &str = ":nth-child(10)";
	let root = Vis::load(&html)?;
	let ul = root.children("ul");
	println!("Finded: {:?}", ul.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		ul.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn nth_child_2n5() -> RunResult {
	let html: String = format!(
		r##"
	    <ul>{}</ul>
	  "##,
		String::from("<li></li>").repeat(NODECOUNT)
	);
	const SELECTOR: &str = ":nth-child(2n + 5)";
	let root = Vis::load(&html)?;
	let ul = root.children("ul");
	println!("Finded: {:?}", ul.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		ul.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn nth_last_child() -> RunResult {
	let html: String = format!(
		r##"
	    <ul>
	      {}
	    </ul>
	  "##,
		String::from("<li></li>").repeat(NODECOUNT)
	);
	const SELECTOR: &str = ":nth-last-child(2n),:nth-last-child(3n),:nth-last-child(5n)";
	let root = Vis::load(&html)?;
	let ul = root.children("ul");
	println!("Finded: {:?}", ul.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		ul.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn nth_last_child_10() -> RunResult {
	let html: String = format!(
		r##"
	    <ul>
	      {}
	    </ul>
	  "##,
		String::from("<li></li>").repeat(NODECOUNT)
	);
	const SELECTOR: &str = ":nth-last-child(10)";
	let root = Vis::load(&html)?;
	let ul = root.children("ul");
	println!("Finded: {:?}", ul.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		ul.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn nth_last_child_2n5() -> RunResult {
	let html: String = format!(
		r##"
	    <ul>
	      {}
	    </ul>
	  "##,
		String::from("<li></li>").repeat(NODECOUNT)
	);
	const SELECTOR: &str = ":nth-last-child(2n + 5)";
	let root = Vis::load(&html)?;
	let ul = root.children("ul");
	println!("Finded: {:?}", ul.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		ul.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn nth_of_type() -> RunResult {
	let html: String = format!(
		r##"
	    <dl>{}</dl>
	  "##,
		String::from("<dt></dt><dd></dd>").repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = ":nth-of-type(2n),:nth-of-type(3n)";
	let root = Vis::load(&html)?;
	let dl = root.children("dl");
	println!("Finded: {:?}", dl.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		dl.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn nth_of_type_10() -> RunResult {
	let html: String = format!(
		r##"
	    <dl>{}</dl>
	  "##,
		String::from("<dt></dt><dd></dd>").repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = ":nth-of-type(10)";
	let root = Vis::load(&html)?;
	let dl = root.children("dl");
	println!("Finded: {:?}", dl.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		dl.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn nth_of_type_2n5() -> RunResult {
	let html: String = format!(
		r##"
	    <dl>{}</dl>
	  "##,
		String::from("<dt></dt><dd></dd>").repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = ":nth-of-type(2n+5)";
	let root = Vis::load(&html)?;
	let dl = root.children("dl");
	println!("Finded: {:?}", dl.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		dl.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn nth_last_of_type() -> RunResult {
	let html: String = format!(
		r##"
	    <dl>{}</dl>
	  "##,
		String::from("<dt></dt><dd></dd>").repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = ":nth-last-of-type(2n),:nth-last-of-type(3n)";
	let root = Vis::load(&html)?;
	let dl = root.children("dl");
	println!("Finded: {:?}", dl.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		dl.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn nth_last_of_type_10() -> RunResult {
	let html: String = format!(
		r##"
	    <dl>{}</dl>
	  "##,
		String::from("<dt></dt><dd></dd>").repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = ":nth-last-of-type(10)";
	let root = Vis::load(&html)?;
	let dl = root.children("dl");
	println!("Finded: {:?}", dl.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		dl.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn nth_last_of_type_2n5() -> RunResult {
	let html: String = format!(
		r##"
	    <dl>{}</dl>
	  "##,
		String::from("<dt></dt><dd></dd>").repeat(NODECOUNT / 2)
	);
	const SELECTOR: &str = ":nth-last-of-type(2n+5)";
	let root = Vis::load(&html)?;
	let dl = root.children("dl");
	println!("Finded: {:?}", dl.children(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		dl.children(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn nth_child_find() -> RunResult {
	let html: String = format!(
		r##"
	    <ul>
	      {}
	    </ul>
	  "##,
		String::from("<li></li>").repeat(NODECOUNT)
	);
	const SELECTOR: &str = ":nth-child(2n),:nth-child(3n),:nth-child(5n)";
	let root = Vis::load(&html)?;
	let ul = root.children("ul");
	println!("Finded: {:?}", ul.find(SELECTOR).length());
	let used_time = exec_times_avg(|| {
		ul.find(SELECTOR);
	});
	Ok((SELECTOR, used_time))
}

fn main() -> UniResult {
	let mut total_info: Vec<TotalInfo> = Vec::with_capacity(10);
	total_info.push(load_html()?);
	total_info.push(find_id()?);
	total_info.push(find_class()?);
	total_info.push(find_name()?);
	total_info.push(find_attr()?);
	total_info.push(find_prev()?);
	total_info.push(find_prev_all()?);
	total_info.push(find_next()?);
	total_info.push(find_next_all()?);
	total_info.push(empty()?);
	total_info.push(contains()?);
	total_info.push(first_child()?);
	total_info.push(last_child()?);
	total_info.push(first_of_type()?);
	total_info.push(last_of_type()?);
	total_info.push(nth_child()?);
	total_info.push(nth_child_10()?);
	total_info.push(nth_child_2n5()?);
	total_info.push(nth_last_child()?);
	total_info.push(nth_last_child_10()?);
	total_info.push(nth_last_child_2n5()?);
	total_info.push(nth_of_type()?);
	total_info.push(nth_of_type_10()?);
	total_info.push(nth_of_type_2n5()?);
	total_info.push(nth_last_of_type()?);
	total_info.push(nth_last_of_type_10()?);
	total_info.push(nth_last_of_type_2n5()?);
	total_info.push(nth_child_find()?);
	println!("Total info: {:?}", total_info);
	Ok(())
}
