use crate::mesdoc::interface::{BoxDynElement, Elements, IElementTrait, INodeType};
use crate::mesdoc::selector::pattern::Nth;
use crate::mesdoc::selector::rule::{Matcher, MatcherData, Rule, RuleDefItem, RuleItem};
use crate::mesdoc::{
	constants::{DEF_NODES_LEN, PRIORITY_PSEUDO_SELECTOR},
	selector::rule::MatchSpecifiedHandle,
};
use std::cmp::Ordering;
use std::{collections::HashMap, ops::Range};

const PRIORITY: u32 = PRIORITY_PSEUDO_SELECTOR;

/// pseudo selector ":empty"
fn pseudo_empty(rules: &mut Vec<RuleItem>) {
	// empty
	let selector = ":empty";
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		vec![],
		Box::new(|_: MatcherData| Matcher {
			one_handle: Some(Box::new(|ele, _| {
				let child_nodes = ele.child_nodes();
				if child_nodes.is_empty() {
					return true;
				}
				let mut only_comments = true;
				for node in child_nodes {
					match node.node_type() {
						INodeType::Comment => continue,
						_ => {
							only_comments = false;
							break;
						}
					}
				}
				only_comments
			})),
			..Default::default()
		}),
	);
	rules.push(rule.into());
}

// group siblings
struct SiblingsNodeData<'a> {
	range: Range<usize>,
	allow_indexs: Option<Vec<usize>>,
	parent: Option<BoxDynElement<'a>>,
}

fn need_use_specified(search: &usize, total: &usize) -> bool {
	total / search > 3
}

fn group_siblings_then_done<T, F>(eles: &Elements, allow_indexs_fn: T, mut cb: F)
where
	T: Fn(usize) -> Option<Vec<usize>>,
	F: FnMut(&mut SiblingsNodeData),
{
	let mut data = SiblingsNodeData {
		range: 0..0,
		allow_indexs: None,
		parent: None,
	};
	for (index, ele) in eles.get_ref().iter().enumerate() {
		if let Some(parent) = ele.parent() {
			let mut is_first = false;
			let mut in_next_group = false;
			if let Some(prev_parent) = &data.parent {
				if parent.is(&prev_parent) {
					// sibling node, just add
					data.range.end = index + 1;
				} else {
					// not sibling
					in_next_group = true;
				}
			} else {
				is_first = true;
			}
			// when meet next group siblings
			if in_next_group {
				cb(&mut data);
			}
			// when is first or in next group
			if is_first || in_next_group {
				// init the siblings, allow_index, prev_parent
				data.range.start = index;
				data.range.end = index + 1;
				data.parent = Some(parent.cloned());
				data.allow_indexs = allow_indexs_fn(parent.children().length());
			}
		}
	}
	if !data.range.is_empty() {
		cb(&mut data);
	}
}

fn make_asc_or_desc_specified(asc: bool, index: usize) -> MatchSpecifiedHandle {
	if asc {
		Box::new(move |ele, mut callback| {
			let mut find_index: Option<usize> = Some(0);
			// loop from first node to last node
			ele.child_nodes_item_since_by(
				0,
				false,
				Box::new(|child| {
					let is_matched = if let Some(orig_index) = find_index {
						let cur_index = orig_index + 1;
						if cur_index == index {
							find_index = None;
							true
						} else {
							find_index = Some(cur_index);
							false
						}
					} else {
						false
					};
					callback(child, is_matched);
					true
				}),
			)
		})
	} else {
		Box::new(move |ele, mut callback| {
			let total = ele.child_nodes_length();
			let mut find_index: Option<usize> = Some(0);
			// loop from last node to first node
			ele.child_nodes_item_since_by(
				total - 1,
				true,
				Box::new(|child| {
					let is_matched = if let Some(orig_index) = find_index {
						let cur_index = orig_index + 1;
						if cur_index == index {
							find_index = None;
							true
						} else {
							find_index = Some(cur_index);
							false
						}
					} else {
						false
					};
					callback(child, is_matched);
					true
				}),
			)
		})
	}
}

