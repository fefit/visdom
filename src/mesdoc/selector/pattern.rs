use crate::mesdoc::utils::{divide_isize, is_char_available_in_key, RoundType};
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, fmt::Debug, usize};

pub type FromParamsFn = Box<dyn Fn(&str, &str) -> Result<BoxDynPattern, String> + Send + 'static>;
lazy_static! {
	static ref REGEXS: Mutex<HashMap<String, Arc<Regex>>> = Mutex::new(HashMap::new());
	static ref PATTERNS: Mutex<HashMap<&'static str, FromParamsFn>> = Mutex::new(HashMap::new());
}

pub type BoxDynPattern = Box<dyn Pattern>;

fn no_implemented(name: &str) -> ! {
	panic!("No supported pattern '{}' was found", name);
}

pub type MatchedData = HashMap<String, String>;
pub type MatchedQueue = Vec<Matched>;
#[derive(Debug, Default, Clone)]
pub struct Matched {
	pub chars: Vec<char>,
	pub ignore_chars: Option<usize>,
	pub name: &'static str,
	pub data: MatchedData,
}

pub trait Pattern: Send + Sync + Debug {
	fn matched(&self, chars: &[char]) -> Option<Matched>;
	// check if nested pattern
	fn is_nested(&self) -> bool {
		false
	}
	// get a pattern trait object
	fn from_params(s: &str, _p: &str) -> Result<BoxDynPattern, String>
	where
		Self: Sized + Send + 'static,
	{
		no_implemented(s);
	}
}

impl Pattern for char {
	fn matched(&self, chars: &[char]) -> Option<Matched> {
		let ch = chars[0];
		if *self == ch {
			return Some(Matched {
				chars: vec![ch],
				..Default::default()
			});
		}
		None
	}
}

impl Pattern for &[char] {
	fn matched(&self, chars: &[char]) -> Option<Matched> {
		let total = self.len();
		if total > chars.len() {
			return None;
		}
		let mut result: Vec<char> = Vec::with_capacity(total);
		for (index, &ch) in self.iter().enumerate() {
			let cur = chars
				.get(index)
				.expect("Pattern for slice char's length must great than target's chars.z");
			if ch == *cur {
				result.push(ch);
			} else {
				return None;
			}
		}
		Some(Matched {
			chars: result,
			..Default::default()
		})
	}
}

impl Pattern for Vec<char> {
	fn matched(&self, chars: &[char]) -> Option<Matched> {
		self.as_slice().matched(chars)
	}
}

/// Identity
#[derive(Debug, Default)]
pub struct Identity;

impl Pattern for Identity {
	fn matched(&self, chars: &[char]) -> Option<Matched> {
		let mut result: Vec<char> = Vec::with_capacity(5);
		let first = chars[0];
		let name: &str = "identity";
		if !(first.is_ascii_alphabetic() || first == '_') {
			return None;
		}
		// allow translate character '\': fix issue #2
		let mut is_in_translate = false;
		let mut ignore_chars = 0;
		for &c in chars {
			if !is_in_translate {
				if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
					result.push(c);
				} else if c == '\\' {
					is_in_translate = true;
					ignore_chars += 1;
				} else {
					break;
				}
			} else {
				result.push(c);
				is_in_translate = false;
			}
		}
		let ignore_chars = if ignore_chars > 0 {
			Some(ignore_chars)
		} else {
			None
		};
		Some(Matched {
			chars: result,
			name,
			ignore_chars,
			..Default::default()
		})
	}
	// from_str
	fn from_params(s: &str, p: &str) -> Result<BoxDynPattern, String> {
		check_params_return(&[s, p], || Box::new(Identity::default()))
	}
}
/// AttrKey
#[derive(Debug, Default)]
pub struct AttrKey;

impl Pattern for AttrKey {
	fn matched(&self, chars: &[char]) -> Option<Matched> {
		let mut result = Vec::with_capacity(5);
		for ch in chars {
			if is_char_available_in_key(ch) {
				result.push(*ch);
			} else {
				break;
			}
		}
		if !result.is_empty() {
			return Some(Matched {
				chars: result,
				name: "attr_key",
				..Default::default()
			});
		}
		None
	}
	// from_params
	fn from_params(s: &str, p: &str) -> Result<BoxDynPattern, String> {
		check_params_return(&[s, p], || Box::new(AttrKey::default()))
	}
}
/// Spaces
#[derive(Debug, Default)]
pub struct Spaces;

