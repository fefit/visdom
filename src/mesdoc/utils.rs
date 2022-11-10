use std::cmp::Ordering;

pub fn vec_char_to_clean_str(v: &mut Vec<char>) -> String {
	v.drain(..).collect::<String>()
}

/**
 * non characters
 * https://infra.spec.whatwg.org/#noncharacter
*/
pub fn is_non_character(ch: &char) -> bool {
	matches!(
		ch,
		'\u{FDD0}'
			..='\u{FDEF}'
				| '\u{FFFE}'
				| '\u{FFFF}'
				| '\u{1FFFE}'
				| '\u{1FFFF}'
				| '\u{2FFFE}'
				| '\u{2FFFF}'
				| '\u{3FFFE}'
				| '\u{3FFFF}'
				| '\u{4FFFE}'
				| '\u{4FFFF}'
				| '\u{5FFFE}'
				| '\u{5FFFF}'
				| '\u{6FFFE}'
				| '\u{6FFFF}'
				| '\u{7FFFE}'
				| '\u{7FFFF}'
				| '\u{8FFFE}'
				| '\u{8FFFF}'
				| '\u{9FFFE}'
				| '\u{9FFFF}'
				| '\u{AFFFE}'
				| '\u{AFFFF}'
				| '\u{BFFFE}'
				| '\u{BFFFF}'
				| '\u{CFFFE}'
				| '\u{CFFFF}'
				| '\u{DFFFE}'
				| '\u{DFFFF}'
				| '\u{EFFFE}'
				| '\u{EFFFF}'
				| '\u{FFFFE}'
				| '\u{FFFFF}'
				| '\u{10FFFE}'
				| '\u{10FFFF}'
	)
}

/**
 *
 * https://www.w3.org/TR/2012/WD-html-markup-20120329/syntax.html#syntax-attributes
 * https://html.spec.whatwg.org/multipage/syntax.html#attributes-2
*/
pub fn is_char_available_in_key(ch: &char) -> bool {
	if ch.is_ascii_alphanumeric() || ['_', '-', '.', ':'].contains(ch) {
		return true;
	}
	if ch.is_ascii_whitespace()
		|| ch.is_ascii_control()
		|| is_non_character(ch)
		|| ch.is_ascii_punctuation()
		|| *ch == '\u{0000}'
	{
		return false;
	}
	true
}
#[allow(dead_code)]
pub enum RoundType {
	Floor,
	Ceil,
	Round,
}
pub fn divide_isize(a: isize, b: isize, round: RoundType) -> isize {
	// the rust divide method's behavior is same as 'rem'
	// the value is near 'zero'
	// so get the expression '(a/b) * b + (a % b) = a'
	//
	let mut res = a / b;
	let remainder = a % b;
	use RoundType::*;
	match round {
		Floor => {
			// -∞ -> 0
			if res < 0 && remainder != 0 {
				res -= 1;
			}
		}
		Ceil => {
			// 0 -> ∞
			if res > 0 && remainder != 0 {
				res += 1;
			}
		}
		Round => {
			if res != 0 && remainder != 0 {
				let symbol = if res < 0 { -1 } else { 1 };
				let total = remainder * 2 - symbol * b;
				// the result's symbol is same as dividend 'a'
				if (total >= 0 && a > 0) || (total <= 0 && a < 0) {
					res += symbol;
				}
			}
		}
	}
	res
}

pub fn retain_by_index<T>(v: &mut Vec<T>, indexs: &[usize]) {
	for (i, index) in indexs.iter().enumerate() {
		v.remove(index - i);
	}
	/*
	let mut loop_index: usize = 0;
	v.retain(|_| {
		let removed = indexs.contains(&loop_index);
		loop_index += 1;
		!removed
	});
	*/
}

// get a class list from class attribute
pub fn get_class_list(attr_class: &str) -> Vec<Vec<char>> {
	let mut class_list: Vec<Vec<char>> = Vec::with_capacity(2);
	let mut class_name: Vec<char> = Vec::with_capacity(5);
	let mut prev_is_whitespace = true;
	for ch in attr_class.chars() {
		if ch.is_ascii_whitespace() {
			if prev_is_whitespace {
				continue;
			}
			// end of class name
			prev_is_whitespace = true;
			class_list.push(class_name);
			class_name = Vec::with_capacity(5);
		} else {
			// in class name
			prev_is_whitespace = false;
			class_name.push(ch);
		}
	}
	// last class name
	if !prev_is_whitespace {
		class_list.push(class_name);
	}
	class_list
}

// get a string from class list
pub fn class_list_to_string(class_list: &[Vec<char>]) -> String {
	let total = class_list.len();
	match total.cmp(&1) {
		Ordering::Equal => class_list[0].iter().collect::<String>(),
		Ordering::Greater => {
			let mut attr_class: Vec<char> = Vec::with_capacity(total * 5);
			let last_index = total - 1;
			for name in &class_list[..last_index] {
				attr_class.extend_from_slice(name);
				attr_class.push(' ');
			}
			attr_class.extend_from_slice(&class_list[last_index]);
			attr_class.iter().collect::<String>()
		}
		Ordering::Less => String::new(),
	}
}

pub fn is_equal_chars_ignore_case(target: &[char], cmp: &[char]) -> bool {
	if target.len() != cmp.len() {
		return false;
	}
	for (index, ch) in target.iter().enumerate() {
		let cmp_ch = &cmp[index];
		if cmp_ch == ch {
			continue;
		}
		match cmp_ch {
			'a'..='z' => {
				if &cmp_ch.to_ascii_uppercase() != ch {
					return false;
				}
			}
			'A'..='Z' => {
				if &cmp_ch.to_ascii_lowercase() != ch {
					return false;
				}
			}
			_ => {
				// not equal
				return false;
			}
		}
	}
	true
}