// make for 'nth-child','nth-last-child'
fn make_asc_or_desc_nth_child(selector: &'static str, asc: bool) -> RuleDefItem {
	let handle = if asc {
		|eles: &Elements,
		 range: &Range<usize>,
		 allow_indexs: &[usize],
		 childs: &Elements|
		 -> Vec<BoxDynElement> {
			// do with the siblings
			let childs = childs.get_ref();
			let mut finded: Vec<BoxDynElement> = Vec::with_capacity(allow_indexs.len());
			// optimize if loop all the childs
			if range.len() == childs.len() {
				// get all by indexs
				for &index in allow_indexs {
					finded.push(childs[index].cloned());
				}
			} else {
				let eles = eles.get_ref();
				let siblings = &eles[range.start..range.end];
				let mut start_index = 0;
				let mut allow_start_index = 0;
				let allow_indexs_total = allow_indexs.len();
				'loop_sibling: for ele in siblings {
					for (index, child) in childs[start_index..].iter().enumerate() {
						if child.is(ele) {
							let actual_index = start_index + index;
							for &allow_index in &allow_indexs[allow_start_index..] {
								match allow_index.cmp(&actual_index) {
									Ordering::Equal => {
										finded.push(ele.cloned());
										allow_start_index += 1;
										break;
									}
									Ordering::Less => {
										allow_start_index += 1;
									}
									Ordering::Greater => {
										break;
									}
								}
								if allow_start_index >= allow_indexs_total {
									break 'loop_sibling;
								}
							}
							start_index = actual_index + 1;
							break;
						}
					}
				}
			}
			finded
		}
	} else {
		|eles: &Elements, range: &Range<usize>, allow_indexs: &[usize], childs: &Elements| {
			// do with the siblings
			let childs = childs.get_ref();
			let total = childs.len();
			let mut finded: Vec<BoxDynElement> = Vec::with_capacity(allow_indexs.len());
			// optimize when loop all the childrens
			if range.len() == total {
				for &index in allow_indexs.iter().rev() {
					finded.push(childs[total - index - 1].cloned());
				}
			} else {
				let eles = eles.get_ref();
				let siblings = &eles[range.start..range.end];
				let mut cur_end = range.len();
				for (index, child) in childs.iter().rev().enumerate() {
					let last_index = total - index - 1;
					// use binary search for faster speed
					if allow_indexs.binary_search(&last_index).is_err() {
						continue;
					}
					for (i, ele) in siblings[..cur_end].iter().rev().enumerate() {
						if child.is(ele) {
							cur_end -= i + 1;
							finded.push(ele.cloned());
							break;
						}
					}
					// break if at the beginning
					if cur_end == 0 {
						break;
					}
				}
				finded.reverse();
			}
			finded
		}
	};
	let name = if asc { ":nth-child" } else { ":nth-last-child" };
	RuleDefItem(
		name,
		selector,
		PRIORITY,
		vec![("nth", 0)],
		Box::new(move |data: MatcherData| {
			let n = Rule::param(&data, ("nth", 0, "n"));
			let index = Rule::param(&data, ("nth", 0, "index"));
			let specified_handle = if n.is_none() {
				let index = index
					.expect("Nth's n and index must have one")
					.parse::<usize>()
					.expect("Nth's index is not ok");
				Some(make_asc_or_desc_specified(asc, index))
			} else {
				None
			};
			Matcher {
				all_handle: Some(Box::new(move |eles: &Elements, is_all| {
					let mut result: Elements = Elements::with_capacity(DEF_NODES_LEN);
					if is_all.is_none() {
						group_siblings_then_done(
							eles,
							|total: usize| Some(Nth::get_allowed_indexs(n, index, total)),
							|data: &mut SiblingsNodeData| {
								let allow_indexs = data.allow_indexs.as_ref().expect("allow indexs must set");
								if allow_indexs.is_empty() {
									return;
								}
								let childs = data
									.parent
									.as_ref()
									.expect("parent must set in callback")
									.children();
								let finded = handle(&eles, &data.range, &allow_indexs, &childs);
								result.get_mut_ref().extend(finded);
							},
						);
					} else {
						let total = eles.length();
						let range = 0..total;
						let allow_indexs = Nth::get_allowed_indexs(n, index, total);
						let finded = handle(&eles, &range, &allow_indexs, &eles);
						result.get_mut_ref().extend(finded);
					}
					result
				})),
				specified_handle,
				..Default::default()
			}
		}),
	)
}
/// pseudo selector: `:nth-child`
fn pseudo_nth_child(rules: &mut Vec<RuleItem>) {
	let rule = make_asc_or_desc_nth_child(":nth-child({spaces}{nth}{spaces})", true);
	rules.push(rule.into());
}