impl Pattern for Spaces {
	fn matched(&self, chars: &[char]) -> Option<Matched> {
		let mut result: Vec<char> = Vec::with_capacity(2);
		for ch in chars {
			if ch.is_ascii_whitespace() {
				result.push(*ch);
			} else {
				break;
			}
		}
		Some(Matched {
			chars: result,
			name: "spaces",
			..Default::default()
		})
	}
	// from params
	fn from_params(s: &str, p: &str) -> Result<BoxDynPattern, String> {
		check_params_return(&[s, p], || Box::new(Spaces::default()))
	}
}

/// `Nth`
/// 2n/+2n+1/2n-1/-2n+1/+0/-1/2
#[derive(Debug, Default)]
pub struct Nth;

impl Pattern for Nth {
	fn matched(&self, chars: &[char]) -> Option<Matched> {
		let rule: RegExp = RegExp {
			context: Cow::from(
				r#"^(?:([-+])?([1-9]\d+|[0-9])?n(?:\s*([+-])\s*([1-9]\d+|[0-9]))?|([-+])?([1-9]\d+|[0-9]))"#,
			),
		};
		let mut data = HashMap::with_capacity(2);
		let mut matched_chars: Vec<char> = Vec::new();
		if let Some(v) = Pattern::matched(&rule, chars) {
			let rule_data = v.data;
			// when the group index 6,
			let only_index = rule_data.get("6").is_some();
			let index_keys = if only_index { ("6", "5") } else { ("4", "3") };
			// set index
			if let Some(index) = Nth::get_number(&rule_data, index_keys, None) {
				data.insert("index".to_string(), index);
			}
			// also has `n`
			if !only_index {
				if let Some(n) = Nth::get_number(&rule_data, ("2", "1"), Some("1".to_string())) {
					data.insert("n".to_string(), n);
				}
			}
			matched_chars = v.chars;
		} else {
			// maybe 'even' or 'odd'
			let even = vec!['e', 'v', 'e', 'n'];
			let odd = vec!['o', 'd', 'd'];
			if Pattern::matched(&even, chars).is_some() {
				data.insert("n".to_string(), "2".to_string());
				data.insert("index".to_string(), "0".to_string());
				matched_chars = even;
			} else if Pattern::matched(&odd, chars).is_some() {
				data.insert("n".to_string(), "2".to_string());
				data.insert("index".to_string(), "1".to_string());
				matched_chars = odd;
			}
		}
		if !data.is_empty() {
			return Some(Matched {
				name: "nth",
				data,
				chars: matched_chars,
				ignore_chars: None,
			});
		}
		None
	}
	// from params to pattern
	fn from_params(s: &str, p: &str) -> Result<BoxDynPattern, String> {
		check_params_return(&[s, p], || Box::new(Nth::default()))
	}
}

impl Nth {
	fn get_number(data: &MatchedData, keys: (&str, &str), def: Option<String>) -> Option<String> {
		const MINUS: &str = "-";
		if let Some(idx) = data.get(keys.0).or(def.as_ref()) {
			let mut index = idx.to_owned();
			if let Some(op) = data.get(keys.1) {
				if op == MINUS {
					index = String::from(op) + &index;
				}
			}
			return Some(index);
		}
		None
	}
	// get indexs allowed
	pub fn get_allowed_indexs(
		n: &Option<String>,
		index: &Option<String>,
		total: usize,
	) -> Vec<usize> {
		// has n
		if let Some(n) = n {
			let n = n.parse::<isize>().unwrap();
			let index = index
				.as_ref()
				.map(|index| index.parse::<isize>().unwrap())
				.unwrap_or(0);
			// n == 0
			if n == 0 {
				if index > 0 {
					let index = index as usize;
					if index <= total {
						return vec![index - 1];
					}
				}
				return vec![];
			}
			// n < 0 or n > 0
			let mut start_loop: isize;
			let end_loop: isize;
			if n < 0 {
				// -2n - 1/ -2n + 0
				if index <= 0 {
					return vec![];
				}
				// -2n + 1
				if index <= -n {
					let index = index as usize;
					if index <= total {
						return vec![index - 1];
					}
					return vec![];
				}
				start_loop = divide_isize(index - (total as isize), -n, RoundType::Ceil);
				end_loop = divide_isize(index - 1, -n, RoundType::Floor);
			} else {
				// n > 0
				start_loop = divide_isize(1 - index, n, RoundType::Ceil);
				end_loop = divide_isize((total as isize) - index, n, RoundType::Floor);
			}
			// set start_loop min 0
			if start_loop < 0 {
				start_loop = 0;
			}
			// when start_loop >= end_loop, no index is allowed
			if start_loop > end_loop {
				return vec![];
			}
			let start = start_loop as usize;
			let end = end_loop as usize;
			let mut allow_indexs = Vec::with_capacity((end - start + 1) as usize);
			for i in start..=end {
				let cur_index = (i as isize * n + index) as usize;
				if cur_index < 1 {
					continue;
				}
				// last index need -1 for real list index
				allow_indexs.push(cur_index - 1);
			}
			if n < 0 {
				allow_indexs.reverse();
			}
			return allow_indexs;
		}
		// only index
		let index = index
			.as_ref()
			.expect("Nth must have 'index' value when 'n' is not setted.")
			.parse::<isize>()
			.expect("Nth's index is not a correct number");
		if index <= 0 || index > (total as isize) {
			return vec![];
		}
		vec![(index - 1) as usize]
	}
}

