use crate::mesdoc::constants::{NAME_SELECTOR_ALL, PRIORITY_ALL_SELECTOR};
use crate::mesdoc::interface::Elements;
use crate::mesdoc::selector::rule::{Matcher, RuleDefItem, RuleItem};
/// selector: `*`
pub fn init(rules: &mut Vec<RuleItem>) {
	let rule: RuleItem = RuleDefItem(
		NAME_SELECTOR_ALL,
		"*",
		PRIORITY_ALL_SELECTOR,
		Box::new(|_| Matcher {
			all_handle: Some(Box::new(|eles: &Elements, _| eles.cloned())),
			..Default::default()
		}),
	)
	.into();
	rules.push(rule);
}