/// pseudo selector: `:nth-child`
fn pseudo_nth_last_child(rules: &mut Vec<RuleItem>) {
	let rule = make_asc_or_desc_nth_child(":nth-last-child({spaces}{nth}{spaces})", false);
	rules.push(rule.into());
}

/// pseudo selector `:first-child,:last-child`
fn pseudo_first_child(rules: &mut Vec<RuleItem>) {
	// first-child,alias for ':nth-child(1)'
	let selector = ":first-child";
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		vec![],
		Box::new(|_| Matcher {
			all_handle: Some(Box::new(|_, _| Elements::new())),
			..Default::default()
		}),
	);
	rules.push(rule.into());
}

fn pseudo_last_child(rules: &mut Vec<RuleItem>) {
	// last-child,alias for ':nth-last-child(1)'
	let selector = ":last-child";
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		vec![],
		Box::new(|_| Rule::make_alias(":nth-last-child(1)")),
	);
	rules.push(rule.into());
}

type NameCounterHashMap = HashMap<String, usize>;

// check if cur tag's name is ok
fn get_allowed_name_ele(
	ele: &BoxDynElement,
	names: &mut NameCounterHashMap,
	allow_indexs: &[usize],
	node_indexs: &mut Vec<usize>,
) {
	let name = ele.tag_name();
	if let Some(index) = names.get_mut(name) {
		// increase index
		*index += 1;
		// use binary search is much faster than contains
		if allow_indexs.binary_search(index).is_ok() {
			node_indexs.push(ele.index())
		}
	} else {
		let index = 0;
		names.insert(String::from(name), index);
		// just check if first is 0
		if allow_indexs[0] == 0 {
			node_indexs.push(ele.index())
		}
	}
}

// collect available elements from siblings
fn collect_avail_name_eles(
	node_indexs: &mut Vec<usize>,
	siblings: &[BoxDynElement],
	finded: &mut Vec<BoxDynElement>,
) {
	let av_total = node_indexs.len();
	if av_total > 0 {
		let mut av_index = 0;
		let sib_total = siblings.len();
		let mut sib_index = 0;
		while av_index < av_total && sib_index < sib_total {
			let cur_avail_index = node_indexs[av_index];
			av_index += 1;
			for ele in &siblings[sib_index..] {
				let cur_sib_index = ele.index();
				match cur_sib_index.cmp(&cur_avail_index) {
					Ordering::Equal => {
						finded.push(ele.cloned());
						sib_index += 1;
						break;
					}
					Ordering::Greater => {
						break;
					}
					Ordering::Less => {
						sib_index += 1;
					}
				}
			}
		}
	}
}

// make nth of type: `:nth-of-type`, `:nth-last-of-type`
fn make_asc_or_desc_nth_of_type(selector: &'static str, asc: bool) -> RuleDefItem {
	let name = if asc {
		":nth-of-type"
	} else {
		":nth-last-of-type"
	};
	// last of type
	RuleDefItem(
		name,
		selector,
		PRIORITY,
		vec![("nth", 0)],
		Box::new(move |data: MatcherData| {
			let n = Rule::param(&data, ("nth", 0, "n"));
			let index = Rule::param(&data, ("nth", 0, "index"));
			Matcher {
				all_handle: Some(Box::new(move |eles: &Elements, _| {
					let mut result: Elements = Elements::with_capacity(DEF_NODES_LEN);
					group_siblings_then_done(
						eles,
						|total: usize| Some(Nth::get_allowed_indexs(n, index, total)),
						|data: &mut SiblingsNodeData| {
							let allow_indexs = data
								.allow_indexs
								.as_ref()
								.expect("Nth allow indexs must have");
							// return if allow_indexs is empty
							if allow_indexs.is_empty() {
								return;
							}
							// childs
							let childs = data
								.parent
								.as_ref()
								.expect("parent must set in callback")
								.children();
							let mut names: NameCounterHashMap = HashMap::with_capacity(5);
							let range = &data.range;
							let eles = eles.get_ref();
							let siblings = &eles[range.start..range.end];
							let mut node_indexs: Vec<usize> = Vec::with_capacity(childs.length());
							// loop to get allowed child's node indexs
							if asc {
								for child in childs.get_ref() {
									get_allowed_name_ele(child, &mut names, allow_indexs, &mut node_indexs);
								}
							} else {
								for child in childs.get_ref().iter().rev() {
									get_allowed_name_ele(child, &mut names, allow_indexs, &mut node_indexs);
								}
								node_indexs.reverse();
							}
							collect_avail_name_eles(&mut node_indexs, siblings, result.get_mut_ref());
						},
					);
					result
				})),
				..Default::default()
			}
		}),
	)
}

