use super::{BoxDynElement, IAttrValue, IElementTrait, InsertPosition, MaybeDoc, Texts};
use crate::mesdoc::{constants::ATTR_CLASS, error::Error as IError, utils::class_list_to_string};
use crate::mesdoc::{
	constants::DEF_NODES_LEN,
	selector::{
		rule::{MatchAllHandle, MatchOneHandle},
		Combinator, QueryProcess, Selector, SelectorSegment,
	},
};
use crate::mesdoc::{
	selector::rule::MatchSpecifiedHandle,
	utils::{get_class_list, retain_by_index, to_static_str},
};
use std::collections::HashSet;
use std::{
	cmp::Ordering,
	collections::VecDeque,
	ops::{Bound, RangeBounds},
};
use std::{collections::HashMap, error::Error};

// get the ele indexs in tree
fn get_tree_indexs(ele: &BoxDynElement) -> VecDeque<usize> {
	let mut indexs: VecDeque<usize> = VecDeque::with_capacity(DEF_NODES_LEN);
	fn loop_handle(ele: &BoxDynElement, indexs: &mut VecDeque<usize>) {
		indexs.push_front(ele.index());
		if let Some(parent) = &ele.parent() {
			loop_handle(parent, indexs);
		}
	}
	loop_handle(ele, &mut indexs);
	indexs
}

// compare indexs
fn compare_indexs(a: &VecDeque<usize>, b: &VecDeque<usize>) -> Ordering {
	let a_total = a.len();
	let b_total = b.len();
	let loop_total = if a_total > b_total { b_total } else { a_total };
	for i in 0..loop_total {
		let a_index = a[i];
		let b_index = b[i];
		match a_index.cmp(&b_index) {
			Ordering::Equal => continue,
			order => return order,
		}
	}
	a_total.cmp(&b_total)
}
enum ElementRelation {
	Ancestor,
	Equal,
	Descendant,
	Feauture,
}
// check if ancestor and descendants
fn relation_of(a: &VecDeque<usize>, b: &VecDeque<usize>) -> ElementRelation {
	let a_total = a.len();
	let b_total = b.len();
	let loop_total = if a_total > b_total { b_total } else { a_total };
	let mut equal_num = 0;
	for i in 0..loop_total {
		let a_index = a[i];
		let b_index = b[i];
		match a_index.cmp(&b_index) {
			Ordering::Equal => {
				equal_num += 1;
				continue;
			}
			_ => break,
		}
	}
	let a_left = a_total - equal_num;
	let b_left = b_total - equal_num;
	match (a_left == 0, b_left == 0) {
		(false, false) => ElementRelation::Feauture,
		(false, true) => ElementRelation::Descendant,
		(true, true) => ElementRelation::Equal,
		(true, false) => ElementRelation::Ancestor,
	}
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum FilterType {
	Filter,
	Not,
	Is,
	IsAll,
}

#[derive(Default)]
pub struct Elements<'a> {
	nodes: Vec<BoxDynElement<'a>>,
}

/*
*** Base methods
*/
impl<'a> Elements<'a> {
	/*-----------trigger method proxy---------------*/
	pub(crate) fn trigger_method<F, T: Default>(&self, method: &str, selector: &str, handle: F) -> T
	where
		F: Fn(&mut Selector) -> T,
	{
		if !self.is_empty() {
			// filter handles don't use lookup
			const USE_LOOKUP: bool = false;
			let s = Selector::from_str(selector, USE_LOOKUP);
			if let Ok(mut s) = s {
				return handle(&mut s);
			}
			self.trigger_method_throw_error(method, Box::new(s.unwrap_err()));
		}
		Default::default()
	}

	pub(crate) fn trigger_method_throw_error(&self, method: &str, error: Box<dyn Error>) {
		if let Some(doc) = &self
			.get(0)
			.expect("Use index 0 when length > 0")
			.owner_document()
		{
			doc.trigger_error(Box::new(IError::MethodOnInvalidSelector {
				method: String::from(method),
				error: error.to_string(),
			}));
		}
	}
	/*-----------create a elements----------------*/
	// new
	pub fn new() -> Self {
		Default::default()
	}
	// crate only methods
	pub(crate) fn with_node(ele: &BoxDynElement) -> Self {
		Elements {
			nodes: vec![ele.cloned()],
		}
	}
	// with nodes
	pub fn with_nodes(nodes: Vec<BoxDynElement<'a>>) -> Self {
		Elements { nodes }
	}

	// with capacity
	pub fn with_capacity(size: usize) -> Self {
		Elements {
			nodes: Vec::with_capacity(size),
		}
	}
	/*------------get/set element nodes---------------*/
	// get a element from the set
	pub fn get(&self, index: usize) -> Option<&BoxDynElement<'a>> {
		self.get_ref().get(index)
	}

	// get ref
	pub fn get_ref(&self) -> &Vec<BoxDynElement<'a>> {
		&self.nodes
	}

	// get mut ref
	pub(crate) fn get_mut_ref(&mut self) -> &mut Vec<BoxDynElement<'a>> {
		&mut self.nodes
	}
	// push node
	pub(crate) fn push(&mut self, ele: BoxDynElement<'a>) {
		self.get_mut_ref().push(ele);
	}
}

/*
*** Helper Methods
*/
impl<'a> Elements<'a> {
	// pub fn `for_each`
	pub fn for_each<F>(&mut self, mut handle: F) -> &mut Self
	where
		F: FnMut(usize, &mut BoxDynElement) -> bool,
	{
		for (index, ele) in self.get_mut_ref().iter_mut().enumerate() {
			if !handle(index, ele) {
				break;
			}
		}
		self
	}
	// alias for `for_each`
	pub fn each<F>(&mut self, handle: F) -> &mut Self
	where
		F: FnMut(usize, &mut BoxDynElement) -> bool,
	{
		self.for_each(handle)
	}
	// pub fn `map`
	pub fn map<F, T: Sized>(&self, mut handle: F) -> Vec<T>
	where
		F: FnMut(usize, &BoxDynElement) -> T,
	{
		let mut result: Vec<T> = Vec::with_capacity(self.length());
		for (index, ele) in self.get_ref().iter().enumerate() {
			result.push(handle(index, ele));
		}
		result
	}

	/// pub fn `length`
	pub fn length(&self) -> usize {
		self.nodes.len()
	}
	/// pub fn `is_empty`
	pub fn is_empty(&self) -> bool {
		self.length() == 0
	}
	/// pub fn `document`, a quick way to get document
	pub fn document(&self) -> MaybeDoc {
		for ele in self.get_ref() {
			if let Some(doc) = ele.owner_document() {
				return Some(doc);
			}
		}
		None
	}
}

