#![deny(clippy::print_stdout)]
use crate::mesdoc::interface::{BoxDynElement, Elements, IAttrValue, IElementTrait, INodeType};
use crate::mesdoc::selector::pattern::Nth;
use crate::mesdoc::selector::rule::{Matcher, Rule, RuleDefItem, RuleItem};
use crate::mesdoc::selector::MatchedQueue;
use crate::mesdoc::utils::{contains_chars, is_equal_chars};
use crate::mesdoc::{
	constants::{
		DEF_NODES_LEN, PRIORITY_PSEUDO_SELECTOR, SELECTOR_ALIAS_NAME_HEADER, SELECTOR_ALIAS_NAME_INPUT,
		SELECTOR_ALIAS_NAME_SUBMIT,
	},
	selector::rule::MatchSpecifiedHandle,
};
use std::{cmp::Ordering, collections::VecDeque};
use std::{collections::HashMap, ops::Range};
const PRIORITY: u32 = PRIORITY_PSEUDO_SELECTOR;

fn nth_index_to_number(index: &Option<String>) -> isize {
	index
		.as_ref()
		.expect("Nth's n and index must have one")
		.parse::<isize>()
		.expect("Nth's index is not ok")
}

/// Pseudo selector ":root"
fn pseudo_root(rules: &mut Vec<RuleItem>) {
	let selector = ":root";
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		Box::new(|_| Matcher {
			specified_handle: Some(Box::new(|ele, mut callback| {
				if ele.is_root_element() {
					let total = ele.child_nodes_length();
					for index in 0..total {
						let node = ele
							.child_nodes_item(index)
							.expect("Child nodes item index must less than total");
						if matches!(node.node_type(), INodeType::Element) {
							let ele = node
								.typed()
								.into_element()
								.expect("Call `typed` for element ele.");
							if ele.tag_name() == "HTML" {
								callback(&*ele, true, false);
							}
						}
					}
				}
			})),
			one_handle: Some(Box::new(|ele, _| {
				if ele.tag_name() == "HTML" {
					if let Some(parent) = &ele.parent() {
						return matches!(parent.node_type(), INodeType::Document);
					}
				}
				false
			})),
			..Default::default()
		}),
	);
	rules.push(rule.into());
}

/// pseudo selector ":empty"
fn pseudo_empty(rules: &mut Vec<RuleItem>) {
	// empty
	let selector = ":empty";
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		Box::new(|_| Matcher {
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
				if parent.is(prev_parent) {
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
				data.allow_indexs = allow_indexs_fn(parent.children().length());
				data.parent = Some(parent);
			}
		}
	}
	if !data.range.is_empty() {
		cb(&mut data);
	}
}

// make specified for nth child
// index means nth(index), so 1 means the first child
fn make_asc_or_desc_nth_child_specified(asc: bool, index: isize) -> MatchSpecifiedHandle {
	if index > 0 {
		let index = (index - 1) as usize;
		if asc {
			Box::new(move |ele, mut callback| {
				let mut cur_index: usize = 0;
				// loop from first node to last node
				ele.child_nodes_item_since_by(
					0,
					false,
					Box::new(|child| {
						let is_matched = cur_index == index;
						cur_index += 1;
						callback(child, is_matched, true);
						true
					}),
				);
			})
		} else {
			Box::new(move |ele, mut callback| {
				let total = ele.child_nodes_length();
				let mut matched: VecDeque<bool> = VecDeque::with_capacity(total);
				// loop from last node to first node
				// gather the matched info
				let mut cur_index: usize = 0;
				ele.child_nodes_item_since_by(
					total - 1,
					true,
					Box::new(|_| {
						let is_matched = cur_index == index;
						cur_index += 1;
						matched.push_front(is_matched);
						true
					}),
				);
				// loop then execute the callback
				let mut loop_index: usize = 0;
				ele.child_nodes_item_since_by(
					0,
					false,
					Box::new(|child| {
						callback(child, matched[loop_index], true);
						loop_index += 1;
						true
					}),
				);
			})
		}
	} else {
		// match nothing, do nothing
		Box::new(move |_, _| {})
	}
}