pub fn is_equal_chars(target: &[char], cmp: &[char]) -> bool {
	let t_len = target.len();
	let s_len = cmp.len();
	if t_len == s_len {
		for (index, ch) in target.iter().enumerate() {
			if ch != &cmp[index] {
				return false;
			}
		}
		return true;
	}
	false
}

fn contains_chars_nocheck(target: &[char], search: &[char], t_len: usize, s_len: usize) -> bool {
	// check if match
	let max_start_index: usize = t_len - s_len;
	let mut start_index: usize = 0;
	while start_index <= max_start_index {
		let mut move_one = false;
		for (index, ch) in target[start_index..start_index + s_len].iter().enumerate() {
			if ch == &search[index] {
				continue;
			}
			move_one = true;
		}
		if !move_one {
			return true;
		}
		start_index += 1;
	}
	false
}

pub fn contains_chars(target: &[char], search: &[char]) -> bool {
	let t_len = target.len();
	let s_len = search.len();
	if t_len < s_len {
		return false;
	}
	contains_chars_nocheck(target, search, t_len, s_len)
}

#[cfg(test)]
mod tests {
	use super::{divide_isize, RoundType};
	#[test]
	fn test_divide_isize() {
		// round
		// both positive
		assert_eq!(divide_isize(7, 4, RoundType::Round), 2);
		assert_eq!(divide_isize(6, 4, RoundType::Round), 2);
		assert_eq!(divide_isize(5, 4, RoundType::Round), 1);
		assert_eq!(divide_isize(4, 4, RoundType::Round), 1);
		// both negative
		assert_eq!(divide_isize(-7, -4, RoundType::Round), 2);
		assert_eq!(divide_isize(-6, -4, RoundType::Round), 2);
		assert_eq!(divide_isize(-5, -4, RoundType::Round), 1);
		assert_eq!(divide_isize(-4, -4, RoundType::Round), 1);
		// one positive, one negative
		assert_eq!(divide_isize(7, -4, RoundType::Round), -2);
		assert_eq!(divide_isize(6, -4, RoundType::Round), -2);
		assert_eq!(divide_isize(5, -4, RoundType::Round), -1);
		assert_eq!(divide_isize(4, -4, RoundType::Round), -1);
		// one positive, one negative
		assert_eq!(divide_isize(-7, 4, RoundType::Round), -2);
		assert_eq!(divide_isize(-6, 4, RoundType::Round), -2);
		assert_eq!(divide_isize(-5, 4, RoundType::Round), -1);
		assert_eq!(divide_isize(-4, 4, RoundType::Round), -1);
		// floor
		// 1
		assert_eq!(divide_isize(7, 4, RoundType::Floor), 1);
		assert_eq!(divide_isize(6, 4, RoundType::Floor), 1);
		assert_eq!(divide_isize(5, 4, RoundType::Floor), 1);
		assert_eq!(divide_isize(4, 4, RoundType::Floor), 1);
		// 2
		assert_eq!(divide_isize(-7, -4, RoundType::Floor), 1);
		assert_eq!(divide_isize(-6, -4, RoundType::Floor), 1);
		assert_eq!(divide_isize(-5, -4, RoundType::Floor), 1);
		assert_eq!(divide_isize(-4, -4, RoundType::Floor), 1);
		// 3
		assert_eq!(divide_isize(7, -4, RoundType::Floor), -2);
		assert_eq!(divide_isize(6, -4, RoundType::Floor), -2);
		assert_eq!(divide_isize(5, -4, RoundType::Floor), -2);
		assert_eq!(divide_isize(4, -4, RoundType::Floor), -1);
		// 4
		assert_eq!(divide_isize(-7, 4, RoundType::Floor), -2);
		assert_eq!(divide_isize(-6, 4, RoundType::Floor), -2);
		assert_eq!(divide_isize(-5, 4, RoundType::Floor), -2);
		assert_eq!(divide_isize(-4, 4, RoundType::Floor), -1);
		// ceil
		// 1
		assert_eq!(divide_isize(7, 4, RoundType::Ceil), 2);
		assert_eq!(divide_isize(6, 4, RoundType::Ceil), 2);
		assert_eq!(divide_isize(5, 4, RoundType::Ceil), 2);
		assert_eq!(divide_isize(4, 4, RoundType::Ceil), 1);
		// 2
		assert_eq!(divide_isize(-7, -4, RoundType::Ceil), 2);
		assert_eq!(divide_isize(-6, -4, RoundType::Ceil), 2);
		assert_eq!(divide_isize(-5, -4, RoundType::Ceil), 2);
		assert_eq!(divide_isize(-4, -4, RoundType::Ceil), 1);
		// 3
		assert_eq!(divide_isize(7, -4, RoundType::Ceil), -1);
		assert_eq!(divide_isize(6, -4, RoundType::Ceil), -1);
		assert_eq!(divide_isize(5, -4, RoundType::Ceil), -1);
		assert_eq!(divide_isize(4, -4, RoundType::Ceil), -1);
		// 4
		assert_eq!(divide_isize(-7, 4, RoundType::Ceil), -1);
		assert_eq!(divide_isize(-6, 4, RoundType::Ceil), -1);
		assert_eq!(divide_isize(-5, 4, RoundType::Ceil), -1);
		assert_eq!(divide_isize(-4, 4, RoundType::Ceil), -1);
	}
}