/*
*** Sort and Uniqueues
**  sort elements and unique elements
*/
impl<'a> Elements<'a> {
	// keep one sibling, first<asc:true> or last<asc:false>
	fn unique_sibling(&self, asc: bool) -> Elements<'a> {
		let total = self.length();
		let mut parents_indexs: HashSet<VecDeque<usize>> = HashSet::with_capacity(total);
		let mut uniques: Vec<BoxDynElement> = Vec::with_capacity(total);
		let mut prev_parent: Option<BoxDynElement> = None;
		let mut has_root = false;
		let mut handle = |ele: &BoxDynElement| {
			if let Some(parent) = ele.parent() {
				if let Some(prev_parent) = &prev_parent {
					if parent.is(prev_parent) {
						return;
					}
				}
				// parents
				let indexs = get_tree_indexs(&parent);
				// set prev parent
				prev_parent = Some(parent);
				// new parent
				if parents_indexs.get(&indexs).is_none() {
					parents_indexs.insert(indexs);
					uniques.push(ele.cloned());
				}
			} else if !has_root {
				has_root = true;
				uniques.push(ele.cloned());
			}
		};
		// just keep one sibling node
		if asc {
			for ele in self.get_ref() {
				handle(ele);
			}
		} else {
			for ele in self.get_ref().iter().rev() {
				handle(ele)
			}
			// reverse, keep the order
			uniques.reverse();
		}
		Elements::with_nodes(uniques)
	}
	// keep first sibling
	fn unique_sibling_first(&self) -> Elements<'a> {
		self.unique_sibling(true)
	}
	// keep last sibling
	fn unique_sibling_last(&self) -> Elements<'a> {
		self.unique_sibling(false)
	}
	// keep siblings
	fn unique_all_siblings(&self) -> Vec<(BoxDynElement<'a>, bool)> {
		// should first unique siblings
		// if have two siblings, then use parent.children
		let total = self.length();
		let mut parents_indexs: HashMap<VecDeque<usize>, (usize, bool)> = HashMap::with_capacity(total);
		let mut uniques: Vec<(BoxDynElement, bool)> = Vec::with_capacity(total);
		let mut parents: HashMap<usize, BoxDynElement> = HashMap::with_capacity(total);
		// just keep one sibling node
		for ele in self.get_ref() {
			if let Some(parent) = ele.parent() {
				// parent indexs
				let indexs = get_tree_indexs(&parent);
				if let Some((index, changed)) = parents_indexs.get_mut(&indexs) {
					if !*changed {
						let index = *index;
						*changed = true;
						uniques[index] = (
							parents
								.remove(&index)
								.expect("When call `unique_all_siblings`, the parents variable's index must exist"),
							true,
						);
					}
				} else {
					let cur_index = uniques.len();
					parents_indexs.insert(indexs, (cur_index, false));
					uniques.push((ele.cloned(), false));
					parents.insert(cur_index, parent);
				}
			}
		}
		uniques
	}
	// unique parent, keep the top parent
	fn unique_parents(&self) -> Elements<'a> {
		// keep only top parent
		let mut ancestors: Vec<(VecDeque<usize>, &BoxDynElement)> = Vec::with_capacity(self.length());
		for ele in self.get_ref() {
			let ele_indexs = get_tree_indexs(ele);
			let cur_len = ancestors.len();
			if cur_len > 0 {
				// just check the last ancestor
				let (top_ele_indexs, _) = &ancestors[cur_len - 1];
				match relation_of(&ele_indexs, top_ele_indexs) {
					ElementRelation::Feauture => {
						ancestors.push((ele_indexs, ele));
					}
					ElementRelation::Descendant => {}
					_ => unreachable!("The elements call `unique_parents` is not ordered"),
				}
			} else {
				ancestors.push((ele_indexs, ele));
			}
		}
		let mut result = Elements::with_capacity(ancestors.len());
		for (_, ele) in ancestors {
			result.push(ele.cloned());
		}
		result
	}
	// sort
	fn sort(&mut self) {
		self.get_mut_ref().sort_by(|a, b| {
			let a_index = get_tree_indexs(a);
			let b_index = get_tree_indexs(b);
			compare_indexs(&a_index, &b_index)
		});
	}
	// unique
	fn unique(&mut self) {
		self.get_mut_ref().dedup_by(|a, b| a.is(b));
	}
	// sort then unique
	fn sort_and_unique(&mut self) {
		self.sort();
		self.unique();
	}
}

