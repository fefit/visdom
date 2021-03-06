use std::error::Error;

pub fn to_static_str(content: String) -> &'static str {
	Box::leak(content.into_boxed_str())
}

pub fn vec_char_to_clean_str(v: &mut Vec<char>) -> &'static str {
	to_static_str(v.drain(..).collect::<String>())
}

pub fn chars_to_int(v: &[char]) -> Result<usize, Box<dyn Error>> {
	let index = v.iter().collect::<String>();
	let index = index.parse::<usize>()?;
	Ok(index)
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
	if indexs.len() < 30 {
		for (i, index) in indexs.iter().enumerate() {
			v.remove(index - i);
		}
	} else {
		let mut loop_index: usize = 0;
		v.retain(|_| {
			let removed = indexs.contains(&loop_index);
			loop_index += 1;
			!removed
		});
	}
}

pub fn get_class_list(v: &str) -> Vec<&str> {
	let v = v.trim();
	if v.is_empty() {
		vec![]
	} else {
		v.split_ascii_whitespace().collect::<Vec<&str>>()
	}
}

#[cfg(test)]
mod test {
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
