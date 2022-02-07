use visdom::{types::BoxDynError, Vis};

#[test]
#[should_panic]
fn test_wrong_html() {
	// unmatched tag
	let html = r#"
  <!doctype html>  
  <html>
    <head></head>
    <aa></a>
  </html>
  "#;
	let _ = Vis::load_options(html, Default::default()).unwrap();
}

#[test]
fn test_wrong_html_catch() {
	// unmatched tag
	let html = r#"
  <!doctype html>  
  <html>
    <head></head>
    <aa></a>
  </html>
  "#;
	let _ = Vis::load_options_catch(
		html,
		Default::default(),
		Box::new(|_| {
			// ignore the error, or write a log
		}),
	);
}
#[test]
#[should_panic]
fn test_wrong_selector() {
	let html = r#"
  <!doctype html>  
  <html>
    <head></head>
    <a></a>
  </html>
  "#;
	let root = Vis::load_options_catch(
		html,
		Default::default(),
		Box::new(|e: BoxDynError| {
			// errors will catched
			panic!("{:?}", e.to_string());
		}),
	);
	// no pseudo selector ":all-child"
	let _ = root.find("a:all-childs");
}

#[test]
fn test_wrong_selector_catch() {
	let html = r#"
  <!doctype html>  
  <html>
    <head></head>
    <a></a>
  </html>
  "#;
	let root = Vis::load_catch(
		html,
		Box::new(|_: BoxDynError| {
			// errors will be catched here
			// error selector just get an empty elements
		}),
	);
	// no pseudo selector ":all-child"
	let _ = root.find("a:all-childs");
}