/*
*** Selector APIs
**  [methods]
*/
impl<'a> Elements<'a> {
	// for all combinator selectors
	fn select_with_comb(&self, method: &str, selector: &str, comb: Combinator) -> Elements<'a> {
		if selector.is_empty() {
			let segment = Selector::make_comb_all(comb);
			let selector = Selector::from_segment(segment);
			return self.find_selector(&selector);
		}
		self.trigger_method(method, selector, |selector| {
			selector.head_combinator(comb);
			self.find_selector(&selector)
		})
	}
	// for all combinator until selectors
	fn select_with_comb_until(
		&self,
		method: &str,
		selector: &str,
		filter: &str,
		contains: bool,
		comb: Combinator,
	) -> Elements<'a> {
		let selector = selector.parse::<Selector>();
		if let Ok(selector) = &selector {
			let segment = Selector::make_comb_all(comb);
			let next_selector = Selector::from_segment(segment);
			let mut result = Elements::with_capacity(DEF_NODES_LEN);
			let (next_ok, filter) = if !filter.is_empty() {
				let filter = filter.parse::<Selector>();
				if let Ok(filter) = filter {
					(true, Some(filter))
				} else {
					self.trigger_method_throw_error(method, Box::new(filter.unwrap_err()));
					(false, None)
				}
			} else {
				(true, None)
			};
			if next_ok {
				// has filter
				for ele in self.get_ref() {
					let mut cur_eles = Elements::with_node(ele);
					loop {
						// find the next element
						cur_eles = cur_eles.find_selector(&next_selector);
						if !cur_eles.is_empty() {
							let meet_until = cur_eles.filter_type_handle(&selector, &FilterType::Is).1;
							// meet the until element, and not contains, stop before check element
							if meet_until && !contains {
								break;
							}
							// check if cur_eles filter
							let should_add = if let Some(filter) = &filter {
								// filter true
								cur_eles.filter_type_handle(filter, &FilterType::Is).1
							} else {
								// no need filter, just add
								true
							};
							if should_add {
								result.push(
									cur_eles
										.get(0)
										.expect("Elements get 0 must have when length > 0")
										.cloned(),
								);
							}
							// find the until, stop the loop at the end whenever contains or not
							if meet_until {
								break;
							}
						} else {
							break;
						}
					}
				}
				return result;
			}
		} else {
			self.trigger_method_throw_error(method, Box::new(selector.unwrap_err()));
		}
		Elements::new()
	}

	// prev
	pub fn prev(&self, selector: &str) -> Elements<'a> {
		self.select_with_comb("prev", selector, Combinator::Prev)
	}
	// prev_all
	pub fn prev_all(&self, selector: &str) -> Elements<'a> {
		let uniques = self.unique_sibling_last();
		uniques.select_with_comb("prev_all", selector, Combinator::PrevAll)
	}
	// prev_until
	pub fn prev_until(&self, selector: &str, filter: &str, contains: bool) -> Elements<'a> {
		let uniques = self.unique_sibling_last();
		uniques.select_with_comb_until("prev_until", selector, filter, contains, Combinator::Prev)
	}
	// next
	pub fn next(&self, selector: &str) -> Elements<'a> {
		self.select_with_comb("next", selector, Combinator::Next)
	}
	// next_all
	pub fn next_all(&self, selector: &str) -> Elements<'a> {
		// unique, keep the first sibling node
		let uniques = self.unique_sibling_first();
		uniques.select_with_comb("next_all", selector, Combinator::NextAll)
	}
	// next_until
	pub fn next_until(&self, selector: &str, filter: &str, contains: bool) -> Elements<'a> {
		// unique, keep the first sibling node
		let uniques = self.unique_sibling_first();
		uniques.select_with_comb_until("next_until", selector, filter, contains, Combinator::Next)
	}

	// siblings
	pub fn siblings(&self, selector: &str) -> Elements<'a> {
		let uniques = self.unique_all_siblings();
		// when selector is empty or only
		let mut siblings_selector: Selector;
		let siblings_comb = Combinator::Siblings;
		let mut child_selector: Selector;
		let child_comb = Combinator::Children;
		let selector = selector.trim();
		if selector.is_empty() {
			siblings_selector = Selector::from_segment(Selector::make_comb_all(siblings_comb));
			child_selector = Selector::from_segment(Selector::make_comb_all(child_comb));
		} else {
			// self
			let sib_selector = selector.parse::<Selector>();
			if let Ok(sib_selector) = sib_selector {
				// clone the selector to a child selector
				child_selector = selector
					.parse::<Selector>()
					.expect("The selector has detected");
				child_selector.head_combinator(child_comb);
				// use siblings selector
				siblings_selector = sib_selector;
				siblings_selector.head_combinator(siblings_comb);
			} else {
				self.trigger_method_throw_error(
					"siblings",
					Box::new(IError::InvalidTraitMethodCall {
						method: "siblings".to_string(),
						message: format!(
							"Invalid selector:{}",
							sib_selector.err().expect("Selector parse error")
						),
					}),
				);
				return Elements::new();
			}
		}
		// uniques
		let mut result = Elements::with_capacity(DEF_NODES_LEN);
		for (ele, is_parent) in &uniques {
			let eles = Elements::with_node(ele);
			let finded = if *is_parent {
				eles.find_selector(&child_selector)
			} else {
				eles.find_selector(&siblings_selector)
			};
			result.get_mut_ref().extend(finded);
		}
		// sort the result
		result.sort();
		result
	}
	// children
	pub fn children(&self, selector: &str) -> Elements<'a> {
		self.select_with_comb("children", selector, Combinator::Children)
	}

	// parent
	pub fn parent(&self, selector: &str) -> Elements<'a> {
		// unique, keep the first sibling node
		let uniques = self.unique_sibling_first();
		uniques.select_with_comb("parent", selector, Combinator::Parent)
	}
	// parents
	pub fn parents(&self, selector: &str) -> Elements<'a> {
		// unique, keep the first sibling node
		let uniques = self.unique_sibling_first();
		let mut result = uniques.select_with_comb("parents", selector, Combinator::ParentAll);
		result.sort_and_unique();
		result
	}
	// parents_until
	pub fn parents_until(&self, selector: &str, filter: &str, contains: bool) -> Elements<'a> {
		// unique, keep the first sibling node
		let uniques = self.unique_sibling_first();
		let mut result = uniques.select_with_comb_until(
			"parents_until",
			selector,
			filter,
			contains,
			Combinator::Parent,
		);
		result.sort_and_unique();
		result
	}
	// closest
	pub fn closest(&self, selector: &str) -> Elements<'a> {
		// when selector is not provided
		if selector.is_empty() {
			return Elements::new();
		}
		// find the nearst node
		const METHOD: &str = "closest";
		let selector = selector.parse::<Selector>();
		if let Ok(selector) = selector {
			let total = self.length();
			let mut result = Elements::with_capacity(total);
			let mut propagations = Elements::with_capacity(total);
			for ele in self.get_ref() {
				let mut cur_eles = Elements::with_node(ele);
				if cur_eles.filter_type_handle(&selector, &FilterType::Is).1 {
					// check self
					result.get_mut_ref().push(cur_eles.get_mut_ref().remove(0));
				} else {
					propagations
						.get_mut_ref()
						.push(cur_eles.get_mut_ref().remove(0));
				}
			}
			if !propagations.is_empty() {
				let uniques = propagations.unique_sibling_first();
				for ele in uniques.get_ref() {
					let mut cur_eles = Elements::with_node(ele);
					loop {
						if cur_eles.filter_type_handle(&selector, &FilterType::Is).1 {
							result.get_mut_ref().push(cur_eles.get_mut_ref().remove(0));
							break;
						}
						if let Some(parent) = &cur_eles
							.get(0)
							.expect("Elements must have one node")
							.parent()
						{
							if !parent.is_root_element() {
								cur_eles = Elements::with_node(parent);
							} else {
								break;
							}
						} else {
							break;
						}
					}
				}
				// need sort and unique
				result.sort_and_unique();
			}
			result
		} else {
			self.trigger_method_throw_error(METHOD, Box::new(selector.unwrap_err()));
			Elements::new()
		}
	}
	// for `find` and `select_with_comb`
	fn find_selector(&self, selector: &Selector) -> Elements<'a> {
		let mut result = Elements::with_capacity(DEF_NODES_LEN);
		if !self.is_empty() {
			for p in &selector.process {
				let QueryProcess { should_in, query } = p;
				let first_query = &query[0];
				let mut group: Elements = Elements::with_capacity(DEF_NODES_LEN);
				if let Some(lookup) = should_in {
					// find the first query elements
					let finded = Elements::select(self, first_query, Some(&Combinator::ChildrenAll));
					if !finded.is_empty() {
						let first_comb = &first_query[0].1;
						// check the elements if satisfied the lookup
						for ele in finded.get_ref() {
							if self.has_ele(ele, first_comb, Some(&lookup)) {
								group.push(ele.cloned());
							}
						}
					}
				} else {
					// find the first query elements
					group = Elements::select(self, first_query, None);
				}
				if !group.is_empty() {
					let mut need_combine = true;
					if query.len() > 1 {
						for rules in &query[1..] {
							group = Elements::select(&group, rules, None);
							if group.is_empty() {
								need_combine = false;
								break;
							}
						}
					}
					if need_combine {
						result = result.add(group);
					}
				}
			}
		}
		result
	}

	/// pub fn `find`
	/// get elements by selector, support standard css selectors
	pub fn find(&self, selector: &str) -> Elements<'a> {
		let s = Selector::from_str(selector, true);
		if let Ok(selector) = &s {
			return self.find_selector(selector);
		}
		self.trigger_method_throw_error("find", Box::new(s.unwrap_err()));
		Elements::new()
	}
	// select one rule
	// the rule must not in cache
	fn select_by_rule(
		elements: &Elements<'a>,
		rule_item: &SelectorSegment,
		comb: Option<&Combinator>,
	) -> Elements<'a> {
		let cur_comb = comb.unwrap_or(&rule_item.1);
		let (matcher, ..) = rule_item;
		let mut result = Elements::with_capacity(DEF_NODES_LEN);
		use Combinator::*;
		match cur_comb {
			ChildrenAll => {
				// unique if have ancestor and descendant relation elements
				// check if one handle, match one by one
				if let Some(handle) = &matcher.one_handle {
					let exec = |ele: &dyn IElementTrait, result: &mut Elements| {
						fn loop_handle(
							ele: &dyn IElementTrait,
							result: &mut Elements,
							handle: &MatchOneHandle,
						) {
							ele.children_by(Box::new(|child| {
								if handle(child, None) {
									result.get_mut_ref().push(child.cloned());
								}
								if child.child_nodes_length() > 0 {
									loop_handle(child, result, handle);
								}
							}));
						}
						loop_handle(ele, result, handle)
					};
					if elements.length() > 1 {
						let uniques = elements.unique_parents();
						// depth first search, keep the appear order
						for ele in uniques.get_ref() {
							exec(&**ele, &mut result);
						}
					} else {
						for ele in elements.get_ref() {
							exec(&**ele, &mut result);
						}
					}
				} else {
					// specified first, just select the child
					if let Some(handle) = &matcher.specified_handle {
						let exec = |ele: &dyn IElementTrait, result: &mut Elements| {
							fn loop_handle(
								ele: &dyn IElementTrait,
								result: &mut Elements,
								handle: &MatchSpecifiedHandle,
							) {
								handle(
									ele,
									Box::new(|child, is_matched| {
										if is_matched {
											result.get_mut_ref().push(child.cloned());
										}
										if child.child_nodes_length() > 0 {
											loop_handle(child, result, handle);
										}
									}),
								);
							}
							loop_handle(ele, result, handle);
						};
						if elements.length() > 1 {
							let uniques = elements.unique_parents();
							// get elements
							for ele in uniques.get_ref() {
								exec(&**ele, &mut result);
							}
						} else {
							// get elements
							for ele in elements.get_ref() {
								exec(&**ele, &mut result);
							}
						}
					} else {
						// if all handle, check childrens once all
						let handle = matcher.get_all_handle();
						let exec = |ele: &BoxDynElement, result: &mut Elements| {
							// get children
							fn loop_handle(ele: &BoxDynElement, result: &mut Elements, handle: &MatchAllHandle) {
								let childs = ele.children();
								if !childs.is_empty() {
									// apply rule
									let matched_childs = handle(&childs, Some(false));
									let matched_childs = matched_childs.get_ref();
									let total_matched = matched_childs.len();
									let mut cmp_index = 0;
									for child in childs.get_ref() {
										if cmp_index < total_matched {
											let cmp_child = &matched_childs[cmp_index];
											if child.is(&cmp_child) {
												cmp_index += 1;
												result.get_mut_ref().push(child.cloned());
											}
										}
										// loop for sub childs
										if child.child_nodes_length() > 0 {
											loop_handle(child, result, handle);
										}
									}
								}
							}
							loop_handle(ele, result, handle)
						};
						if elements.length() > 1 {
							let uniques = elements.unique_parents();
							// get elements
							for ele in uniques.get_ref() {
								exec(ele, &mut result);
							}
						} else {
							// get elements
							for ele in elements.get_ref() {
								exec(ele, &mut result);
							}
						}
					}
				};
			}
			Children => {
				// because elements is unique, so the children is unique too
				if let Some(handle) = &matcher.one_handle {
					for ele in elements.get_ref() {
						ele.children_by(Box::new(|child| {
							if handle(child, None) {
								result.get_mut_ref().push(child.cloned());
							}
						}));
					}
				} else {
					// specified first
					if let Some(handle) = &matcher.specified_handle {
						for ele in elements.get_ref() {
							handle(
								&**ele,
								Box::new(|ele, is_matched| {
									if is_matched {
										result.get_mut_ref().push(ele.cloned());
									}
								}),
							);
						}
					} else {
						let handle = matcher.get_all_handle();
						for ele in elements.get_ref() {
							let childs = ele.children();
							let match_childs = handle(&childs, Some(false));
							if !match_childs.is_empty() {
								result.get_mut_ref().extend(match_childs);
							}
						}
					}
				}
			}
			Parent => {
				// elements is unique, but may be siblings
				// so they maybe has equal parent, just keep only one
				let uniques = elements.unique_sibling_first();
				if let Some(handle) = &matcher.one_handle {
					for ele in uniques.get_ref() {
						if let Some(parent) = ele.parent() {
							if !parent.is_root_element() && handle(&*parent, None) {
								result.get_mut_ref().push(parent);
							}
						}
					}
				} else {
					let handle = matcher.get_all_handle();
					let mut parents = Elements::with_capacity(uniques.length());
					for ele in uniques.get_ref() {
						if let Some(parent) = ele.parent() {
							if !parent.is_root_element() {
								parents.push(parent);
							}
						}
					}
					let matched_parents = handle(&parents, None);
					if !matched_parents.is_empty() {
						result.get_mut_ref().extend(matched_parents);
					}
				}
			}
			ParentAll => {
				if let Some(handle) = &matcher.one_handle {
					let exec = |ele: &BoxDynElement, result: &mut Elements| {
						fn loop_handle(ele: &BoxDynElement, result: &mut Elements, handle: &MatchOneHandle) {
							if let Some(parent) = ele.parent() {
								if !parent.is_root_element() {
									// try to find ancestor first
									// because ancestor appear early than parent, keep the order
									loop_handle(&parent, result, handle);
									// check parent
									if handle(&*parent, None) {
										result.get_mut_ref().push(parent);
									}
								}
							}
						}
						loop_handle(ele, result, handle);
					};
					// loop the elements
					for ele in elements.get_ref() {
						exec(ele, &mut result);
					}
					// maybe not unique, need sort and unique
					result.sort_and_unique();
				} else {
					// gather all parents
					fn loop_handle(ele: &BoxDynElement, parents: &mut Elements) {
						if let Some(parent) = ele.parent() {
							if !parent.is_root_element() {
								// add ancestor first
								loop_handle(&parent, parents);
								// add parent
								parents.push(parent);
							}
						}
					}
					let mut all_parents = Elements::with_capacity(10);
					for ele in elements.get_ref() {
						loop_handle(ele, &mut all_parents);
					}
					// unique all parents;
					all_parents.sort_and_unique();
					// check if matched
					let handle = matcher.get_all_handle();
					let matched_parents = handle(&all_parents, None);
					if !matched_parents.is_empty() {
						result.get_mut_ref().extend(matched_parents);
					}
				}
			}
			NextAll => {
				// unique siblings just keep first
				let uniques = elements.unique_sibling_first();
				for ele in uniques.get_ref() {
					let nexts = ele.next_element_siblings();
					let matched_nexts = matcher.apply(&nexts, None);
					if !matched_nexts.is_empty() {
						result.get_mut_ref().extend(matched_nexts);
					}
				}
			}
			Next => {
				// because elements is unique, so the next is unique too
				let mut nexts = Elements::with_capacity(elements.length());
				if let Some(handle) = &matcher.one_handle {
					for ele in elements.get_ref() {
						if let Some(next) = ele.next_element_sibling() {
							if handle(&*next, None) {
								nexts.push(next);
							}
						}
					}
					result = nexts;
				} else {
					for ele in elements.get_ref() {
						if let Some(next) = ele.next_element_sibling() {
							nexts.push(next);
						}
					}
					result = matcher.apply(&nexts, None);
				}
			}
			PrevAll => {
				// unique siblings just keep last
				let uniques = elements.unique_sibling_last();
				for ele in uniques.get_ref() {
					let nexts = ele.previous_element_siblings();
					result.get_mut_ref().extend(matcher.apply(&nexts, None));
				}
			}
			Prev => {
				// because elements is unique, so the prev is unique too
				let mut prevs = Elements::with_capacity(elements.length());
				if let Some(handle) = &matcher.one_handle {
					for ele in elements.get_ref() {
						if let Some(prev) = ele.previous_element_sibling() {
							if handle(&*prev, None) {
								prevs.push(prev);
							}
						}
					}
					result = prevs;
				} else {
					for ele in elements.get_ref() {
						if let Some(prev) = ele.previous_element_sibling() {
							prevs.push(prev);
						}
					}
					result = matcher.apply(&prevs, None);
				}
			}
			Siblings => {
				// siblings
				// unique first
				let uniques = elements.unique_all_siblings();
				for (ele, is_parent) in uniques {
					let eles = if !is_parent {
						ele.siblings()
					} else {
						ele.children()
					};
					result.get_mut_ref().extend(matcher.apply(&eles, None));
				}
				/*
				for ele in elements.get_ref() {
					let siblings = ele.siblings();
					result.get_mut_ref().extend(matcher.apply(&siblings, None));
				}
				// not unique, need sort and unique
				result.sort_and_unique();
				*/
			}
			Chain => {
				// just filter
				result = matcher.apply(&elements, None);
			}
		};
		result
	}
	// select ele by rules
	fn select(
		elements: &Elements<'a>,
		rules: &[SelectorSegment],
		comb: Option<&Combinator>,
	) -> Elements<'a> {
		let first_rule = &rules[0];
		let comb = comb.unwrap_or(&first_rule.1);
		let mut elements = if first_rule.0.in_cache && matches!(comb, Combinator::ChildrenAll) {
			let (matcher, ..) = first_rule;
			// set use cache true
			let cached = matcher.apply(&elements, Some(true));
			let count = cached.length();
			if count > 0 {
				let mut result = Elements::with_capacity(count);
				for ele in cached.get_ref() {
					if elements.has_ele(ele, comb, None) {
						result.push(ele.cloned());
					}
				}
				result.sort_and_unique();
				result
			} else {
				Elements::new()
			}
		} else {
			Elements::select_by_rule(&elements, first_rule, Some(comb))
		};
		if !elements.is_empty() && rules.len() > 1 {
			for rule in &rules[1..] {
				elements = Elements::select_by_rule(&elements, rule, None);
				if elements.is_empty() {
					break;
				}
			}
		}
		elements
	}
	// cloned
	pub fn cloned(&self) -> Elements<'a> {
		let mut result = Elements::with_capacity(self.length());
		for ele in &self.nodes {
			result.push(ele.cloned());
		}
		result
	}
	// `has_ele`
	pub(crate) fn has_ele(
		&self,
		ele: &BoxDynElement,
		comb: &Combinator,
		lookup: Option<&[Vec<SelectorSegment>]>,
	) -> bool {
		let mut elements = Elements::with_node(ele);
		let mut lookup_comb = comb.reverse();
		if let Some(lookup) = lookup {
			for rules in lookup.iter().rev() {
				let finded = Elements::select(&elements, rules, Some(&lookup_comb));
				if finded.is_empty() {
					return false;
				}
				lookup_comb = rules[0].1.reverse();
				elements = finded;
			}
		}
		use Combinator::*;
		match lookup_comb {
			Parent => {
				for ele in elements.get_ref() {
					if let Some(parent) = &ele.parent() {
						if self.includes(&parent) {
							return true;
						}
					}
				}
			}
			ParentAll => {
				for ele in elements.get_ref() {
					if let Some(parent) = &ele.parent() {
						if self.includes(&parent) {
							return true;
						}
						if let Some(ancestor) = &parent.parent() {
							if self.includes(&ancestor) {
								return true;
							}
							// iterator the search process, becareful with the combinator now is ChildrenAll
							// the sentences must in if condition, otherwise it will break the for loop
							if self.has_ele(&ancestor, &Combinator::ChildrenAll, None) {
								return true;
							}
						}
					}
				}
			}
			Prev => {
				for ele in elements.get_ref() {
					if let Some(prev) = &ele.previous_element_sibling() {
						if self.includes(&prev) {
							return true;
						}
					}
				}
			}
			PrevAll => {
				for ele in elements.get_ref() {
					let prevs = ele.previous_element_siblings();
					for prev in prevs.get_ref() {
						if self.includes(prev) {
							return true;
						}
					}
				}
			}
			Chain => {
				for ele in elements.get_ref() {
					if self.includes(ele) {
						return true;
					}
				}
			}
			_ => panic!("Unsupported lookup combinator:{:?}", comb),
		};
		false
	}
	// filter_type_handle:
	// type     | rule processes
	// ----------------------------------------
	// Filter   | merge all elmements which matched each process
	// Not      | merge all elmements which matched each process, then exclude them all.
	// Is       | once matched a process, break
	// IsAll    | merge all elmements which matched each process, check if the matched equal to self
	pub(crate) fn filter_type_handle(
		&self,
		selector: &Selector,
		filter_type: &FilterType,
	) -> (Elements<'a>, bool) {
		let eles = self.get_ref();
		let total = eles.len();
		let mut result = Elements::with_capacity(total);
		let mut all_matched = false;
		let chain_comb = Combinator::Chain;
		let mut root: Option<Elements> = None;
		for process in selector.process.iter() {
			// filter methods make sure do not use `should_in`
			let QueryProcess { query, .. } = process;
			let query_num = query.len();
			let mut filtered = Elements::new();
			if query_num > 0 {
				let last_query = &query[query_num - 1];
				let last_query_first_rule = &last_query[0];
				filtered =
					Elements::select_by_rule(self, last_query_first_rule, Some(&chain_comb)).cloned();
				if !filtered.is_empty() && last_query.len() > 1 {
					for rule in &last_query[1..] {
						filtered = Elements::select_by_rule(&filtered, rule, None);
						if filtered.is_empty() {
							break;
						}
					}
				}
				if !filtered.is_empty() && query_num > 1 {
					// set root first
					root = root.or_else(|| {
						let cur_first = filtered.get(0).expect("Filtered length greater than 0");
						let root_element = cur_first
							.root_element()
							.unwrap_or_else(|| cur_first.cloned());
						Some(Elements::with_node(&root_element))
					});
					// get root elements
					let root_eles = root.as_ref().expect("root element must have");
					// find elements from root_eles by selector
					let lookup = Some(&query[..query_num - 1]);
					let mut lasts = Elements::with_capacity(filtered.length());
					let comb = &last_query_first_rule.1;
					for ele in filtered.get_ref() {
						if root_eles.has_ele(ele, comb, lookup) {
							lasts.get_mut_ref().push(ele.cloned());
						}
					}
					filtered = lasts;
				}
			}
			if !filtered.is_empty() {
				match filter_type {
					FilterType::Is => {
						all_matched = true;
						break;
					}
					_ => {
						result = result.add(filtered);
					}
				}
			}
		}
		match filter_type {
			FilterType::IsAll => {
				all_matched = result.length() == total;
			}
			FilterType::Not => {
				// Exclude `filtered` from self
				if result.is_empty() {
					// no element matched the not selector
					result = self.cloned();
				} else {
					// filtered by not in
					result = self.not_in(&result);
				}
			}
			_ => {
				// FilterType::Is: just return 'all_matched'
				// FilterType::Filter: just return 'result'
			}
		}
		(result, all_matched)
	}

	// filter in type
	fn filter_in_handle(&self, search: &Elements, filter_type: FilterType) -> (Elements<'a>, bool) {
		let eles = self.get_ref();
		let total = eles.len();
		let mut result = Elements::with_capacity(total);
		let mut all_matched = false;
		match filter_type {
			FilterType::Filter => {
				let mut start_index = 0;
				let search_total = search.length();
				for ele in eles {
					if let Some(index) = search.index_of(ele, start_index) {
						// also in search, include
						start_index = index + 1;
						result.push(ele.cloned());
						if start_index >= search_total {
							break;
						}
					}
				}
			}
			FilterType::Not => {
				let mut start_index = 0;
				for ele in eles {
					if let Some(index) = search.index_of(ele, start_index) {
						// also in search, exclude
						start_index = index + 1;
					} else {
						result.push(ele.cloned());
					}
				}
			}
			FilterType::Is => {
				for ele in eles {
					if search.includes(ele) {
						all_matched = true;
						break;
					}
				}
			}
			FilterType::IsAll => {
				if total <= search.length() {
					let mut is_all_matched = true;
					let mut start_index = 0;
					for ele in eles {
						if let Some(index) = search.index_of(ele, start_index) {
							// also in search, exclude
							start_index = index + 1;
						} else {
							is_all_matched = false;
							break;
						}
					}
					all_matched = is_all_matched;
				}
			}
		}
		(result, all_matched)
	}

	// filter
	pub fn filter(&self, selector: &str) -> Elements<'a> {
		const METHOD: &str = "filter";
		self.trigger_method(METHOD, selector, |selector| {
			self.filter_type_handle(&selector, &FilterType::Filter).0
		})
	}

	// filter_by
	pub fn filter_by<F>(&self, handle: F) -> Elements<'a>
	where
		F: Fn(usize, &BoxDynElement) -> bool,
	{
		let mut result = Elements::with_capacity(self.length());
		for (index, ele) in self.get_ref().iter().enumerate() {
			if handle(index, ele) {
				// find the ele, allow cloned
				result.push(ele.cloned());
			}
		}
		result
	}

	// filter in
	pub fn filter_in(&self, search: &Elements) -> Elements<'a> {
		self.filter_in_handle(search, FilterType::Filter).0
	}

	// is
	pub fn is(&self, selector: &str) -> bool {
		const METHOD: &str = "is";
		self.trigger_method(METHOD, selector, |selector| {
			self.filter_type_handle(selector, &FilterType::Is).1
		})
	}

	// is by
	pub fn is_by<F>(&self, handle: F) -> bool
	where
		F: Fn(usize, &BoxDynElement) -> bool,
	{
		let mut flag = false;
		for (index, ele) in self.get_ref().iter().enumerate() {
			if handle(index, ele) {
				flag = true;
				break;
			}
		}
		flag
	}

	// is in
	pub fn is_in(&self, search: &Elements) -> bool {
		self.filter_in_handle(search, FilterType::Is).1
	}

	// is_all
	pub fn is_all(&self, selector: &str) -> bool {
		const METHOD: &str = "is_all";
		self.trigger_method(METHOD, selector, |selector| {
			self.filter_type_handle(&selector, &FilterType::IsAll).1
		})
	}

	// is_all_by
	pub fn is_all_by<F>(&self, handle: F) -> bool
	where
		F: Fn(usize, &BoxDynElement) -> bool,
	{
		let mut flag = true;
		for (index, ele) in self.get_ref().iter().enumerate() {
			if !handle(index, ele) {
				flag = false;
				break;
			}
		}
		flag
	}

	// is_all_in
	pub fn is_all_in(&self, search: &Elements) -> bool {
		self.filter_in_handle(search, FilterType::IsAll).1
	}

	// not
	pub fn not(&self, selector: &str) -> Elements<'a> {
		const METHOD: &str = "not";
		self.trigger_method(METHOD, selector, |selector| {
			self.filter_type_handle(&selector, &FilterType::Not).0
		})
	}

	// not by
	pub fn not_by<F>(&self, handle: F) -> Elements<'a>
	where
		F: Fn(usize, &BoxDynElement) -> bool,
	{
		let mut result = Elements::with_capacity(self.length());
		for (index, ele) in self.get_ref().iter().enumerate() {
			if !handle(index, ele) {
				result.push(ele.cloned());
			}
		}
		result
	}

	/// pub fn `not_in`
	/// remove element from `Self` which is also in `search`
	pub fn not_in(&self, search: &Elements) -> Elements<'a> {
		self.filter_in_handle(search, FilterType::Not).0
	}

	// has
	pub fn has(&self, selector: &str) -> Elements<'a> {
		const METHOD: &str = "has";
		fn loop_handle(ele: &BoxDynElement, selector: &Selector) -> bool {
			let childs = ele.children();
			if !childs.is_empty() {
				let (_, all_matched) = childs.filter_type_handle(selector, &FilterType::Is);
				if all_matched {
					return true;
				}
				for child in childs.get_ref() {
					if loop_handle(child, selector) {
						return true;
					}
				}
			}
			false
		}
		self.trigger_method(METHOD, selector, |selector| {
			self.filter_by(|_, ele| loop_handle(ele, selector))
		})
	}

	// has_in
	pub fn has_in(&self, search: &Elements) -> Elements<'a> {
		fn loop_handle(ele: &BoxDynElement, search: &Elements) -> bool {
			let childs = ele.children();
			if !childs.is_empty() {
				let (_, all_matched) = childs.filter_in_handle(search, FilterType::Is);
				if all_matched {
					return true;
				}
				for child in childs.get_ref() {
					if loop_handle(child, search) {
						return true;
					}
				}
			}
			false
		}
		self.filter_by(|_, ele| loop_handle(ele, &search))
	}
}

