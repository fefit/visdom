use crate::mesdoc::constants::{NAME_SELECTOR_NAME, PRIORITY_NAME_SELECTOR};
use crate::mesdoc::selector::rule::{Matcher, RuleDefItem, RuleItem};
use crate::mesdoc::selector::MatchedQueue;
use crate::mesdoc::utils::is_equal_chars_ignore_case;

pub fn init(rules: &mut Vec<RuleItem>) {
	let rule = RuleDefItem(
		NAME_SELECTOR_NAME,
		"{identity}",
		PRIORITY_NAME_SELECTOR,
		Box::new(|mut data: MatchedQueue| {
			let name = data.remove(0).chars;
			Matcher {
				one_handle: Some(Box::new(move |ele, _| {
					is_equal_chars_ignore_case(&ele.tag_names(), &name)
				})),
				..Default::default()
			}
		}),
	);
	rules.push(rule.into());
}