type NthChildHandle = Box<
	dyn for<'a, 'r> Fn(
			&'a Elements<'r>,
			&'a Range<usize>,
			&'a [usize],
			&'a Elements<'r>,
		) -> Vec<BoxDynElement<'r>>
		+ Send
		+ Sync,
>;
// make nth child handle
fn make_asc_or_desc_nth_child_handle(asc: bool) -> NthChildHandle {
	if asc {
		Box::new(
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
			},
		)
	} else {
		Box::new(
			|eles: &Elements, range: &Range<usize>, allow_indexs: &[usize], childs: &Elements| {
				// do with the siblings
				let childs = childs.get_ref();
				let total = childs.len();
				let mut finded: Vec<BoxDynElement> = Vec::with_capacity(allow_indexs.len());
				// optimize when loop all the childrens
				if range.len() == total {
					// becareful the index in allow_indexs is calc from the end
					// so the index 0 is the last child
					for &index in allow_indexs.iter().rev() {
						finded.push(childs[total - index - 1].cloned());
					}
				} else {
					let eles = eles.get_ref();
					let siblings = &eles[range.start..range.end];
					let mut cur_end = range.len();
					for (index, child) in childs.iter().rev().enumerate() {
						// use binary search for faster speed
						// now the index has reversed, so it's same as the index in allow_indexs
						if allow_indexs.binary_search(&index).is_err() {
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
			},
		)
	}
}

// all handle for nth child and first-child or last-child
fn handle_nth_child<'r>(
	data: &SiblingsNodeData,
	eles: &Elements<'r>,
	result: &mut Elements<'r>,
	handle: &NthChildHandle,
) {
	let allow_indexs = data.allow_indexs.as_ref().expect("allow indexs must set");
	if allow_indexs.is_empty() {
		return;
	}
	let childs = data
		.parent
		.as_ref()
		.expect("parent must set in callback")
		.children();
	let finded = handle(eles, &data.range, allow_indexs, &childs);
	result.get_mut_ref().extend(finded);
}
// make for 'nth-child','nth-last-child'
fn make_asc_or_desc_nth_child(selector: &'static str, asc: bool) -> RuleDefItem {
	let name = selector;
	RuleDefItem(
		name,
		selector,
		PRIORITY,
		Box::new(move |data: MatchedQueue| {
			let nth_data = &data[2].data;
			let n = nth_data.get("n").map(|s| s.clone());
			let index = nth_data.get("index").map(|s| s.clone());
			let handle = make_asc_or_desc_nth_child_handle(asc);
			let specified_handle = if n.is_none() {
				let index = nth_index_to_number(&index);
				Some(make_asc_or_desc_nth_child_specified(asc, index))
			} else {
				None
			};
			Matcher {
				all_handle: Some(Box::new(move |eles: &Elements, is_all| {
					let mut result: Elements = Elements::with_capacity(DEF_NODES_LEN);
					if is_all.is_none() {
						group_siblings_then_done(
							eles,
							|total: usize| Some(Nth::get_allowed_indexs(&n, &index, total)),
							|data: &mut SiblingsNodeData| {
								handle_nth_child(data, eles, &mut result, &handle);
							},
						);
					} else {
						let total = eles.length();
						let range = 0..total;
						let allow_indexs = Nth::get_allowed_indexs(&n, &index, total);
						let finded = handle(eles, &range, &allow_indexs, eles);
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

// make first or last child rule
fn make_first_or_last_child(selector: &'static str, asc: bool) -> RuleDefItem {
	let name = selector;
	RuleDefItem(
		name,
		selector,
		PRIORITY,
		Box::new(move |_| {
			let handle = make_asc_or_desc_nth_child_handle(asc);
			let specified_handle = Some(make_asc_or_desc_nth_child_specified(asc, 1));
			Matcher {
				all_handle: Some(Box::new(move |eles, _| {
					let mut result: Elements = Elements::with_capacity(DEF_NODES_LEN);
					group_siblings_then_done(
						eles,
						|_| Some(vec![0]),
						|data: &mut SiblingsNodeData| {
							handle_nth_child(data, eles, &mut result, &handle);
						},
					);
					/*
						// is_all
						let range = 0..1;
						let allow_indexs = vec![0];
						let finded = handle(&eles, &range, &allow_indexs, &eles);
						result.get_mut_ref().extend(finded);
					*/
					result
				})),
				specified_handle,
				..Default::default()
			}
		}),
	)
}

/// pseudo selector `:first-child,:last-child`
fn pseudo_first_child(rules: &mut Vec<RuleItem>) {
	// first-child
	let rule = make_first_or_last_child(":first-child", true);
	rules.push(rule.into());
}

fn pseudo_last_child(rules: &mut Vec<RuleItem>) {
	// last-child,alias for ':nth-last-child(1)'
	let rule = make_first_or_last_child(":last-child", false);
	rules.push(rule.into());
}

type NameCounterHashMap = HashMap<String, usize>;

// check if cur tag's name is ok
fn get_allowed_name_ele(
	ele: &dyn IElementTrait,
	names: &mut NameCounterHashMap,
	allow_indexs: &[usize],
	node_indexs: &mut Vec<usize>,
) -> bool {
	let name = ele.tag_name();
	if let Some(index) = names.get_mut(&name) {
		// increase index
		*index += 1;
		// use binary search is much faster than contains
		if allow_indexs.binary_search(index).is_ok() {
			node_indexs.push(ele.index());
			return true;
		}
	} else {
		let index = 0;
		names.insert(name, index);
		// just check if first is 0
		if allow_indexs[0] == 0 {
			node_indexs.push(ele.index());
			return true;
		}
	}
	false
}

// collect available elements from siblings
fn collect_avail_name_eles(
	node_indexs: &mut [usize],
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

// index is nth-of-type(index), so it's begin 1
fn make_asc_or_desc_nth_of_type_specified(asc: bool, index: isize) -> MatchSpecifiedHandle {
	if index > 0 {
		let allow_indexs = vec![(index - 1) as usize];
		if asc {
			Box::new(move |ele, mut callback| {
				let mut names: NameCounterHashMap = HashMap::with_capacity(DEF_NODES_LEN);
				let mut node_indexs: Vec<usize> = Vec::with_capacity(DEF_NODES_LEN);
				// loop from first node to last node
				ele.child_nodes_item_since_by(
					0,
					false,
					Box::new(|child| {
						let is_matched =
							get_allowed_name_ele(child, &mut names, &allow_indexs, &mut node_indexs);
						callback(child, is_matched, true);
						true
					}),
				);
			})
		} else {
			Box::new(move |ele, mut callback| {
				let mut names: NameCounterHashMap = HashMap::with_capacity(DEF_NODES_LEN);
				let mut node_indexs: Vec<usize> = Vec::with_capacity(DEF_NODES_LEN);
				// total
				let total = ele.child_nodes_length();
				let mut matched: VecDeque<bool> = VecDeque::with_capacity(total);
				// loop from last node to first node
				// gather the matched info
				ele.child_nodes_item_since_by(
					total - 1,
					true,
					Box::new(|child| {
						let is_matched =
							get_allowed_name_ele(child, &mut names, &allow_indexs, &mut node_indexs);
						matched.push_front(is_matched);
						true
					}),
				);
				// loop then execute the callback
				let mut loop_index: usize = 0;
				ele.child_nodes_item_since_by(
					0,
					false,
					Box::new(|child| {
						callback(child, matched[loop_index], true);
						loop_index += 1;
						true
					}),
				);
			})
		}
	} else {
		// match nothing and do nothing
		Box::new(|_, _| {})
	}
}

// handle nth of type
fn handle_nth_of_type(asc: bool, data: &SiblingsNodeData, eles: &Elements, result: &mut Elements) {
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
	let mut names: NameCounterHashMap = HashMap::with_capacity(DEF_NODES_LEN);
	let mut node_indexs: Vec<usize> = Vec::with_capacity(childs.length());
	let range = &data.range;
	let eles = eles.get_ref();
	let siblings = &eles[range.start..range.end];
	// loop to get allowed child's node indexs
	if asc {
		for child in childs.get_ref() {
			get_allowed_name_ele(&**child, &mut names, allow_indexs, &mut node_indexs);
		}
	} else {
		for child in childs.get_ref().iter().rev() {
			get_allowed_name_ele(&**child, &mut names, allow_indexs, &mut node_indexs);
		}
		node_indexs.reverse();
	}
	collect_avail_name_eles(&mut node_indexs, siblings, result.get_mut_ref());
}

// make nth of type: `:nth-of-type`, `:nth-last-of-type`
fn make_asc_or_desc_nth_of_type(selector: &'static str, asc: bool) -> RuleDefItem {
	let name = selector;
	// last of type
	RuleDefItem(
		name,
		selector,
		PRIORITY,
		Box::new(move |mut data: MatchedQueue| {
			let nth_data = data.remove(2).data;
			let n = nth_data.get("n").map(|s| s.clone());
			let index = nth_data.get("index").map(|s| s.clone());
			let specified_handle = if n.is_none() {
				let index = nth_index_to_number(&index);
				Some(make_asc_or_desc_nth_of_type_specified(asc, index))
			} else {
				None
			};
			Matcher {
				all_handle: Some(Box::new(move |eles: &Elements, is_all| {
					let mut result: Elements = Elements::with_capacity(DEF_NODES_LEN);
					if is_all.is_none() {
						group_siblings_then_done(
							eles,
							|total: usize| Some(Nth::get_allowed_indexs(&n, &index, total)),
							|data: &mut SiblingsNodeData| {
								handle_nth_of_type(asc, data, eles, &mut result);
							},
						);
					} else {
						// is_all
						let total = eles.length();
						let allow_indexs = Some(Nth::get_allowed_indexs(&n, &index, total));
						let parent = if total > 0 {
							eles.get(0).expect("length > 0").parent()
						} else {
							None
						};
						let data = SiblingsNodeData {
							range: 0..total,
							allow_indexs,
							parent,
						};
						handle_nth_of_type(asc, &data, eles, &mut result);
					}
					result
				})),
				specified_handle,
				..Default::default()
			}
		}),
	)
}

/// pseudo selector:`:nth-of-type`
fn pseudo_nth_of_type(rules: &mut Vec<RuleItem>) {
	// nth of type
	let rule = make_asc_or_desc_nth_of_type(":nth-of-type({spaces}{nth}{spaces})", true);
	rules.push(rule.into());
}

/// pseudo selector:`:nth-last-of-type`
fn pseudo_nth_last_of_type(rules: &mut Vec<RuleItem>) {
	// nth last of type
	let rule = make_asc_or_desc_nth_of_type(":nth-last-of-type({spaces}{nth}{spaces})", false);
	rules.push(rule.into());
}

// make first-of-type last-of-type
fn make_first_or_last_of_type(selector: &'static str, asc: bool) -> RuleDefItem {
	let name = selector;
	// last of type
	RuleDefItem(
		name,
		selector,
		PRIORITY,
		Box::new(move |_| {
			let specified_handle = Some(make_asc_or_desc_nth_of_type_specified(asc, 1));
			Matcher {
				all_handle: Some(Box::new(move |eles: &Elements, _| {
					let mut result: Elements = Elements::with_capacity(DEF_NODES_LEN);
					group_siblings_then_done(
						eles,
						|_: usize| Some(vec![0]),
						|data: &mut SiblingsNodeData| {
							handle_nth_of_type(asc, data, eles, &mut result);
						},
					);
					/*
					 let total = eles.length();
					 let allow_indexs = Some(vec![0]);
					 let parent = if total > 0 {
						 eles.get(0).expect("length > 0").parent()
					 } else {
						 None
					 };
					 let data = SiblingsNodeData {
						 range: 0..total,
						 allow_indexs,
						 parent,
					 };
					 handle_nth_of_type(asc, &data, eles, &mut result);
					*/
					result
				})),
				specified_handle,
				..Default::default()
			}
		}),
	)
}
/// pseudo selector:`:first-of-type `
fn pseudo_first_of_type(rules: &mut Vec<RuleItem>) {
	// first of type, equal to 'nth-of-type(1)'
	let rule = make_first_or_last_of_type(":first-of-type", true);
	rules.push(rule.into());
}

/// pseudo selector:`:last-of-type`
fn pseudo_last_of_type(rules: &mut Vec<RuleItem>) {
	// last of type, equal to 'nth-last-of-type(1)'
	let rule = make_first_or_last_of_type(":last-of-type", false);
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
		Box::new(move |_| Matcher {
			all_handle: Some(Box::new(|eles: &Elements, _| {
				let mut result = Elements::with_capacity(DEF_NODES_LEN);
				let mut prev_parent: Option<BoxDynElement> = None;
				for ele in eles.get_ref() {
					if let Some(parent) = ele.parent() {
						if let Some(prev_parent) = &prev_parent {
							if prev_parent.is(&parent) {
								continue;
							}
						}
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
						prev_parent = Some(parent);
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
							let name = child.tag_name();
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
										if &ele.tag_name() == name {
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
		Box::new(|data: MatchedQueue| {
			let selector = data[2].chars.iter().collect::<String>();
			Matcher {
				all_handle: Some(Box::new(move |eles: &Elements, _| eles.not(&selector))),
				..Default::default()
			}
		}),
	);
	rules.push(rule.into());
}

/// pseudo selector: `:contains`
fn pseudo_contains(rules: &mut Vec<RuleItem>) {
	let name = ":contains";
	let selector = r##":contains({spaces}{regexp#(?:'((?:\\?+.)*?)'|"((?:\\?+.)*?)"|([^)\s'"<>/=`]*))#}{spaces})"##;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		Box::new(|mut data: MatchedQueue| {
			let mut find_chars = data.remove(2).chars;
			if !find_chars.is_empty() {
				let first = find_chars[0];
				let search = if first == '"' || first == '\'' {
					find_chars.pop();
					find_chars.split_off(1)
				} else {
					find_chars
				};
				Matcher {
					one_handle: Some(Box::new(move |ele, _| {
						let contents = ele.text_contents();
						// let contents = vec!['b'];
						// let contents_count = contents.len();
						contains_chars(&contents, &search)
						// match contents_count.cmp(&search_count) {
						// 	Ordering::Less => false,
						// 	_ => ,
						// }
					})),
					..Default::default()
				}
			} else {
				Matcher {
					one_handle: Some(Box::new(move |_, _| true)),
					..Default::default()
				}
			}
		}),
	);
	rules.push(rule.into());
}

// -----------jquery selectors----------

/// pseudo selector: `:has`
fn pseudo_has(rules: &mut Vec<RuleItem>) {
	let name = ":has";
	let selector = ":has({spaces}{selector}{spaces})";
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		Box::new(|data: MatchedQueue| {
			let selector = data[2].chars.iter().collect::<String>();
			Matcher {
				all_handle: Some(Box::new(move |eles: &Elements, _| eles.has(&selector))),
				..Default::default()
			}
		}),
	);
	rules.push(rule.into());
}

/// pseudo selector: `:checked`
fn pseudo_checked(rules: &mut Vec<RuleItem>) {
	let selector = ":checked";
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		Box::new(|_| Matcher {
			one_handle: Some(Box::new(|ele, _| {
				let tag_name = ele.tag_names();
				let input_tag = ['i', 'n', 'p', 'u', 't'];
				let option_tag = ['o', 'p', 't', 'i', 'o', 'n'];
				let select_tag = ['s', 'e', 'l', 'e', 'c', 't'];
				if is_equal_chars(&tag_name, &input_tag) {
					// an input element that with type 'checkbox' or 'radio'
					if let Some(IAttrValue::Value(input_type, _)) = ele.get_attribute("type") {
						let lower_input_type = input_type.to_ascii_lowercase();
						// when the input is checkbox or radio
						if lower_input_type == "checkbox" || lower_input_type == "radio" {
							return ele.has_attribute("checked");
						}
					}
					false
				} else if is_equal_chars(&tag_name, &option_tag) {
					let is_selected = ele.has_attribute("selected");
					if is_selected {
						// if the option tag has 'selected' attribute
						true
					} else {
						// check if is the default option
						// 1. under the parent 'select' element, without nested tags
						// 2. the 'select' is not a multiple select
						// 3. the 'option' is the first 'option' tag
						// 4. the 'select' has no selected 'option'
						if let Some(parent) = &ele.parent() {
							// check condition 1 & 2
							if is_equal_chars(&parent.tag_names(), &select_tag)
								&& !parent.has_attribute("multiple")
							{
								// check if is the first option, condition 3
								let mut prev = ele.previous_element_sibling();
								while let Some(prev_ele) = &prev {
									if is_equal_chars(&prev_ele.tag_names(), &option_tag) {
										return false;
									}
									prev = prev_ele.previous_element_sibling();
								}
								// check if the select has selected option, condition 4
								fn check_selected_option(ele: &BoxDynElement, option_tag: &[char]) -> bool {
									if is_equal_chars(&ele.tag_names(), option_tag) {
										return ele.has_attribute("selected");
									} else {
										// check the childs
										let total = ele.child_nodes_length();
										for index in 0..total {
											let node = ele
												.child_nodes_item(index)
												.expect("Child nodes item index must less than total");
											if matches!(node.node_type(), INodeType::Element) {
												let child = node
													.typed()
													.into_element()
													.expect("Call `typed` for element ele.");
												if check_selected_option(&child, option_tag) {
													return true;
												}
											}
										}
									}
									false
								}
								// check the next siblings
								let mut next = ele.next_element_sibling();
								while let Some(next_ele) = &next {
									if check_selected_option(next_ele, &option_tag) {
										return false;
									}
									next = next_ele.next_element_sibling();
								}
								// now the option is default option
								return true;
							}
						}
						// not selected option
						false
					}
				} else {
					// need loop the child
					false
				}
			})),
			..Default::default()
		}),
	);
	rules.push(rule.into());
}

/// pseudo selector: `:header`
fn pseudo_alias_header(rules: &mut Vec<RuleItem>) {
	let (selector, alias) = SELECTOR_ALIAS_NAME_HEADER;
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		Box::new(move |_| Rule::make_alias(alias)),
	);
	rules.push(rule.into());
}

/// pseudo selector: `:input`
fn pseudo_alias_input(rules: &mut Vec<RuleItem>) {
	let (selector, alias) = SELECTOR_ALIAS_NAME_INPUT;
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		Box::new(move |_| Rule::make_alias(alias)),
	);
	rules.push(rule.into());
}

/// pseudo selector: `:submit`
fn pseudo_alias_submit(rules: &mut Vec<RuleItem>) {
	let (selector, alias) = SELECTOR_ALIAS_NAME_SUBMIT;
	let name = selector;
	let rule = RuleDefItem(
		name,
		selector,
		PRIORITY,
		Box::new(move |_| Rule::make_alias(alias)),
	);
	rules.push(rule.into());
}

pub fn init(rules: &mut Vec<RuleItem>) {
	pseudo_root(rules);
	pseudo_empty(rules);
	// :first-child, :last-child
	pseudo_first_child(rules);
	pseudo_last_child(rules);
	// :only-child
	pseudo_only_child(rules);
	// :nth-child,:nth-last-child
	pseudo_nth_child(rules);
	pseudo_nth_last_child(rules);
	// :first-of-type,:last-of-type
	pseudo_first_of_type(rules);
	pseudo_last_of_type(rules);
	// :nth-of-type,:nth-last-of-type
	pseudo_nth_of_type(rules);
	pseudo_nth_last_of_type(rules);
	// :only-of-type
	pseudo_only_of_type(rules);
	// :not
	pseudo_not(rules);
	// :contains
	pseudo_contains(rules);
	// ---- jquery selectors -----
	// :has
	pseudo_has(rules);
	// :checked
	pseudo_checked(rules);
	// :header alias
	pseudo_alias_header(rules);
	// :input alias
	pseudo_alias_input(rules);
	// :submit alias
	pseudo_alias_submit(rules);
}