/*
*** Other Selector and Helper APIs
**  [Methods]
*/
impl<'a> Elements<'a> {
	/// pub fn `eq`
	/// get a element by index
	pub fn eq(&self, index: usize) -> Elements<'a> {
		if let Some(ele) = self.get(index) {
			Elements::with_node(ele)
		} else {
			Elements::new()
		}
	}

	/// pub fn `first`
	/// get the first element, alias for 'eq(0)'
	pub fn first(&self) -> Elements<'a> {
		self.eq(0)
	}

	/// pub fn `last`
	/// get the last element, alias for 'eq(len - 1)'
	pub fn last(&self) -> Elements<'a> {
		self.eq(self.length() - 1)
	}

	/// pub fn `slice`
	/// get elements by a range parameter
	/// `slice(0..1)` equal to `eq(0)`, `first`
	pub fn slice<T: RangeBounds<usize>>(&self, range: T) -> Elements<'a> {
		let mut start = 0;
		let mut end = self.length();
		match range.start_bound() {
			Bound::Unbounded => {
				// start = 0
			}
			Bound::Included(&cur_start) => {
				if cur_start < end {
					start = cur_start;
				} else {
					// empty
					return Elements::new();
				}
			}
			_ => {
				// start bound not have exclude
			}
		};
		match range.end_bound() {
			Bound::Unbounded => {
				// end = total
			}
			Bound::Excluded(&cur_end) => {
				if cur_end < end {
					end = cur_end;
				}
			}
			Bound::Included(&cur_end) => {
				let cur_end = cur_end + 1;
				if cur_end < end {
					end = cur_end;
				}
			}
		}
		let mut result = Elements::with_capacity(end - start);
		let eles = self.get_ref();
		for ele in &eles[start..end] {
			result.push(ele.cloned());
		}
		result
	}

	/// pub fn `add`
	/// concat two element set to a new set,
	/// it will take the owership of the parameter element set, but no sence to `Self`
	pub fn add(&self, eles: Elements<'a>) -> Elements<'a> {
		if self.is_empty() {
			return eles;
		}
		if eles.is_empty() {
			return self.cloned();
		}
		let first_eles = self;
		let second_eles = &eles;
		// compare first and second
		let first_count = first_eles.length();
		let second_count = second_eles.length();
		let mut mids: Vec<(usize, usize)> = Vec::with_capacity(second_count);
		let mut sec_left_index = 0;
		let sec_right_index = second_count - 1;
		let mut first_indexs: HashMap<usize, VecDeque<usize>> = HashMap::with_capacity(first_count);
		let mut fir_left_index = 0;
		let fir_right_index = first_count - 1;
		let first = first_eles.get_ref();
		let second = second_eles.get_ref();
		// get first index cached or from cached
		fn get_first_index_cached<'a>(
			first_indexs: &'a mut HashMap<usize, VecDeque<usize>>,
			first: &[BoxDynElement],
			index: usize,
		) -> &'a mut VecDeque<usize> {
			first_indexs
				.entry(index)
				.or_insert_with(|| get_tree_indexs(&first[index]))
		}
		while fir_left_index <= fir_right_index && sec_left_index <= sec_right_index {
			// the second left
			let sec_left = &second[sec_left_index];
			let sec_left_level = get_tree_indexs(sec_left);
			// the first left
			let fir_left_level = get_first_index_cached(&mut first_indexs, &first, fir_left_index);
			match compare_indexs(&sec_left_level, &fir_left_level) {
				Ordering::Equal => {
					// move forward both
					sec_left_index += 1;
					fir_left_index += 1;
				}
				Ordering::Greater => {
					// second left is behind first left
					// if second left is also behind first right
					let fir_right_level = get_first_index_cached(&mut first_indexs, &first, fir_right_index);
					match compare_indexs(&sec_left_level, &fir_right_level) {
						Ordering::Greater => {
							// now second is all after first right
							let cur_fir_index = fir_right_index + 1;
							for index in sec_left_index..=sec_right_index {
								mids.push((index, cur_fir_index));
							}
							break;
						}
						Ordering::Less => {
							// second left is between first left and right
							// use binary search
							let mut l = fir_left_index;
							let mut r = fir_right_index;
							let mut mid = (l + r) / 2;
							let mut find_equal = false;
							while mid != l {
								let mid_level = get_first_index_cached(&mut first_indexs, &first, mid);
								match compare_indexs(&sec_left_level, &mid_level) {
									Ordering::Greater => {
										// second left is behind middle
										l = mid;
										mid = (l + r) / 2;
									}
									Ordering::Less => {
										// second left is before middle
										r = mid;
										mid = (l + r) / 2;
									}
									Ordering::Equal => {
										// find equal
										find_equal = true;
										break;
									}
								}
							}
							if !find_equal {
								mids.push((sec_left_index, mid + 1));
							}
							// set first left from mid
							fir_left_index = mid;
							// move second left index
							sec_left_index += 1;
						}
						Ordering::Equal => {
							// equal to first right, now all the second after current is behind first right
							let cur_fir_index = sec_right_index + 1;
							for index in sec_left_index + 1..=sec_right_index {
								mids.push((index, cur_fir_index));
							}
							break;
						}
					}
				}
				Ordering::Less => {
					let sec_right = &second[sec_right_index];
					let sec_right_level = get_tree_indexs(sec_right);
					match compare_indexs(&sec_right_level, &fir_left_level) {
						Ordering::Less => {
							// now second is all before current first left
							for index in sec_left_index..=sec_right_index {
								mids.push((index, fir_left_index));
							}
							break;
						}
						Ordering::Greater => {
							// second contains first or second right is in first
							// just move second left
							mids.push((sec_left_index, fir_left_index));
							sec_left_index += 1;
						}
						Ordering::Equal => {
							// equal to first left, now all the second are before current first left
							for index in sec_left_index..sec_right_index {
								mids.push((index, fir_left_index));
							}
							break;
						}
					}
				}
			}
		}
		let mids_count = mids.len();
		let mut result = Elements::with_capacity(first_count + mids_count);
		// add first and mids
		let mut mid_loop = 0;
		for (index, ele) in first_eles.get_ref().iter().enumerate() {
			if mid_loop < mids_count {
				let cur_mids = &mids[mid_loop..];
				// maybe multiple middles is between first left and right
				for (sec_index, mid_index) in cur_mids {
					if *mid_index == index {
						mid_loop += 1;
						let mid_ele = &second[*sec_index];
						result.push(mid_ele.cloned());
					} else {
						break;
					}
				}
			}
			result.push(ele.cloned());
		}
		// the second elements after first
		if mid_loop < mids_count {
			for (sec_index, _) in &mids[mid_loop..] {
				let mid_ele = &second[*sec_index];
				result.push(mid_ele.cloned());
			}
		}
		result
	}

	/// check if the ele list contains some ele
	fn includes(&self, ele: &BoxDynElement) -> bool {
		self.get_ref().iter().any(|n| ele.is(n))
	}

	/// index of
	fn index_of(&self, ele: &BoxDynElement, start_index: usize) -> Option<usize> {
		let total = self.length();
		if start_index < total {
			let nodes = self.get_ref();
			for (index, cur_ele) in nodes[start_index..].iter().enumerate() {
				if ele.is(cur_ele) {
					return Some(start_index + index);
				}
			}
		}
		None
	}
}