/// RegExp
#[derive(Debug)]
pub struct RegExp<'a> {
	pub context: Cow<'a, str>,
}

impl<'a> Pattern for RegExp<'a> {
	/// impl `matched`
	fn matched(&self, chars: &[char]) -> Option<Matched> {
		let Self { context } = self;
		let content = chars.iter().collect::<String>();
		let rule = RegExp::get_rule(&context);
		if let Some(caps) = rule.captures(&content) {
			let total_len = caps[0].chars().count();
			let mut data = HashMap::with_capacity(caps.len() - 1);
			for (index, m) in caps.iter().skip(1).enumerate() {
				if let Some(m) = m {
					data.insert((index + 1).to_string(), m.as_str().to_string());
				}
			}
			let result = chars[..total_len].to_vec();
			return Some(Matched {
				chars: result,
				name: "regexp",
				data,
				ignore_chars: None,
			});
		}
		None
	}
	/// impl `from_params`
	fn from_params(s: &str, p: &str) -> Result<BoxDynPattern, String> {
		check_params_return(&[s], || {
			Box::new(RegExp {
				context: Cow::Owned(p.to_string()),
			})
		})
	}
}

impl<'a> RegExp<'a> {
	pub fn get_rule(context: &str) -> Arc<Regex> {
		let wrong_regex = format!("Wrong regex context '{}'", context);
		let last_context = String::from("^") + context;
		let mut regexs = REGEXS.lock().unwrap();
		if let Some(rule) = regexs.get(&last_context[..]) {
			Arc::clone(rule)
		} else {
			let key = last_context;
			let rule = Regex::new(&key).expect(&wrong_regex);
			let value = Arc::new(rule);
			let result = Arc::clone(&value);
			regexs.insert(key, value);
			result
		}
	}
}

/// Nested
#[derive(Debug, Default)]
pub struct NestedSelector;

impl Pattern for NestedSelector {
	fn matched(&self, _chars: &[char]) -> Option<Matched> {
		None
	}
	// from params to pattern
	fn from_params(s: &str, p: &str) -> Result<BoxDynPattern, String> {
		check_params_return(&[s, p], || Box::new(NestedSelector::default()))
	}
	// set to be nested
	fn is_nested(&self) -> bool {
		true
	}
}

pub fn add_pattern(name: &'static str, from_handle: FromParamsFn) {
	let mut patterns = PATTERNS.lock().unwrap();
	if patterns.get(name).is_some() {
		panic!("The pattern '{}' is already exist.", name);
	} else {
		patterns.insert(name, from_handle);
	}
}

pub(crate) fn init() {
	// add lib supported patterns
	add_pattern("identity", Box::new(Identity::from_params));
	add_pattern("spaces", Box::new(Spaces::from_params));
	add_pattern("attr_key", Box::new(AttrKey::from_params));
	add_pattern("nth", Box::new(Nth::from_params));
	add_pattern("regexp", Box::new(RegExp::from_params));
	add_pattern("selector", Box::new(NestedSelector::from_params));
}

pub fn to_pattern(name: &str, s: &str, p: &str) -> Result<BoxDynPattern, String> {
	let patterns = PATTERNS.lock().unwrap();
	if let Some(cb) = patterns.get(name) {
		return cb(s, p);
	}
	no_implemented(name);
}

pub fn exec(queues: &[BoxDynPattern], chars: &[char]) -> (MatchedQueue, usize, usize, bool) {
	let mut start_index = 0;
	let mut result: MatchedQueue = Vec::with_capacity(queues.len());
	let mut matched_num: usize = 0;
	for item in queues {
		if let Some(matched) = item.matched(&chars[start_index..]) {
			start_index += matched.chars.len() + matched.ignore_chars.unwrap_or(0);
			matched_num += 1;
			result.push(matched);
		} else {
			break;
		}
	}
	(result, start_index, matched_num, start_index == chars.len())
}