/// pseudo selector:`:nth-of-type`
fn pseudo_nth_of_type(rules: &mut Vec<RuleItem>) {
	// last of type
	let rule = make_asc_or_desc_nth_of_type(":nth-of-type({spaces}{nth}{spaces})", true);
	rules.push(rule.into());
}

/// pseudo selector:`:nth-last-of-type`
fn pseudo_nth_last_of_type(rules: &mut Vec<RuleItem>) {
	// last of type
	let rule = make_asc_or_desc_nth_of_type(":nth-last-of-type({spaces}{nth}{spaces})", false);
	rules.push(rule.into());
}

/// pseudo selector:`:first-of-type `
fn pseudo_first_of_type(rules: &mut Vec<RuleItem>) {
	// first of type, alias for 'nth-of-type(1)'
	let selector = ":first-of-type";
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		vec![],
		Box::new(|_| Rule::make_alias(":nth-of-type(1)")),
	);
	rules.push(rule.into());
}

/// pseudo selector:`:last-of-type`
fn pseudo_last_of_type(rules: &mut Vec<RuleItem>) {
	// last of type, alias for 'nth-last-of-type(1)'
	let selector = ":last-of-type";
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		vec![],
		Box::new(|_| Rule::make_alias(":nth-last-of-type(1)")),
	);
	rules.push(rule.into());
}

/// pseudo selector: `only-child`
fn pseudo_only_child(rules: &mut Vec<RuleItem>) {
	let selector = ":only-child";
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		vec![],
		Box::new(move |_| Matcher {
			all_handle: Some(Box::new(|eles: &Elements, _| {
				let mut result = Elements::with_capacity(DEF_NODES_LEN);
				let mut prev_parent: Option<BoxDynElement> = None;
				for ele in eles.get_ref() {
					if let Some(parent) = &ele.parent() {
						if let Some(prev_parent) = &prev_parent {
							if prev_parent.is(parent) {
								continue;
							}
						}
						prev_parent = Some(parent.cloned());
						let child_nodes = parent.child_nodes();
						let mut count = 0;
						for node in &child_nodes {
							if matches!(node.node_type(), INodeType::Element) {
								count += 1;
								if count > 1 {
									break;
								}
							}
						}
						if count == 1 {
							result.push(ele.cloned());
						}
					}
				}
				result
			})),
			..Default::default()
		}),
	);
	rules.push(rule.into());
}

/// pseudo selector: `only-child`
fn pseudo_only_of_type(rules: &mut Vec<RuleItem>) {
	let selector = ":only-of-type";
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		vec![],
		Box::new(move |_| Matcher {
			all_handle: Some(Box::new(|eles: &Elements, _| {
				let mut result = Elements::with_capacity(DEF_NODES_LEN);
				group_siblings_then_done(
					eles,
					|_| None,
					|data: &mut SiblingsNodeData| {
						let childs = data
							.parent
							.as_ref()
							.expect("parent must set in callback")
							.children();
						let eles = eles.get_ref();
						let range = &data.range;
						let siblings = &eles[range.start..range.end];
						let mut only_names: Vec<(String, usize)> = Vec::with_capacity(DEF_NODES_LEN);
						let mut repeated: Vec<String> = Vec::with_capacity(DEF_NODES_LEN);
						for (index, child) in childs.get_ref().iter().enumerate() {
							let name = String::from(child.tag_name());
							if !repeated.contains(&name) {
								let find_index = only_names
									.iter()
									.position(|(tag_name, _)| tag_name == &name);
								if let Some(index) = find_index {
									repeated.push(name);
									only_names.remove(index);
								} else {
									only_names.push((name, index));
								}
							}
						}
						if !only_names.is_empty() {
							let finded = result.get_mut_ref();
							// most time, we detect all the childs
							if siblings.len() == childs.length() {
								for (_, index) in &only_names {
									finded.push(siblings[*index].cloned());
								}
							} else {
								let mut cur_index = 0;
								for (name, _) in &only_names {
									for (index, ele) in siblings[cur_index..].iter().enumerate() {
										if ele.tag_name() == name {
											cur_index += index + 1;
											finded.push(ele.cloned());
											break;
										}
									}
								}
							}
						}
					},
				);
				result
			})),
			..Default::default()
		}),
	);
	rules.push(rule.into());
}

