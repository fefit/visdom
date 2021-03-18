use crate::mesdoc::constants::{NAME_SELECTOR_CLASS, PRIORITY_CLASS_SELECTOR};
use crate::mesdoc::interface::IAttrValue;
use crate::mesdoc::selector::rule::Matcher;
use crate::mesdoc::selector::rule::{RuleDefItem, RuleItem};
use crate::mesdoc::selector::MatchedQueue;
use crate::mesdoc::utils::get_class_list;

pub fn init(rules: &mut Vec<RuleItem>) {
	let rule = RuleDefItem(
		NAME_SELECTOR_CLASS,
		".{identity}",
		PRIORITY_CLASS_SELECTOR,
		Box::new(|mut data: MatchedQueue| {
			// class name parameter
			let class_name = data.remove(1).chars;
			// matcher
			Matcher {
				one_handle: Some(Box::new(move |ele, _| -> bool {
					if let Some(IAttrValue::Value(names, _)) = ele.get_attribute("class") {
						let class_list = get_class_list(&names);
						return class_list.contains(&class_name);
					}
					false
				})),
				..Default::default()
			}
		}),
	);
	rules.push(rule.into());
}