pub fn check_params_return<F: Fn() -> BoxDynPattern>(
	params: &[&str],
	cb: F,
) -> Result<BoxDynPattern, String> {
	for &p in params {
		if !p.is_empty() {
			let all_params = params.iter().fold(String::from(""), |mut r, &s| {
				r.push_str(s);
				r
			});
			return Err(format!("Unrecognized params '{}'", all_params));
		}
	}
	Ok(cb())
}

#[cfg(test)]
mod tests {
	use super::{
		add_pattern, check_params_return, AttrKey, BoxDynPattern, Matched, Nth, Pattern, RegExp,
	};
	#[test]
	fn test_allow_indexs() {
		assert_eq!(
			Nth::get_allowed_indexs(&Some("-2".to_string()), &Some("3".to_string()), 9),
			vec![0, 2]
		);
		assert_eq!(
			Nth::get_allowed_indexs(&Some("2".to_string()), &Some("3".to_string()), 9),
			vec![2, 4, 6, 8]
		);
		assert_eq!(
			Nth::get_allowed_indexs(&None, &Some("3".to_string()), 9),
			vec![2]
		);
		assert!(Nth::get_allowed_indexs(&None, &Some("3".to_string()), 2).is_empty());
		assert_eq!(
			Nth::get_allowed_indexs(&Some("0".to_string()), &Some("3".to_string()), 9),
			vec![2]
		);
		assert!(Nth::get_allowed_indexs(&Some("0".to_string()), &Some("-3".to_string()), 9).is_empty());
		assert!(Nth::get_allowed_indexs(&Some("1".to_string()), &Some("6".to_string()), 5).is_empty());
		assert_eq!(
			Nth::get_allowed_indexs(&Some("2".to_string()), &None, 9),
			vec![1, 3, 5, 7]
		);
		assert!(Nth::get_allowed_indexs(&Some("-2".to_string()), &None, 9).is_empty());
		assert!(Nth::get_allowed_indexs(&Some("-4".to_string()), &Some("3".to_string()), 2).is_empty());
	}

	#[test]
	fn test_check_params_return() {
		assert!(check_params_return(&["a"], || Box::new('c')).is_err());
		assert!(check_params_return(&["", "a"], || Box::new('c')).is_err());
		assert!(check_params_return(&["", ""], || Box::new('c')).is_ok());
	}
	#[test]
	#[should_panic]
	fn test_new_pattern() {
		#[derive(Debug)]
		struct TestPattern;
		impl Pattern for TestPattern {
			fn matched(&self, _: &[char]) -> Option<Matched> {
				None
			}
			fn from_params(s: &str, p: &str) -> Result<BoxDynPattern, String>
			where
				Self: Sized + Send + 'static,
			{
				check_params_return(&[s, p], || Box::new(TestPattern))
			}
		}
		let pat: Box<dyn Pattern> = Box::new(TestPattern);
		assert!(!pat.is_nested());
		assert!(pat.matched(&['a']).is_none());
		assert!(format!("{:?}", pat).contains("Pattern"));
		assert!(TestPattern::from_params("a", "").is_err());
		add_pattern("test", Box::new(TestPattern::from_params));
		add_pattern("test", Box::new(TestPattern::from_params));
	}

	#[test]
	#[should_panic]
	fn test_from_params() {
		let _ = char::from_params("", "");
	}

	#[test]
	fn test_pattern_matched() {
		let nth: BoxDynPattern = Box::new(Nth);
		assert!(nth.matched(&['-', 'a']).is_none());
		assert!(nth.matched(&['-', '1']).is_some());
		let part_matched = nth.matched(&['-', '2', 'n', '+', 'a']);
		assert!(part_matched.is_some());
		assert_eq!(part_matched.unwrap().chars, vec!['-', '2', 'n']);
		// attr key
		let attr_key: BoxDynPattern = Box::new(AttrKey);
		assert!(attr_key.matched(&[',']).is_none());
		assert!(attr_key.matched(&[' ']).is_none());
		assert!(attr_key.matched(&['\u{0000}']).is_none());
		// regexp
		let reg_exp: BoxDynPattern = Box::new(RegExp {
			context: std::borrow::Cow::from("abc"),
		});
		assert!(format!("{:?}", reg_exp).contains("abc"));
	}
}
