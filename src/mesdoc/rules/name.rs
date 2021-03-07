use crate::mesdoc::constants::{NAME_SELECTOR_NAME, PRIORITY_NAME_SELECTOR};
use crate::mesdoc::selector::rule::{Matcher, MatcherData, Rule, RuleDefItem, RuleItem};

pub fn init(rules: &mut Vec<RuleItem>) {
	let rule = RuleDefItem(
		NAME_SELECTOR_NAME,
		"{identity}",
		PRIORITY_NAME_SELECTOR,
		vec![("identity", 0)],
		Box::new(|data: MatcherData| {
			let name = Rule::param(&data, "identity")
				.expect("The 'name' selector must have a tag name")
				.to_ascii_uppercase();
			Matcher {
				one_handle: Some(Box::new(move |ele, _| ele.tag_name() == name)),
				..Default::default()
			}
		}),
	);
	rules.push(rule.into());
}