/*
*** Content APIs
**  [Methods]
**  text, html, set_text, set_html, texts
*/
impl<'a> Elements<'a> {
	// -------------Content API----------------
	/// pub fn `text`
	/// get the text of each element in the set
	pub fn text(&self) -> &str {
		let mut result = String::with_capacity(50);
		for ele in self.get_ref() {
			result.push_str(&ele.text_content());
		}
		to_static_str(result)
	}

	/// pub fn `set_text`
	/// set each element's text to content
	pub fn set_text(&mut self, content: &str) -> &mut Self {
		for ele in self.get_mut_ref() {
			ele.set_text(content);
		}
		self
	}

	/// pub fn `html`
	/// get the first element's html
	pub fn html(&self) -> String {
		if let Some(ele) = self.get(0) {
			return ele.inner_html();
		}
		String::from("")
	}

	/// pub fn `set_html`
	/// set each element's html to content
	pub fn set_html(&mut self, content: &str) -> &mut Self {
		for ele in self.get_mut_ref() {
			ele.set_html(content);
		}
		self
	}

	/// pub fn `outer_html`
	/// get the first element's outer html
	pub fn outer_html(&self) -> String {
		if let Some(ele) = self.get(0) {
			return ele.outer_html();
		}
		String::from("")
	}

	/// pub fn `texts`
	/// get the text node of each element
	pub fn texts(&self, limit_depth: u32) -> Texts<'a> {
		let mut result = Texts::with_capacity(DEF_NODES_LEN);
		for ele in self.get_ref() {
			if let Some(text_nodes) = ele.texts(limit_depth) {
				result.get_mut_ref().extend(text_nodes);
			}
		}
		result
	}
}