/// pseudo selector: `:not`
fn pseudo_not(rules: &mut Vec<RuleItem>) {
	let name = ":not";
	let selector = ":not({spaces}{selector}{spaces})";
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		vec![("selector", 0)],
		Box::new(|data: MatcherData| {
			let selector = Rule::param(&data, "selector").expect("selector param must have.");
			Matcher {
				all_handle: Some(Box::new(move |eles: &Elements, _| eles.not(selector))),
				..Default::default()
			}
		}),
	);
	rules.push(rule.into());
}

/// pseudo selector: `:contains`
fn pseudo_contains(rules: &mut Vec<RuleItem>) {
	let name = ":contains";
	let selector =
		r##":contains({spaces}{regexp#(?:'((?:\\?+.)*?)'|"((?:\\?+.)*?)"|([^\s'"<>/=`]*))#}{spaces})"##;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		vec![("regexp", 0)],
		Box::new(|data: MatcherData| {
			let search = Rule::param(&data, ("regexp", 0, "1"))
				.or_else(|| Rule::param(&data, ("regexp", 0, "2")))
				.or_else(|| Rule::param(&data, ("regexp", 0, "3")))
				.expect("The :contains selector must have a content");
			Matcher {
				one_handle: Some(Box::new(move |ele, _| {
					if search.is_empty() {
						return true;
					}
					ele.text().contains(search)
				})),
				..Default::default()
			}
		}),
	);
	rules.push(rule.into());
}

// -----------jquery selectors----------

/// pseudo selector: `:header`
fn pseudo_alias_header(rules: &mut Vec<RuleItem>) {
	let selector = ":header";
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		vec![],
		Box::new(|_| Rule::make_alias("h1,h2,h3,h4,h5,h6")),
	);
	rules.push(rule.into());
}

/// pseudo selector: `:input`
fn pseudo_alias_input(rules: &mut Vec<RuleItem>) {
	let selector = ":input";
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		vec![],
		Box::new(|_| Rule::make_alias("input,select,textarea,button")),
	);
	rules.push(rule.into());
}

/// pseudo selector: `:submit`
fn pseudo_alias_submit(rules: &mut Vec<RuleItem>) {
	let selector = ":submit";
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		vec![],
		Box::new(|_| Rule::make_alias("input[type='submit'],button[type='submit']")),
	);
	rules.push(rule.into());
}

pub fn init(rules: &mut Vec<RuleItem>) {
	pseudo_empty(rules);
	// first-child, last-child
	pseudo_first_child(rules);
	pseudo_last_child(rules);
	// only-child
	pseudo_only_child(rules);
	// nth-child,nth-last-child
	pseudo_nth_child(rules);
	pseudo_nth_last_child(rules);
	// first-of-type,last-of-type
	pseudo_first_of_type(rules);
	pseudo_last_of_type(rules);
	// nth-of-type,nth-last-of-type
	pseudo_nth_of_type(rules);
	pseudo_nth_last_of_type(rules);
	// only-of-type
	pseudo_only_of_type(rules);
	// not
	pseudo_not(rules);
	// contains
	pseudo_contains(rules);
	// ---- jquery selectors -----
	// :header alias
	pseudo_alias_header(rules);
	// :input alias
	pseudo_alias_input(rules);
	// :submit alias
	pseudo_alias_submit(rules);
}
