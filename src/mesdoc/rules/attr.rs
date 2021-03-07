#![allow(clippy::or_fun_call)]

use crate::mesdoc::constants::{NAME_SELECTOR_ATTR, PRIORITY_ATTR_SELECTOR};
use crate::mesdoc::interface::IAttrValue;
use crate::mesdoc::selector::rule::{Matcher, MatcherData};
use crate::mesdoc::selector::rule::{Rule, RuleDefItem, RuleItem};
pub fn init(rules: &mut Vec<RuleItem>) {
	let rule = RuleDefItem(
		NAME_SELECTOR_ATTR,
		r##"[{spaces}{attr_key}{spaces}{regexp#(?:([*^$~|!]?)=\s*(?:'((?:\\?+.)*?)'|([^\s\]'"<>/=`]+)|"((?:\\?+.)*?)"))?#}{spaces}]"##,
		PRIORITY_ATTR_SELECTOR,
		vec![("attr_key", 0), ("regexp", 0)],
		Box::new(|data: MatcherData| {
			let attr_key =
				Rule::param(&data, "attr_key").expect("The attribute selector's key is not correct");
			let attr_value = Rule::param(&data, ("regexp", 0, "2"))
				.or_else(|| Rule::param(&data, ("regexp", 0, "3")))
				.or_else(|| Rule::param(&data, ("regexp", 0, "4")));
			let match_mode = Rule::param(&data, ("regexp", 0, "1")).unwrap_or("");
			let handle: Box<dyn Fn(&Option<IAttrValue>) -> bool> = if let Some(attr_value) = attr_value {
				if attr_value.is_empty() {
					// empty attribute value
					Box::new(|_val: &Option<IAttrValue>| false)
				} else {
					match match_mode {
						// begin with value
						"^" => Box::new(move |val: &Option<IAttrValue>| match val {
							Some(IAttrValue::Value(v, _)) => v.starts_with(attr_value),
							_ => false,
						}),
						// end with value
						"$" => Box::new(move |val: &Option<IAttrValue>| match val {
							Some(IAttrValue::Value(v, _)) => v.ends_with(attr_value),
							_ => false,
						}),
						// contains value
						"*" => Box::new(move |val: &Option<IAttrValue>| match val {
							Some(IAttrValue::Value(v, _)) => v.contains(attr_value),
							_ => false,
						}),
						// either equal to value or start with `value` and followed `-`
						"|" => Box::new(move |val: &Option<IAttrValue>| match val {
							Some(IAttrValue::Value(v, _)) => {
								if v == attr_value {
									return true;
								}
								let attr_value: String = format!("{}-", attr_value);
								v.starts_with(&attr_value)
							}
							_ => false,
						}),
						// in a value list that splitted by whitespaces
						"~" => Box::new(move |val: &Option<IAttrValue>| match val {
							Some(IAttrValue::Value(v, _)) => {
								let split_v = v.split_ascii_whitespace();
								for v in split_v {
									if v == attr_value {
										return true;
									}
								}
								false
							}
							_ => false,
						}),
						// has a attribute and who's value not equal to setted value
						"!" => Box::new(move |val: &Option<IAttrValue>| match val {
							Some(IAttrValue::Value(v, _)) => attr_value != v,
							_ => false,
						}),
						// equal to value
						_ => Box::new(move |val: &Option<IAttrValue>| match val {
							Some(IAttrValue::Value(v, _)) => v == attr_value,
							_ => false,
						}),
					}
				}
			} else {
				// has the attribute name
				Box::new(|val: &Option<IAttrValue>| val.is_some())
			};
			Matcher {
				one_handle: Some(Box::new(move |ele, _| {
					let val = ele.get_attribute(attr_key);
					handle(&val)
				})),
				..Default::default()
			}
		}),
	);
	rules.push(rule.into());
}