/*
*** Attribute APIs
**  [Methods]
**  attr, set_attr, remove_attr,
**  has_class, add_class, remove_class, toggle_class
*/
impl<'a> Elements<'a> {
	/// pub fn `attr`
	/// get the first element's attribute value
	pub fn attr(&self, attr_name: &str) -> Option<IAttrValue> {
		if let Some(ele) = self.get(0) {
			return ele.get_attribute(attr_name);
		}
		None
	}

	/// pub fn `has_attr`
	/// check if any element has an attribute
	pub fn has_attr(&self, attr_name: &str) -> bool {
		for ele in self.get_ref() {
			if ele.has_attribute(attr_name) {
				return true;
			}
		}
		false
	}

	/// pub fn `set_attr`
	/// set each element's attribute to `key` = attr_name, `value` = value.  
	pub fn set_attr(&mut self, attr_name: &str, value: Option<&str>) -> &mut Self {
		for ele in self.get_mut_ref() {
			ele.set_attribute(attr_name, value);
		}
		self
	}

	/// pub fn `remove_attr`
	pub fn remove_attr(&mut self, attr_name: &str) -> &mut Self {
		for ele in self.get_mut_ref() {
			ele.remove_attribute(attr_name);
		}
		self
	}

	/// pub fn `has_class`
	pub fn has_class(&self, class_name: &str) -> bool {
		let class_name = class_name.trim();
		if !class_name.is_empty() {
			let class_list = get_class_list(class_name);
			for ele in self.get_ref() {
				let class_value = ele.get_attribute(ATTR_CLASS);
				if let Some(IAttrValue::Value(cls, _)) = class_value {
					let orig_class_list = get_class_list(&cls);
					for class_name in &class_list {
						// if any of element contains the class
						if orig_class_list.contains(class_name) {
							return true;
						}
					}
				}
			}
		}
		false
	}

