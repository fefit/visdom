use super::pattern::{self, exec, to_pattern, BoxDynPattern, MatchedQueue, Pattern};
use crate::mesdoc::utils::vec_char_to_clean_str;
use crate::mesdoc::{
	constants::PRIORITY_PSEUDO_SELECTOR,
	interface::{Elements, IElementTrait},
};
use lazy_static::lazy_static;
use std::fmt;
use std::sync::{Arc, Mutex};
lazy_static! {
	pub static ref RULES: Mutex<Vec<(&'static str, Arc<Rule>)>> = Mutex::new(Vec::with_capacity(20));
}
// matcher handles
pub type MatchAllHandle = Box<dyn (for<'a, 'r> Fn(&'a Elements<'r>, Option<bool>) -> Elements<'r>)>;
pub type MatchOneHandle = Box<dyn (Fn(&dyn IElementTrait, Option<bool>) -> bool)>;
pub type MatchSpecifiedHandle =
	Box<dyn for<'a> Fn(&'a dyn IElementTrait, Box<dyn FnMut(&dyn IElementTrait, bool, bool) + 'a>)>;
// matcher factory
pub type MatcherFactory = Box<dyn (Fn(MatchedQueue) -> Matcher) + Send + Sync>;

#[derive(Default)]
pub struct Matcher {
	pub all_handle: Option<MatchAllHandle>,
	pub one_handle: Option<MatchOneHandle>,
	pub specified_handle: Option<MatchSpecifiedHandle>,
	pub priority: u32,
	pub in_cache: bool,
}

impl fmt::Debug for Matcher {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(
			format!(
				"Matcher{{ all_handle: {}, one_handle: {}, specified_handle: {} }}",
				self.all_handle.is_some(),
				self.one_handle.is_some(),
				self.specified_handle.is_some(),
			)
			.as_str(),
		)
	}
}

impl Matcher {
	// apply all elements
	pub fn apply<'a, 'r>(&self, eles: &'a Elements<'r>, use_cache: Option<bool>) -> Elements<'r> {
		if let Some(handle) = &self.all_handle {
			return handle(eles, use_cache);
		}
		let handle = self.one_handle.as_ref().unwrap();
		let mut result = Elements::with_capacity(5);
		for ele in eles.get_ref() {
			if handle(&**ele, use_cache) {
				result.push(ele.cloned());
			}
		}
		result
	}

	// get all handle
	pub fn get_all_handle(&self) -> &MatchAllHandle {
		self.all_handle.as_ref().expect("All handle is None")
	}
}

// get char vec
const DEF_SIZE: usize = 2;
fn get_char_vec() -> Vec<char> {
	Vec::with_capacity(DEF_SIZE)
}

// unmatched start or end
fn panic_unmatched(ch: char, index: usize) -> ! {
	panic!(
		"Unmatched '{ch}' at index {index},you can escape it using both {ch}{ch}",
		ch = ch,
		index = index
	)
}

struct MatchedStore {
	hashs_num: usize,
	is_wait_end: bool,
	is_in_matched: bool,
	raw_params: Vec<char>,
	suf_params: Vec<char>,
	names: Vec<char>,
}

impl Default for MatchedStore {
	fn default() -> Self {
		MatchedStore {
			hashs_num: 0,
			is_wait_end: false,
			is_in_matched: false,
			raw_params: get_char_vec(),
			suf_params: get_char_vec(),
			names: get_char_vec(),
		}
	}
}
impl MatchedStore {
	fn next(&mut self) -> Result<Box<dyn Pattern>, String> {
		self.hashs_num = 0;
		self.is_in_matched = false;
		self.is_wait_end = false;
		let name = vec_char_to_clean_str(&mut self.names);
		let s = vec_char_to_clean_str(&mut self.suf_params);
		let r = vec_char_to_clean_str(&mut self.raw_params);
		to_pattern(&name, &s, &r)
	}
}

pub struct Rule {
	pub in_cache: bool,
	pub priority: u32,
	pub(crate) queues: Vec<Box<dyn Pattern>>,
	pub handle: MatcherFactory,
}

impl fmt::Debug for Rule {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(format!("Rule{{ queues: {:?} }}", self.queues).as_str())
	}
}

