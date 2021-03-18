use crate::mesdoc::selector::rule::{Matcher, Rule, RuleItem};
use crate::mesdoc::selector::MatchedQueue;
use crate::mesdoc::{
	constants::{NAME_SELECTOR_ID, PRIORITY_ID_SELECTOR},
	interface::Elements,
};

pub fn init(rules: &mut Vec<RuleItem>) {
	let rule: RuleItem = RuleItem {
		name: NAME_SELECTOR_ID,
		context: "#{identity}",
		rule: Rule {
			priority: PRIORITY_ID_SELECTOR,
			in_cache: true,
			handle: Box::new(|data: MatchedQueue| {
				let id = data[1].chars.iter().collect::<String>();
				Matcher {
					all_handle: Some(Box::new(move |eles: &Elements, use_cache: Option<bool>| {
						let use_cache = use_cache.is_some();
						let mut result = Elements::with_capacity(1);
						if !eles.is_empty() {
							let first_ele = eles
								.get_ref()
								.get(0)
								.expect("The elements must have at least one element.");
							if let Some(doc) = &first_ele.owner_document() {
								if let Some(id_element) = doc.get_element_by_id(&id) {
									if use_cache {
										// just add, will checked if the element contains the id element
										result.push(id_element);
									} else {
										// filter methods, will filtered in elements
										for ele in eles.get_ref() {
											if ele.is(&id_element) {
												result.push(id_element);
												break;
											}
										}
									}
								}
							}
						}
						result
					})),
					..Default::default()
				}
			}),
			queues: Vec::new(),
		},
	};
	rules.push(rule);
}