	/// pub fn `add_class`
	pub fn add_class(&mut self, class_name: &str) -> &mut Self {
		let class_list = get_class_list(class_name);
		if !class_list.is_empty() {
			for ele in self.get_mut_ref() {
				let class_value = ele.get_attribute(ATTR_CLASS);
				if let Some(IAttrValue::Value(cls, _)) = class_value {
					let mut orig_class_list = get_class_list(&cls);
					for class_name in &class_list {
						if !orig_class_list.contains(class_name) {
							orig_class_list.push(class_name.clone());
						}
					}
					ele.set_attribute(
						ATTR_CLASS,
						Some(class_list_to_string(&orig_class_list).as_str()),
					);
					continue;
				}
				ele.set_attribute(ATTR_CLASS, Some(class_name));
			}
		}
		self
	}
	/// pub fn `remove_class`
	pub fn remove_class(&mut self, class_name: &str) -> &mut Self {
		let class_name = class_name.trim();
		if !class_name.is_empty() {
			let class_list = get_class_list(class_name);
			for ele in self.get_mut_ref() {
				let class_value = ele.get_attribute(ATTR_CLASS);
				if let Some(IAttrValue::Value(cls, _)) = class_value {
					let mut orig_class_list = get_class_list(&cls);
					let mut removed_indexs: Vec<usize> = Vec::with_capacity(class_list.len());
					for class_name in &class_list {
						if let Some(index) = orig_class_list.iter().position(|name| name == class_name) {
							removed_indexs.push(index);
						}
					}
					if !removed_indexs.is_empty() {
						retain_by_index(&mut orig_class_list, &removed_indexs);
						ele.set_attribute(
							ATTR_CLASS,
							Some(class_list_to_string(&orig_class_list).as_str()),
						);
					}
				}
			}
		}
		self
	}
	/// pub fn `toggle_class`
	pub fn toggle_class(&mut self, class_name: &str) -> &mut Self {
		let class_name = class_name.trim();
		if !class_name.is_empty() {
			let class_list = get_class_list(class_name);
			let total = class_list.len();
			for ele in self.get_mut_ref() {
				let class_value = ele.get_attribute(ATTR_CLASS);
				if let Some(IAttrValue::Value(cls, _)) = class_value {
					let mut orig_class_list = get_class_list(&cls);
					let mut removed_indexs: Vec<usize> = Vec::with_capacity(total);
					let mut added_class_list: Vec<Vec<char>> = Vec::with_capacity(total);
					for class_name in &class_list {
						if let Some(index) = orig_class_list.iter().position(|name| name == class_name) {
							removed_indexs.push(index);
						} else {
							added_class_list.push(class_name.clone());
						}
					}
					let mut need_set = false;
					if !removed_indexs.is_empty() {
						retain_by_index(&mut orig_class_list, &removed_indexs);
						need_set = true;
					}
					if !added_class_list.is_empty() {
						orig_class_list.extend(added_class_list);
						need_set = true;
					}
					if need_set {
						ele.set_attribute(
							ATTR_CLASS,
							Some(class_list_to_string(&orig_class_list).as_str()),
						);
					}
					continue;
				}
				ele.set_attribute(ATTR_CLASS, Some(class_name));
			}
		}
		self
	}
}