// Rule methods
impl Rule {
	// translate string to queues
	pub(crate) fn get_queues(content: &str) -> Vec<Box<dyn Pattern>> {
		const ANCHOR_CHAR: char = '\0';
		const START_CHAR: char = '{';
		const END_CHAR: char = '}';
		let mut prev_char = ANCHOR_CHAR;
		let mut store: MatchedStore = Default::default();
		let mut raw_chars = get_char_vec();
		let mut queues: Vec<Box<dyn Pattern>> = Vec::with_capacity(DEF_SIZE);
		let mut is_matched_finish = false;
		let mut index: usize = 0;
		for ch in content.chars() {
			index += 1;
			let is_prev_matched_finish = if is_matched_finish {
				is_matched_finish = false;
				true
			} else {
				false
			};
			if store.is_wait_end {
				if ch.is_ascii_whitespace() {
					continue;
				}
				if ch == END_CHAR {
					is_matched_finish = true;
				} else {
					panic!(
						"Unexpected end of the pattern '{}' at index {}, expect '{}' but found '{}'",
						vec_char_to_clean_str(&mut store.names),
						index - 1,
						END_CHAR,
						ch
					);
				}
			} else if !store.is_in_matched {
				// when not in matched
				if prev_char == START_CHAR {
					// translate '{'
					if ch == START_CHAR {
						prev_char = ANCHOR_CHAR;
						continue;
					} else {
						store.names.push(ch);
						store.is_in_matched = true;
						// remove the '{'
						raw_chars.pop();
						if !raw_chars.is_empty() {
							queues.push(Box::new(raw_chars.clone()));
							raw_chars.clear();
						}
					}
				} else if prev_char == END_CHAR {
					if is_prev_matched_finish {
						// is just end of the Pattern type.
						raw_chars.push(ch);
					} else if ch == END_CHAR {
						// translate end char '}'
						prev_char = ANCHOR_CHAR;
						continue;
					} else {
						// panic no matched
						panic_unmatched(END_CHAR, index - 2);
					}
				} else {
					raw_chars.push(ch);
				}
			} else if !store.raw_params.is_empty() {
				// in Pattern's raw params ##gfh#def##
				if ch == '#' {
					let leave_count = store.hashs_num - 1;
					if leave_count == 0 {
						// only one hash
						store.is_wait_end = true;
					} else {
						let raw_len = store.raw_params.len();
						let last_index = raw_len - leave_count;
						if last_index > 0 {
							store.is_wait_end = store.raw_params[last_index..]
								.iter()
								.filter(|&&ch| ch == '#')
								.count() == leave_count;
							if store.is_wait_end {
								store.raw_params.truncate(last_index);
							}
						}
					}
					if !store.is_wait_end {
						store.raw_params.push(ch);
					}
				} else {
					// in raw params
					store.raw_params.push(ch);
				}
			} else {
				// in suf_params or names
				if ch == '}' {
					if store.hashs_num > 0 {
						panic!("Uncomplete raw params: ''");
					}
					is_matched_finish = true;
				} else if ch == '#' {
					// in hashs
					store.hashs_num += 1;
				} else {
					// in names or suf_params
					if prev_char == '#' {
						// in raw params now
						store.raw_params.push(ch);
					} else {
						// check if in suf_params, or character is not a name's character
						if !store.suf_params.is_empty() || !(ch.is_ascii_alphanumeric() || ch == '_') {
							store.suf_params.push(ch);
						} else {
							store.names.push(ch);
						}
					}
				}
			}
			if is_matched_finish {
				match store.next() {
					Ok(queue) => queues.push(queue),
					Err(reason) => panic!("{}", reason),
				};
			}
			prev_char = ch;
		}
		// not end
		if store.is_wait_end || store.is_in_matched {
			panic!(
				"The Mathed type '{}' is not complete",
				store.names.iter().collect::<String>()
			);
		}
		if prev_char == START_CHAR || (prev_char == END_CHAR && !is_matched_finish) {
			panic_unmatched(prev_char, index - 1);
		}
		if !raw_chars.is_empty() {
			if raw_chars.len() == 1 {
				queues.push(Box::new(raw_chars[0]));
			} else {
				queues.push(Box::new(raw_chars));
			}
		}
		queues
	}

	pub fn exec(&self, chars: &[char]) -> Option<(MatchedQueue, usize, usize)> {
		Rule::exec_queues(&self.queues, chars)
	}

	pub fn exec_queues(
		queues: &[BoxDynPattern],
		chars: &[char],
	) -> Option<(MatchedQueue, usize, usize)> {
		let (result, matched_len, matched_queue_item, _) = exec(queues, chars);
		if matched_len > 0 {
			Some((result, matched_len, matched_queue_item))
		} else {
			None
		}
	}
	/// make a matcher
	pub fn make(&self, data: MatchedQueue) -> Matcher {
		let handle = &self.handle;
		let mut matcher = handle(data);
		matcher.priority = self.priority;
		matcher.in_cache = self.in_cache;
		matcher
	}

	/// make a matcher by alias
	pub fn make_alias(selector: &'static str) -> Matcher {
		// if parse the selector string into Selector and save to the closure
		// the mutex rules will trigger a dead lock
		// so there give up, just lost some performance
		Matcher {
			all_handle: Some(Box::new(move |eles: &Elements, _| eles.filter(selector))),
			// priority
			priority: PRIORITY_PSEUDO_SELECTOR,
			..Default::default()
		}
	}

	/// add a rule
	pub fn add(context: &str, mut rule: Rule) -> Self {
		rule.queues = Rule::get_queues(context);
		rule
	}
}

pub struct RuleDefItem(
	pub &'static str,
	pub &'static str,
	pub u32,
	pub MatcherFactory,
);

pub struct RuleItem {
	pub rule: Rule,
	pub context: &'static str,
	pub name: &'static str,
}

impl From<RuleDefItem> for RuleItem {
	fn from(item: RuleDefItem) -> Self {
		RuleItem {
			name: item.0,
			context: item.1,
			rule: Rule {
				priority: item.2,
				in_cache: false,
				handle: item.3,
				queues: Vec::new(),
			},
		}
	}
}

pub fn add_rules(rules: Vec<RuleItem>) {
	let mut all_rules = RULES.lock().unwrap();
	for RuleItem {
		name,
		context,
		rule,
	} in rules
	{
		let cur_rule = Rule::add(context, rule);
		all_rules.push((name, Arc::new(cur_rule)));
	}
}

pub(crate) fn init() {
	pattern::init();
}

#[cfg(test)]
mod tests {
	use super::{Matcher, Rule};
	#[test]
	fn test_allow_debug() {
		let matcher: Matcher = Default::default();
		assert!(format!("{:?}", matcher).contains("Matcher"));
		let rule: Rule = Rule {
			in_cache: false,
			priority: 1,
			queues: vec![],
			handle: Box::new(|_| Default::default()),
		};
		assert!(format!("{:?}", rule).contains("Rule"));
	}

	#[test]
	fn test_rule_escape_start() {
		let _ = Rule::get_queues("{{nth");
	}

	#[test]
	fn test_rule_escape_end() {
		let _ = Rule::get_queues("nth}}");
	}

	#[test]
	#[should_panic]
	fn test_rule_escape_end_repeat() {
		let _ = Rule::get_queues("nth}}}");
	}

	#[test]
	fn test_rule_escape_both() {
		let _ = Rule::get_queues("{{nth}}");
	}

	#[test]
	#[should_panic]
	fn test_rule_escape_no_start() {
		let _ = Rule::get_queues("{{nth}");
	}

	#[test]
	#[should_panic]
	fn test_rule_not_end_1() {
		let _ = Rule::get_queues("{nth");
	}

	#[test]
	#[should_panic]
	fn test_rule_not_end_2() {
		let _ = Rule::get_queues("{nth#");
	}

	#[test]
	#[should_panic]
	fn test_rule_not_end_3() {
		let _ = Rule::get_queues("{nth#a#");
	}

	#[test]
	#[should_panic]
	fn test_rule_not_end_4() {
		let _ = Rule::get_queues("{nth#a##}");
	}

	#[test]
	#[should_panic]
	fn test_wrong_escape_no_end_at_end() {
		let _ = Rule::get_queues("{nth}{");
	}

	#[test]
	#[should_panic]
	fn test_rule_params_not_end() {
		// panic because no pattern register
		let _ = Rule::get_queues("{abc!#abc}");
	}

	#[test]
	#[should_panic]
	fn test_rule_params() {
		// panic because no pattern register
		let _ = Rule::get_queues("{abc!#abc#}");
	}

	#[test]
	#[should_panic]
	fn test_rule_wrong_params() {
		// panic because the params not end correctly
		let _ = Rule::get_queues("{abc!#a#bc#}");
	}

	#[test]
	#[should_panic]
	fn test_rule_params_hash() {
		// panic because no pattern register
		let _ = Rule::get_queues("{abc!##a#bc##}");
	}
}