/*
*** Mutations
**  [methods]
**  remove, empty,
**  append, append_to, prepend, prepend_to,
**  before, insert_before, after, insert_after
*/
impl<'a> Elements<'a> {
	/// pub fn `remove`
	pub fn remove(self) {
		for ele in self.into_iter() {
			if let Some(parent) = ele.parent().as_mut() {
				parent.remove_child(ele);
			}
		}
	}
	// pub fn `empty`
	pub fn empty(&mut self) -> &mut Self {
		self.set_text("");
		self
	}
	// `insert`
	fn insert(&mut self, dest: &Elements, position: &InsertPosition) -> &mut Self {
		for ele in self.get_mut_ref() {
			for inserted in dest.get_ref().iter().rev() {
				ele.insert_adjacent(position, inserted);
			}
		}
		self
	}
	/// pub fn `append`
	pub fn append(&mut self, elements: &mut Elements) -> &mut Self {
		self.insert(elements, &InsertPosition::BeforeEnd);
		self
	}
	/// pub fn `append_to`
	pub fn append_to(&mut self, elements: &mut Elements) -> &mut Self {
		elements.append(self);
		self
	}
	/// pub fn `prepend`
	pub fn prepend(&mut self, elements: &mut Elements) -> &mut Self {
		self.insert(elements, &InsertPosition::AfterBegin);
		self
	}
	/// pub fn `prepend_to`
	pub fn prepend_to(&mut self, elements: &mut Elements) -> &mut Self {
		elements.prepend(self);
		self
	}
	/// pub fn `insert_before`
	pub fn insert_before(&mut self, elements: &mut Elements) -> &mut Self {
		elements.before(self);
		self
	}
	/// pub fn `before`
	pub fn before(&mut self, elements: &mut Elements) -> &mut Self {
		// insert the elements before self
		self.insert(elements, &InsertPosition::BeforeBegin);
		self
	}
	/// pub fn `insert_after`
	pub fn insert_after(&mut self, elements: &mut Elements) -> &mut Self {
		elements.after(self);
		self
	}
	/// pub fn `after`
	pub fn after(&mut self, elements: &mut Elements) -> &mut Self {
		// insert the elements after self
		self.insert(elements, &InsertPosition::AfterEnd);
		self
	}
}

impl<'a> IntoIterator for Elements<'a> {
	type Item = BoxDynElement<'a>;
	type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;
	fn into_iter(self) -> Self::IntoIter {
		Box::new(self.nodes.into_iter())
	}
}

impl<'a> From<Vec<BoxDynElement<'a>>> for Elements<'a> {
	fn from(nodes: Vec<BoxDynElement<'a>>) -> Self {
		Elements { nodes }
	}
}

#[cfg(test)]
mod tests {
	use std::collections::VecDeque;

	use super::{relation_of, ElementRelation};
	use crate::mesdoc::selector::Combinator;
	use crate::Vis;
	use std::error::Error;
	#[test]
	fn test_fn_relation_of() {
		let a: VecDeque<usize> = vec![0].into();
		let b: VecDeque<usize> = vec![0, 1].into();
		assert!(matches!(relation_of(&a, &b), ElementRelation::Ancestor));
		assert!(matches!(relation_of(&b, &a), ElementRelation::Descendant));
		let c: VecDeque<usize> = vec![1, 2, 3].into();
		assert!(matches!(relation_of(&b, &c), ElementRelation::Feauture));
		assert!(matches!(relation_of(&c, &b), ElementRelation::Feauture));
		assert!(matches!(relation_of(&c, &a), ElementRelation::Feauture));
		let d: VecDeque<usize> = vec![0, 1].into();
		assert!(matches!(relation_of(&b, &d), ElementRelation::Equal));
	}
}
