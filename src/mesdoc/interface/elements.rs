#![deny(clippy::print_stdout)]
cfg_feat_text! {
	use super::{BoxDynText, Texts};
}
cfg_feat_insertion! {
	use super::InsertPosition;
}
use super::{BoxDynElement, IAttrValue, IElementTrait, IFormValue, MaybeDoc};
use crate::mesdoc::error::BoxDynError;
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
	utils::{get_class_list, retain_by_index},
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::{
	cmp::Ordering,
	collections::VecDeque,
	ops::{Bound, RangeBounds},
};

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
pub(crate) enum FilterType {
	Filter,
	Not,
	Is,
	IsAll,
}

#[derive(Default)]
pub struct Elements<'a> {
	nodes: Vec<BoxDynElement<'a>>,
	// doc will used by root elements
	#[allow(dead_code)]
	doc: MaybeDoc<'a>,
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

	pub(crate) fn trigger_method_throw_error(&self, method: &str, error: BoxDynError) {
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
			..Default::default()
		}
	}
	// with nodes
	pub fn with_nodes(nodes: Vec<BoxDynElement<'a>>) -> Self {
		Elements {
			nodes,
			..Default::default()
		}
	}
	// with all
	pub(crate) fn with_all(nodes: Vec<BoxDynElement<'a>>, doc: MaybeDoc<'a>) -> Self {
		Elements { nodes, doc }
	}
	// with capacity
	pub fn with_capacity(size: usize) -> Self {
		Elements {
			nodes: Vec::with_capacity(size),
			..Default::default()
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
	/// Iterate over the element in Elements, when the handle return false, stop the iterator.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <ul>
	///       <li>item1</li>
	///       <li>item2</li>
	///       <li>item3</li>
	///     </ul>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let mut items = doc.find("ul > li");
	///   assert_eq!(items.length(), 3);
	///   let mut items_texts:Vec<String> = vec![];
	///   items.for_each(|index, ele| {
	///     if index < 2{
	///       items_texts.push(ele.text());
	///       return true;
	///     }
	///     false
	///   });
	///   assert_eq!(items_texts.len(), 2);
	///   assert_eq!(items_texts.join(","), "item1,item2");
	///   Ok(())
	/// }
	/// ```
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

	/// A short alias for method `for_each`
	pub fn each<F>(&mut self, handle: F) -> &mut Self
	where
		F: FnMut(usize, &mut BoxDynElement) -> bool,
	{
		self.for_each(handle)
	}

	/// Get a collection of values by iterate the each element in Elements and call the `handle` function.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <ul>
	///       <li>item1</li>
	///       <li>item2</li>
	///       <li>item3</li>
	///     </ul>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let mut items = doc.find("ul > li");
	///   assert_eq!(items.length(), 3);
	///   let items_texts: Vec<String> = items.map(|index, ele| {
	///     ele.text()
	///   });
	///   assert_eq!(items_texts.len(), 3);
	///   assert_eq!(items_texts.join(","), "item1,item2,item3");
	///   Ok(())
	/// }
	/// ```
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

	/// Return the length of the Elements set.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <ul>
	///       <li>item1</li>
	///       <li>item2</li>
	///       <li>item3</li>
	///     </ul>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let mut items = doc.find("ul > li:contains('item3')");
	///   assert_eq!(items.length(), 1);
	///   Ok(())
	/// }
	/// ```
	pub fn length(&self) -> usize {
		self.nodes.len()
	}

	/// Check if the Elements is empty, it's a short alias for `.length() == 0`
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <ul>
	///       <li>item1</li>
	///       <li>item2</li>
	///       <li>item3</li>
	///     </ul>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let mut items = doc.find("ul > li:empty");
	///   assert!(items.is_empty());
	///   assert_eq!(items.length(), 0);
	///   Ok(())
	/// }
	/// ```
	pub fn is_empty(&self) -> bool {
		self.length() == 0
	}

	/// A quick way to get the document object when the loaded html is a document.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <ul>
	///           <li>item1</li>
	///           <li>item2</li>
	///           <li>item3</li>
	///         </ul>
	///       </body>
	///     </html>
	///   "##;
	///   let root = Vis::load(html)?;
	///   let mut document = root.document();
	///   assert!(document.is_some());
	///   assert_eq!(document.unwrap().title(), Some(String::from("document")));
	///   Ok(())
	/// }
	/// ```
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
			self.find_selector(selector)
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
							let meet_until = cur_eles.filter_type_handle(selector, &FilterType::Is).1;
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
									Box::new(|child, is_matched, loop_child| {
										if is_matched {
											result.get_mut_ref().push(child.cloned());
										}
										if loop_child && child.child_nodes_length() > 0 {
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
											if child.is(cmp_child) {
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
								Box::new(|ele, is_matched, _| {
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
				result = matcher.apply(elements, None);
			}
		};
		result
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
	/// Get the descendants of each element in the Elements, filtered by the selector
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <ul>
	///           <li>item1</li>
	///           <li>item2</li>
	///           <li>
	///               <ol>
	///                 <li>subitem1</li>
	///                 <li>subitem2</li>
	///               </ol>
	///           </li>
	///         </ul>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   assert_eq!(doc.find("ul").length(), 1);
	///   assert_eq!(doc.find("ul li").length(), 5);
	///   assert_eq!(doc.find("ul > li").length(), 3);
	///   assert_eq!(doc.find("ul li:first-child").text(), "item1subitem1");
	///   Ok(())
	/// }
	/// ```
	pub fn find(&self, selector: &str) -> Elements<'a> {
		let s = Selector::from_str(selector, true);
		if let Ok(selector) = &s {
			return self.find_selector(selector);
		}
		self.trigger_method_throw_error("find", Box::new(s.unwrap_err()));
		Elements::new()
	}

	/// Reduce the Elements to those that match the selector.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <ul>
	///           <li>item1</li>
	///           <li class="item2">item2</li>
	///           <li>item3</li>
	///         </ul>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let items = doc.find("li");
	///   assert_eq!(items.length(), 3);
	///   assert_eq!(items.filter("[class]").length(), 1);
	///   assert_eq!(items.filter("[class]").text(), "item2");
	///   assert_eq!(items.filter("li:contains('item3')").length(), 1);
	///   assert_eq!(items.filter("li:contains('item3')").text(), "item3");
	///   Ok(())
	/// }
	/// ```
	pub fn filter(&self, selector: &str) -> Elements<'a> {
		const METHOD: &str = "filter";
		self.trigger_method(METHOD, selector, |selector| {
			self.filter_type_handle(selector, &FilterType::Filter).0
		})
	}

	/// Reduce the Elements to those that pass the handle function test.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <ul>
	///           <li>item1</li>
	///           <li class="item2">item2</li>
	///           <li>item3</li>
	///         </ul>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let items = doc.find("li");
	///   assert_eq!(items.length(), 3);
	///   let class_items = items.filter_by(|_, ele| ele.get_attribute("class").is_some());
	///   assert_eq!(class_items.length(), 1);
	///   assert_eq!(class_items.text(), "item2");
	///   Ok(())
	/// }
	/// ```
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

	/// Reduce the Elements to those that also in the searched Elements.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <ul>
	///           <li></li>
	///           <li class="item2">item2</li>
	///           <li class="item3"></li>
	///         </ul>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let items = doc.find("li");
	///   assert_eq!(items.length(), 3);
	///   let empty_items = items.filter(":empty");
	///   let class_items = items.filter("[class]");
	///   assert_eq!(empty_items.length(), 2);
	///   assert_eq!(class_items.length(), 2);
	///   // has class and also empty
	///   let class_and_empty_items = class_items.filter_in(&empty_items);
	///   assert_eq!(class_and_empty_items.length(), 1);
	///   assert_eq!(class_and_empty_items.attr("class").unwrap().to_string(), "item3");
	///   Ok(())
	/// }
	/// ```
	pub fn filter_in(&self, search: &Elements) -> Elements<'a> {
		self.filter_in_handle(search, FilterType::Filter).0
	}

	/// Get the children of each element in Elements, when the selector is not empty, will filtered by the selector
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">
	///               <dl>
	///                 <dd>subitem1</dd>
	///                 <dd>subitem2</dd>
	///               </dl>
	///           </dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let top_dl = doc.find("dl").eq(0);
	///   let all_dd = top_dl.find("dd");
	///   assert_eq!(all_dd.length(), 5);
	///   let child_items = top_dl.children("");
	///   assert_eq!(child_items.length(), 4);
	///   let child_dd = top_dl.children("dd");
	///   assert_eq!(child_dd.length(),3);
	///   Ok(())
	/// }
	/// ```
	pub fn children(&self, selector: &str) -> Elements<'a> {
		self.select_with_comb("children", selector, Combinator::Children)
	}

	/// Get the previous sibling of each element in Elements, when the selector is not empty, will filtered by the selector.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let item3 = dl.children(".item3");
	///   assert_eq!(item3.prev("").text(), "item2");
	///   assert_eq!(item3.prev(":not[class]").is_empty(),true);
	///   Ok(())
	/// }
	/// ```
	pub fn prev(&self, selector: &str) -> Elements<'a> {
		self.select_with_comb("prev", selector, Combinator::Prev)
	}

	/// Get all preceding siblings of each element in Elements, when the selector is not empty, will filtered by the selector.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let item3 = dl.children(".item3");
	///   assert_eq!(item3.prev_all("").length(), 3);
	///   assert_eq!(item3.prev_all("dd").length(),2);
	///   assert_eq!(item3.prev_all("dt").length(),1);
	///   assert_eq!(item3.prev_all("dd[class]").length(),1);
	///   Ok(())
	/// }
	/// ```
	pub fn prev_all(&self, selector: &str) -> Elements<'a> {
		let uniques = self.unique_sibling_last();
		uniques.select_with_comb("prev_all", selector, Combinator::PrevAll)
	}

	/// Get all preceding siblings of each element in Elements, until the previous sibling element matched the selector, when contains is true, the matched previous sibling will be included, otherwise it will exclude; when the filter is not empty, will filtered by the selector;
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let item3 = dl.children(".item3");
	///   assert_eq!(item3.prev_until("dt", "", false).length(), 2);
	///   assert_eq!(item3.prev_until("dt", "", false).eq(0).is("dd"), true);
	///   assert_eq!(item3.prev_until("dt", "", true).length(), 3);
	///   assert_eq!(item3.prev_until("dt", "", true).eq(0).is("dt"), true);
	///   assert_eq!(item3.prev_until("dt", "dd", true).length(), 2);
	///   assert_eq!(item3.prev_until("dt", "dd", true).eq(0).is("dd"), true);
	///   Ok(())
	/// }
	/// ```
	pub fn prev_until(&self, selector: &str, filter: &str, contains: bool) -> Elements<'a> {
		let uniques = self.unique_sibling_last();
		let mut result =
			uniques.select_with_comb_until("prev_until", selector, filter, contains, Combinator::Prev);
		// should reverse the result when length > 1
		// because the prevs executed from last to first
		if result.length() > 1 {
			result.get_mut_ref().reverse();
		}
		result
	}

	/// Get the next sibling of each element in Elements, when the selector is not empty, will filtered by the selector.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let dt = dl.children("dt");
	///   assert_eq!(dt.next("").text(), "item1");
	///   assert_eq!(dt.next("[class]").is_empty(), true);
	///   Ok(())
	/// }
	/// ```
	pub fn next(&self, selector: &str) -> Elements<'a> {
		self.select_with_comb("next", selector, Combinator::Next)
	}

	/// Get all following siblings of each element in Elements, when the selector is not empty, will filtered by the selector.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let dt = dl.children("dt");
	///   assert_eq!(dt.next_all("").length(), 3);
	///   assert_eq!(dt.next_all("[class]").length(), 2);
	///   assert_eq!(dt.next_all("[class]").text(), "item2item3");
	///   Ok(())
	/// }
	/// ```
	pub fn next_all(&self, selector: &str) -> Elements<'a> {
		// unique, keep the first sibling node
		let uniques = self.unique_sibling_first();
		uniques.select_with_comb("next_all", selector, Combinator::NextAll)
	}

	/// Get all following siblings of each element in Elements, until the sibling element matched the selector, when contains is true, the matched sibling will be included, otherwise it will exclude; when the filter is not empty, will filtered by the selector;
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let dt = dl.children("dt");
	///   assert_eq!(dt.next_until(".item3", "", false).length(), 2);
	///   assert_eq!(dt.next_until(".item3", "", true).length(), 3);
	///   assert_eq!(dt.next_until(".item3", "[class]", false).length(), 1);
	///   assert_eq!(dt.next_until(".item3", "[class]", true).length(), 2);
	///   Ok(())
	/// }
	/// ```
	pub fn next_until(&self, selector: &str, filter: &str, contains: bool) -> Elements<'a> {
		// unique, keep the first sibling node
		let uniques = self.unique_sibling_first();
		uniques.select_with_comb_until("next_until", selector, filter, contains, Combinator::Next)
	}

	/// Get the siblings of each element in Elements, when the selector is not empty, will filtered by the selector.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let item2 = dl.children(".item2");
	///   assert_eq!(item2.siblings("").length(), 3);
	///   assert_eq!(item2.siblings("").first().is("dt"), true);
	///   assert_eq!(item2.siblings("dd").first().text(), "item1");
	///   Ok(())
	/// }
	/// ```
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
							sib_selector.expect_err("Selector parse error")
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

	/// Get the parent of each element in Elements, when the selector is not empty, will filtered by the selector.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let item2 = dl.children(".item2");
	///   assert_eq!(item2.parent("").length(), 1);
	///   assert_eq!(item2.parent("").get(0).unwrap().tag_name(), "DL");
	///   assert_eq!(item2.parent("ul").length(), 0);
	///   Ok(())
	/// }
	/// ```
	pub fn parent(&self, selector: &str) -> Elements<'a> {
		// unique, keep the first sibling node
		let uniques = self.unique_sibling_first();
		uniques.select_with_comb("parent", selector, Combinator::Parent)
	}

	/// Get the ancestors of each element in Elements, when the selector is not empty, will filtered by the selector.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let item2 = dl.children(".item2");
	///   assert_eq!(item2.parents("").length(), 3);
	///   assert_eq!(item2.parents("").first().is("html"), true);
	///   assert_eq!(item2.parents("").last().is("dl"), true);
	///   assert_eq!(item2.parents("dl").length(), 1);
	///   Ok(())
	/// }
	/// ```
	pub fn parents(&self, selector: &str) -> Elements<'a> {
		// unique, keep the first sibling node
		let uniques = self.unique_sibling_first();
		let mut result = uniques.select_with_comb("parents", selector, Combinator::ParentAll);
		result.sort_and_unique();
		result
	}

	/// Get the ancestors of each element in Elements, until the ancestor matched the selector, when contains is true, the matched ancestor will be included, otherwise it will exclude; when the filter is not empty, will filtered by the selector;
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let item2 = dl.children(".item2");
	///   assert_eq!(item2.parents_until("body", "", false).length(), 1);
	///   assert_eq!(item2.parents_until("body", "", true).length(), 2);
	///   assert_eq!(item2.parents_until("body", "dl", true).length(), 1);
	///   Ok(())
	/// }
	/// ```
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
		// parents may not unique if has ancestor and childs
		// if parents length > 1, the parents need reversed
		result.sort_and_unique();
		result
	}

	/// Get the first matched element of each element in Elements, traversing from self to it's ancestors.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let item2 = dl.children(".item2");
	///   assert_eq!(item2.closest("dd").length(), 1);
	///   assert_eq!(item2.closest("dd").is_all_in(&item2), true);
	///   assert_eq!(item2.closest("dl").is_all_in(&item2.parent("")), true);
	///   Ok(())
	/// }
	/// ```
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
							if self.has_ele(ele, first_comb, Some(lookup)) {
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
			let cached = matcher.apply(elements, Some(true));
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
			Elements::select_by_rule(elements, first_rule, Some(comb))
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

	/// This method will be removed in future versions.
	/// If you want to clone an elements set, please use the `clone()` method instead.
	/// This method only clone the element's `Rc` pointer in the elements set.
	/// Any modifications to the cloned elements set will be reflected on the original elements set.
	pub fn cloned(&self) -> Elements<'a> {
		let mut result = Elements::with_capacity(self.length());
		for ele in &self.nodes {
			result.push(ele.cloned());
		}
		result
	}

	/// pub fn contains
	pub fn contains(&self, ele: &BoxDynElement, comb: &Combinator) -> bool {
		self.has_ele(ele, comb, None)
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
						if self.includes(parent) {
							return true;
						}
					}
				}
			}
			ParentAll => {
				for ele in elements.get_ref() {
					if let Some(parent) = &ele.parent() {
						if self.includes(parent) {
							return true;
						}
						if let Some(ancestor) = &parent.parent() {
							if self.includes(ancestor) {
								return true;
							}
							// iterator the search process, becareful with the combinator now is ChildrenAll
							// the sentences must in if condition, otherwise it will break the for loop
							if self.has_ele(ancestor, &Combinator::ChildrenAll, None) {
								return true;
							}
						}
					}
				}
			}
			Prev => {
				for ele in elements.get_ref() {
					if let Some(prev) = &ele.previous_element_sibling() {
						if self.includes(prev) {
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

	/// Check at least one element in Elements is match the selector.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   assert_eq!(items.is("dd"), true);
	///   assert_eq!(items.is("dt"), true);
	///   assert_eq!(items.is(".item2"), true);
	///   assert_eq!(items.is(".item3"), true);
	///   assert_eq!(items.is(":contains('item2')"), true);
	///   Ok(())
	/// }
	/// ```
	pub fn is(&self, selector: &str) -> bool {
		const METHOD: &str = "is";
		self.trigger_method(METHOD, selector, |selector| {
			self.filter_type_handle(selector, &FilterType::Is).1
		})
	}

	/// Check at least one element in Elements call the handle function return true.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   assert_eq!(items.is_by(|_, ele|{
	///     ele.tag_name() == "DT"
	///   }), true);
	///   assert_eq!(items.is_by(|_, ele|{
	///     ele.tag_name() == "DD"
	///   }), true);
	///   assert_eq!(items.is_by(|_, ele|{
	///     Vis::dom(ele).has_class("item2")
	///   }), true);
	///   Ok(())
	/// }
	/// ```
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

	/// Check at least one element in Elements is also in the other Elements set.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   let dd = items.filter("dd");
	///   let dt = items.filter("dt");
	///   assert_eq!(items.is_in(&dd), true);
	///   assert_eq!(items.is_in(&dt), true);
	///   Ok(())
	/// }
	/// ```
	pub fn is_in(&self, search: &Elements) -> bool {
		self.filter_in_handle(search, FilterType::Is).1
	}

	/// Check if each element in Elements are all matched the selector.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   assert_eq!(items.is_all("dd"), false);
	///   assert_eq!(items.is_all("dt"), false);
	///   assert_eq!(items.is_all("dt,dd"), true);
	///   Ok(())
	/// }
	/// ```
	pub fn is_all(&self, selector: &str) -> bool {
		const METHOD: &str = "is_all";
		self.trigger_method(METHOD, selector, |selector| {
			self.filter_type_handle(selector, &FilterType::IsAll).1
		})
	}

	/// Check if each element in Elements call the handle function are all returned true.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   assert_eq!(items.is_all_by(|_, ele|{
	///     ele.tag_name() == "DT"
	///   }), false);
	///   assert_eq!(items.is_all_by(|_, ele|{
	///     ele.tag_name() == "DD"
	///   }), false);
	///   assert_eq!(items.is_all_by(|_, ele|{
	///     let tag_name = ele.tag_name();
	///     tag_name == "DT" || tag_name == "DD"
	///   }), true);
	///   Ok(())
	/// }
	/// ```
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

	/// Check if each element in Elements is also in the other Elements set.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   let dd = items.filter("dd");
	///   let dt = items.filter("dt");
	///   assert_eq!(items.is_all_in(&dd), false);
	///   assert_eq!(items.is_all_in(&dt), false);
	///   assert_eq!(dd.is_all_in(&items), true);
	///   assert_eq!(dt.is_all_in(&items), true);
	///   Ok(())
	/// }
	/// ```
	pub fn is_all_in(&self, search: &Elements) -> bool {
		self.filter_in_handle(search, FilterType::IsAll).1
	}

	/// Remove elements those that match the selector from the Elements set.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   assert_eq!(items.not("dd").is_all("dt"), true);
	///   assert_eq!(items.not("dt").is_all("dd"), true);
	///   assert_eq!(items.not("dt,dd").is_empty(), true);
	///   Ok(())
	/// }
	/// ```
	pub fn not(&self, selector: &str) -> Elements<'a> {
		const METHOD: &str = "not";
		self.trigger_method(METHOD, selector, |selector| {
			self.filter_type_handle(selector, &FilterType::Not).0
		})
	}

	/// Remove elements those that pass the handle function test from the Elements set.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   assert_eq!(items.not_by(|_, ele|ele.tag_name() == "DD").is_all("dt"), true);
	///   assert_eq!(items.not_by(|_, ele|ele.tag_name() == "DT").is_all("dd"), true);
	///   assert_eq!(items.not_by(|_, ele|ele.tag_name() == "DT" || ele.tag_name() == "DD").is_empty(), true);
	///   Ok(())
	/// }
	/// ```
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

	/// Remove elements those that also in the elements from the Elements set.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd>item1</dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   assert_eq!(items.not_in(&items.filter("dd")).is_all("dt"), true);
	///   assert_eq!(items.not_in(&items.filter("dt")).is_all("dd"), true);
	///   assert_eq!(items.not_in(&items).is_empty(), true);
	///   Ok(())
	/// }
	/// ```
	pub fn not_in(&self, search: &Elements) -> Elements<'a> {
		self.filter_in_handle(search, FilterType::Not).0
	}

	/// Reduce Elements to those that have a descendant that matches the selector.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>&lt;<strong>T</strong>itle&gt;</dt>
	///           <dd><span>item1</span></dd>
	///           <dd class="item2">item2</dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   assert_eq!(items.filter("dt").text(), "<Title>");
	///   assert_eq!(items.filter("dd").text(), "item1item2item3");
	///   Ok(())
	/// }
	/// ```
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

	/// Reduce Elements to those that have a descendant that matches the selector.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd><span>item1</span></dd>
	///           <dd class="item2"><span>item2</span></dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   assert_eq!(items.length(), 4);
	///   assert_eq!(items.has_in(&items.children("span")).length(), 2);
	///   assert_eq!(dl.has_in(&items).length(), 1);
	///   Ok(())
	/// }
	/// ```
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
		self.filter_by(|_, ele| loop_handle(ele, search))
	}
}

/*
*** Other Selector and Helper APIs
**  [Methods]
*/
impl<'a> Elements<'a> {
	/// Get one element from the Elements at the specified index.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd><span>item1</span></dd>
	///           <dd class="item2"><span>item2</span></dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   assert_eq!(items.eq(0).is("dt"), true);
	///   assert_eq!(items.eq(2).has_class("item2"), true);
	///   Ok(())
	/// }
	/// ```
	pub fn eq(&self, index: usize) -> Elements<'a> {
		if let Some(ele) = self.get(index) {
			Elements::with_node(ele)
		} else {
			Elements::new()
		}
	}

	/// Get the first element of the Elements set,equal to eq(0).
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd><span>item1</span></dd>
	///           <dd class="item2"><span>item2</span></dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   assert_eq!(items.first().is_all_in(&items.eq(0)), true);
	///   assert_eq!(items.first().is("dt"), true);
	///   Ok(())
	/// }
	/// ```
	pub fn first(&self) -> Elements<'a> {
		self.eq(0)
	}

	/// Get the last element of the set, equal to eq(length - 1).
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd><span>item1</span></dd>
	///           <dd class="item2"><span>item2</span></dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   assert_eq!(items.last().is_all_in(&items.eq(items.length()-1)), true);
	///   assert_eq!(items.last().is(".item3"), true);
	///   Ok(())
	/// }
	/// ```
	pub fn last(&self) -> Elements<'a> {
		self.eq(self.length() - 1)
	}

	/// Get a subset specified by a range of indices.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd><span>item1</span></dd>
	///           <dd class="item2"><span>item2</span></dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   assert_eq!(items.slice(..).length(), 4);
	///   assert_eq!(items.slice(0..3).length(), 3);
	///   assert_eq!(items.slice(0..=3).length(), 4);
	///   assert_eq!(items.slice(0..10).length(), 4);
	///   Ok(())
	/// }
	/// ```
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

	/// Get a concated element set from Elements and the other parameter elements, it will generate a new element set, take the ownership of the parameter elements, but have no sence with the Elements itself.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd><span>item1</span></dd>
	///           <dd class="item2"><span>item2</span></dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   let dt = items.filter("dt");
	///   let class_dd = items.filter("[class]");
	///   assert_eq!(dt.length(), 1);
	///   assert_eq!(class_dd.length(), 2);
	///   let add_dt_dd = dt.add(class_dd);
	///   assert_eq!(add_dt_dd.length(), 3);
	///   Ok(())
	/// }
	/// ```
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
			let fir_left_level = get_first_index_cached(&mut first_indexs, first, fir_left_index);
			match compare_indexs(&sec_left_level, fir_left_level) {
				Ordering::Equal => {
					// move forward both
					sec_left_index += 1;
					fir_left_index += 1;
				}
				Ordering::Greater => {
					// second left is behind first left
					// if second left is also behind first right
					let fir_right_level = get_first_index_cached(&mut first_indexs, first, fir_right_index);
					match compare_indexs(&sec_left_level, fir_right_level) {
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
								let mid_level = get_first_index_cached(&mut first_indexs, first, mid);
								match compare_indexs(&sec_left_level, mid_level) {
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
					match compare_indexs(&sec_right_level, fir_left_level) {
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
	/// Get the form value of input, select, option, textarea.
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <input type="text" value="textvalue" />
	///         <select name="single">
	///           <option value="default"></option>
	///           <option value="opt1">opt1</option>
	///           <option value="opt2">opt1</option>
	///         </select>
	///         <select multiple>
	///           <option value="default"></option>
	///           <option value="opt1" selected="selected">opt1</option>
	///           <option value="opt2" selected="selected">opt1</option>
	///         </select>
	///         <textarea><div>hello</div></textarea>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let input = doc.find("input[type='text']");
	///   assert_eq!(input.val().to_string(), "textvalue");
	///   let select = doc.find("select[name='single']");
	///   assert_eq!(select.val().to_string(), "default");
	///   let multi_select = doc.find("select[multiple]");
	///   assert_eq!(multi_select.val().to_string(), "opt1,opt2");
	///   let textarea = doc.find("textarea");
	///   assert_eq!(textarea.val().to_string(), "<div>hello</div>");
	///   Ok(())
	/// }
	/// ```
	pub fn val(&self) -> IFormValue {
		if let Some(first) = self.get(0) {
			return first.value();
		}
		IFormValue::Single(String::from(""))
	}

	/// Get the text of each element in Elements，the html entity will auto decoded.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd><span>item1</span></dd>
	///           <dd class="item2"><span>item2</span></dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   let dt = items.filter("dt");
	///   let class_dd = items.filter("[class]");
	///   assert_eq!(dt.length(), 1);
	///   assert_eq!(class_dd.length(), 2);
	///   let add_dt_dd = dt.add(class_dd);
	///   assert_eq!(add_dt_dd.length(), 3);
	///   Ok(())
	/// }
	/// ```
	pub fn text(&self) -> String {
		let mut result = String::with_capacity(50);
		for ele in self.get_ref() {
			result.push_str(&ele.text_content());
		}
		result
	}

	/// Set the Elements's text, the html entity in content will auto encoded.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>set_text()</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd><span>item1</span></dd>
	///           <dd class="item2"><span>item2</span></dd>
	///           <dd class="item3">item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   let mut dt = items.filter("dt");
	///   dt.set_text("<Title>");
	///   assert_eq!(dt.text(), "<Title>");
	///   assert_eq!(dt.html(), "&lt;Title&gt;");
	///   let mut item2 = items.filter(".item2");
	///   assert_eq!(item2.html(), "<span>item2</span>");
	///   item2.set_text("item2");
	///   assert_eq!(item2.html(), "item2");
	///   Ok(())
	/// }
	/// ```
	pub fn set_text(&mut self, content: &str) -> &mut Self {
		for ele in self.get_mut_ref() {
			ele.set_text(content);
		}
		self
	}

	/// Get the html of the first element in Elements.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>html()</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd><span>item1</span></dd>
	///           <dd class="item2"><span>item2</span></dd>
	///           <dd class="item3"><!--comment-->item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   let item2 = items.filter(".item2");
	///   assert_eq!(item2.html(), "<span>item2</span>");
	///   let item3 = items.filter(".item3");
	///   assert_eq!(item3.html(), "<!--comment-->item3");
	///   Ok(())
	/// }
	/// ```
	pub fn html(&self) -> String {
		if let Some(ele) = self.get(0) {
			return ele.inner_html();
		}
		String::from("")
	}

	/// Get the combined html of all the elements in Elements.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>htmls()</title>
	///       </head>
	///       <body>
	///         <div><span class="div1span">span1</span></div>
	///         <div><span class="div2span">span2</span></div>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let divs = doc.find("div");
	///   assert_eq!(divs.htmls(), r#"<span class="div1span">span1</span><span class="div2span">span2</span>"#);
	///   assert_eq!(doc.find("p").htmls(), "");
	///   Ok(())
	/// }
	/// ```
	pub fn htmls(&self) -> String {
		self.map(|_, ele| ele.html()).join("")
	}

	/// Set the html to content of each element in Elements.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd><span>item1</span></dd>
	///           <dd class="item2"><span>item2</span></dd>
	///           <dd class="item3"><!--comment-->item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   let mut item2 = items.filter(".item2");
	///   assert_eq!(item2.html(), "<span>item2</span>");
	///   item2.set_html("set_html:<em>item2</em>");
	///   assert_eq!(item2.html(), "set_html:<em>item2</em>");
	///   Ok(())
	/// }
	/// ```
	pub fn set_html(&mut self, content: &str) -> &mut Self {
		for ele in self.get_mut_ref() {
			ele.set_html(content);
		}
		self
	}

	/// Get the outer html of the first element in Elements.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd><span>item1</span></dd>
	///           <dd class="item2"><span>item2</span></dd>
	///           <dd class="item3"><!--comment-->item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let items = dl.children("");
	///   let mut item2 = items.filter(".item2");
	///   assert_eq!(item2.outer_html(), r#"<dd class="item2"><span>item2</span></dd>"#);
	///   Ok(())
	/// }
	/// ```
	pub fn outer_html(&self) -> String {
		if let Some(ele) = self.get(0) {
			return ele.outer_html();
		}
		String::from("")
	}

	/// Get the combined outer html of all the elements in Elements.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>outer_htmls()</title>
	///       </head>
	///       <body>
	///         <div><span class="div1span">span1</span></div>
	///         <div><span class="div2span">span2</span></div>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let divs = doc.find("div");
	///   assert_eq!(divs.outer_htmls(), r#"<div><span class="div1span">span1</span></div><div><span class="div2span">span2</span></div>"#);
	///   assert_eq!(doc.find("p").outer_htmls(), "");
	///   Ok(())
	/// }
	/// ```
	pub fn outer_htmls(&self) -> String {
		self.map(|_, ele| ele.outer_html()).join("")
	}

	cfg_feat_text! {
		/// pub fn `texts`
		/// get the text node of each element
		pub fn texts(&self, limit_depth: usize) -> Texts<'a> {
			let mut result = Texts::with_capacity(DEF_NODES_LEN);
			for ele in self.get_ref() {
				if let Some(text_nodes) = ele.texts(limit_depth) {
					result.get_mut_ref().extend(text_nodes);
				}
			}
			result
		}

		/// pub fn `texts_by`
		/// get the text node of each element, filter by the handle
		pub fn texts_by(
			&self,
			limit_depth: usize,
			handle: Box<dyn Fn(usize, &BoxDynText) -> bool>,
		) -> Texts<'a> {
			self.texts_by_rec(limit_depth, handle, Box::new(|_: &BoxDynElement|true))
		}

		/// pub fn `texts_by_rec`
		/// get the text node of each element, filter by the handle, and check if need recursive by the result of rec_handle with child element
		pub fn texts_by_rec(
			&self,
			limit_depth: usize,
			handle: Box<dyn Fn(usize, &BoxDynText) -> bool>,
			rec_handle: Box<dyn Fn(&BoxDynElement) -> bool>
		) -> Texts<'a> {
			let mut result = Texts::with_capacity(DEF_NODES_LEN);
			for ele in self.get_ref() {
				if let Some(text_nodes) = ele.texts_by_rec(limit_depth, &handle, &rec_handle) {
					result.get_mut_ref().extend(text_nodes);
				}
			}
			result
		}
	}
}

/*
*** Attribute APIs
**  [Methods]
**  attr, set_attr, remove_attr,
**  has_class, add_class, remove_class, toggle_class
*/
impl<'a> Elements<'a> {
	/// Get an atrribute by name from the first element in Elements.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <input type="text" class="inp inp-txt" readonly />
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let input = doc.find("input");
	///   let attr_readonly = input.attr("readonly");
	///   assert!(attr_readonly.is_some() && attr_readonly.unwrap().is_true());
	///   let attr_type = input.attr("type");
	///   assert!(attr_type.is_some() && attr_type.unwrap().to_string() == "text");
	///   let attr_class = input.attr("class");
	///   assert!(attr_class.is_some() && attr_class.unwrap().to_list().contains(&"inp"));
	///   Ok(())
	/// }
	/// ```
	pub fn attr(&self, attr_name: &str) -> Option<IAttrValue> {
		if let Some(ele) = self.get(0) {
			return ele.get_attribute(attr_name);
		}
		None
	}

	/// Check if has an attribute with specified name.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <input type="text" class="inp inp-txt" readonly />
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let input = doc.find("input");
	///   assert!(input.has_attr("readonly"));
	///   assert!(input.has_attr("class"));
	///   assert!(input.has_attr("type"));
	///   assert_eq!(input.has_attr("value"), false);
	///   Ok(())
	/// }
	/// ```
	pub fn has_attr(&self, attr_name: &str) -> bool {
		for ele in self.get_ref() {
			if ele.has_attribute(attr_name) {
				return true;
			}
		}
		false
	}

	/// Set a specified name attribute with a value who's type is an `Option<&str>`, when the value is `None`，that means the attribute does'n have a string value but a bool value of `true`.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <input type="text" class="inp inp-txt" readonly />
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let mut input = doc.find("input");
	///   assert_eq!(input.has_attr("value"), false);
	///   input.set_attr("value", None);
	///   assert!(input.attr("value").is_some() && input.attr("value").unwrap().is_true());
	///   input.set_attr("value", Some("myinput"));
	///   assert!(input.attr("value").is_some() && input.attr("value").unwrap().to_string() == "myinput");
	///   input.set_attr("value", Some(""));
	///   assert!(input.attr("value").is_some() && input.attr("value").unwrap().is_true() == false);
	///   Ok(())
	/// }
	/// ```  
	pub fn set_attr(&mut self, attr_name: &str, value: Option<&str>) -> &mut Self {
		for ele in self.get_mut_ref() {
			ele.set_attribute(attr_name, value);
		}
		self
	}

	/// Remove a specified name attribute from the each element in the Elements.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <input type="text" class="inp inp-txt" readonly />
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let mut input = doc.find("input");
	///   assert_eq!(input.has_attr("value"), false);
	///   input.set_attr("value", None);
	///   assert!(input.attr("value").is_some() && input.attr("value").unwrap().is_true());
	///   input.remove_attr("value");
	///   assert!(input.attr("value").is_none());
	///   assert!(input.attr("readonly").is_some());
	///   input.remove_attr("readonly");
	///   assert!(input.attr("readonly").is_none());
	///   Ok(())
	/// }
	/// ```
	pub fn remove_attr(&mut self, attr_name: &str) -> &mut Self {
		for ele in self.get_mut_ref() {
			ele.remove_attribute(attr_name);
		}
		self
	}

	/// Check if Elements's ClassList contains the specified class name, multiple classes can be splitted by whitespaces.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <input type="text" class="inp inp-txt" readonly />
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let input = doc.find("input");
	///   assert_eq!(input.has_class("inp"), true);
	///   assert_eq!(input.has_class("inp-txt"), true);
	///   assert_eq!(input.has_class("inp-txt inp"), true);
	///   assert_eq!(input.has_class("inp-txt inp noinp"), false);
	///   Ok(())
	/// }
	/// ```
	pub fn has_class(&self, class_name: &str) -> bool {
		let class_name = class_name.trim();
		if !class_name.is_empty() {
			let class_list = get_class_list(class_name);
			for ele in self.get_ref() {
				let class_value = ele.get_attribute(ATTR_CLASS);
				if let Some(IAttrValue::Value(cls, _)) = class_value {
					let orig_class_list = get_class_list(&cls);
					let mut has_all = true;
					for class_name in &class_list {
						// if any class name is not in the original class list
						// the flag will set by false
						if !orig_class_list.contains(class_name) {
							has_all = false;
							break;
						}
					}
					// if any of the element contains the all class list
					if has_all {
						return true;
					}
				}
			}
		}
		false
	}

	/// Add class to Elements's ClassList, multiple classes can be splitted by whitespaces.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <input type="text" class="inp inp-txt" readonly />
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let mut input = doc.find("input");
	///   assert_eq!(input.has_class("noinp"), false);
	///   assert_eq!(input.has_class("inp-red"), false);
	///   input.add_class("noinp inp-red");
	///   assert_eq!(input.has_class("noinp"), true);
	///   assert_eq!(input.has_class("inp-red"), true);
	///   Ok(())
	/// }
	/// ```
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

	/// Remove class from Elements's ClassList, multiple classes can be splitted by whitespaces.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <input type="text" class="inp inp-txt" readonly />
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let mut input = doc.find("input");
	///   assert_eq!(input.has_class("inp"), true);
	///   assert_eq!(input.has_class("inp-txt"), true);
	///   input.remove_class("inp inp-txt");
	///   assert_eq!(input.has_class("inp"), false);
	///   assert_eq!(input.has_class("inp-txt"), false);
	///   assert_eq!(input.has_attr("class"), true);
	///   Ok(())
	/// }
	/// ```
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

	/// Toggle the class name from Elements's ClassList, multiple classes can be splitted by whitespaces.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <input type="text" class="inp inp-txt" readonly />
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let mut input = doc.find("input");
	///   assert_eq!(input.has_class("inp"), true);
	///   assert_eq!(input.has_class("inp-txt"), true);
	///   input.toggle_class("inp inp-red");
	///   assert_eq!(input.has_class("inp"), false);
	///   assert_eq!(input.has_class("inp-txt"), true);
	///   assert_eq!(input.has_class("inp-red"), true);
	///   Ok(())
	/// }
	/// ```
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

cfg_feat_mutation! {
	/// Clone the Elements set.
	///
	/// ```
	/// use visdom::Vis;
	/// use visdom::types::BoxDynError;
	/// fn main()-> Result<(), BoxDynError>{
	///   let html = r##"
	///     <html>
	///       <head>
	///         <title>document</title>
	///       </head>
	///       <body>
	///         <dl>
	///           <dt>Title</dt>
	///           <dd><span class="span">item1</span></dd>
	///           <dd class="item2"><span>item2</span></dd>
	///           <dd class="item3"><!--comment-->item3</dd>
	///         </dl>
	///       </body>
	///     </html>
	///   "##;
	///   let doc = Vis::load(html)?;
	///   let dl = doc.find("dl");
	///   let span = dl.find("span.span");
	///   assert_eq!(span.text(), "item1");
	///   // clone the "dl" Elements
	///   let clone_dl = dl.clone();
	///   let mut clone_span = clone_dl.find("span.span");
	///   clone_span.set_text("span");
	///   assert_eq!(span.text(), "item1");
	///   assert_eq!(clone_dl.find("span.span").text(), "span");
	///   Ok(())
	/// }
	/// ```
	impl<'a> std::clone::Clone for Elements<'a>{
		fn clone(&self) -> Self {
			let nodes: Vec<BoxDynElement> = self.get_ref().iter().map(|ele|ele.copied()).collect();
			Elements::with_nodes(nodes)
		}
	}
}

impl<'a> Elements<'a> {
	// when feature 'destroy' or 'insertion' is open
	cfg_feat_mutation! {
		/// Remove the Elements set.
		///
		/// ```
		/// use visdom::Vis;
		/// use visdom::types::BoxDynError;
		/// fn main()-> Result<(), BoxDynError>{
		///   let html = r##"
		///     <html>
		///       <head>
		///         <title>document</title>
		///       </head>
		///       <body>
		///         <dl>
		///           <dt>Title</dt>
		///           <dd><span>item1</span></dd>
		///           <dd class="item2"><span>item2</span></dd>
		///           <dd class="item3"><!--comment-->item3</dd>
		///         </dl>
		///       </body>
		///     </html>
		///   "##;
		///   let doc = Vis::load(html)?;
		///   let dl = doc.find("dl");
		///   let items = dl.children("");
		///   assert_eq!(items.length(), 4);
		///   // remove the dt element
		///   items.filter("dt").remove();
		///   let now_items = dl.children("");
		///   assert_eq!(now_items.length(), 3);
		///   Ok(())
		/// }
		/// ```
		pub fn remove(self) {
			for ele in self.into_iter() {
				if let Some(parent) = ele.parent().as_mut() {
					parent.remove_child(ele);
				}
			}
		}

		/// Clear all the nodes in the Elements set.
		///
		/// ```
		/// use visdom::Vis;
		/// use visdom::types::BoxDynError;
		/// fn main()-> Result<(), BoxDynError>{
		///   let html = r##"
		///     <html>
		///       <head>
		///         <title>document</title>
		///       </head>
		///       <body>
		///         <dl>
		///           <dt>Title</dt>
		///           <dd><span>item1</span></dd>
		///           <dd class="item2"><span>item2</span></dd>
		///           <dd class="item3"><!--comment-->item3</dd>
		///         </dl>
		///       </body>
		///     </html>
		///   "##;
		///   let doc = Vis::load(html)?;
		///   let mut dl = doc.find("dl");
		///   let items = dl.children("");
		///   assert_eq!(items.length(), 4);
		///   // clear the dl
		///   dl.empty();
		///   let now_items = dl.children("");
		///   assert_eq!(now_items.length(), 0);
		///   Ok(())
		/// }
		/// ```
		pub fn empty(&mut self) -> &mut Self {
			self.set_text("");
			self
		}

	}
	// when feature 'insertion' is open
	cfg_feat_insertion! {
		// `insert`
		fn insert(&mut self, dest: &Elements, position: &InsertPosition) {
			for ele in self.get_mut_ref() {
				for inserted in dest.get_ref().iter().rev() {
					ele.insert_adjacent(position, inserted);
				}
			}
		}
		// `replace`
		fn replace(&mut self, dest: &Elements){
			for ele in self.get_mut_ref() {
				for inserted in dest.get_ref().iter().rev() {
					ele.replace_with(inserted);
				}
			}
		}
		/// Append the parameter Elements to the child before the tag end of the current Elements set.
		///
		/// ```
		/// use visdom::Vis;
		/// use visdom::types::BoxDynError;
		/// fn main()-> Result<(), BoxDynError>{
		///   let html = r##"
		///     <html>
		///       <head>
		///         <title>document</title>
		///       </head>
		///       <body>
		///         <dl>
		///           <dt>Title</dt>
		///           <dd><span>item1</span></dd>
		///           <dd class="item2"><span>item2</span></dd>
		///           <dd class="item3"><!--comment-->item3</dd>
		///         </dl>
		///       </body>
		///     </html>
		///   "##;
		///   let doc = Vis::load(html)?;
		///   let mut dl = doc.find("dl");
		///   let items = dl.children("");
		///   assert_eq!(items.length(), 4);
		///   assert_eq!(items.last().is(".item3"), true);
		///   // now append item2 to the last child
		///   let mut item2 = items.filter(".item2");
		///   dl.append(&mut item2);
		///   let now_items = dl.children("");
		///   assert_eq!(now_items.last().is(".item2"), true);
		///   // append a new document fragement
		///   let mut append_dd = Vis::load(r#"<dd class="item4">item4</dd>"#)?;
		///   dl.append(&mut append_dd);
		///   let now_items = dl.children("");
		///   assert_eq!(now_items.last().is(".item4"), true);
		///   Ok(())
		/// }
		/// ```
		pub fn append(&mut self, elements: &mut Elements) -> &mut Self {
			self.insert(elements, &InsertPosition::BeforeEnd);
			self
		}
		/// Same as `append`, but exchange the caller and the parameter target.
		pub fn append_to(&mut self, elements: &mut Elements) -> &mut Self {
			elements.append(self);
			self
		}
		/// Append the parameter Elements to the child after the tag start of the current Elements set.
		pub fn prepend(&mut self, elements: &mut Elements) -> &mut Self {
			self.insert(elements, &InsertPosition::AfterBegin);
			self
		}
		/// Same as `prepend`, but exchange the caller and the parameter target.
		pub fn prepend_to(&mut self, elements: &mut Elements) -> &mut Self {
			elements.prepend(self);
			self
		}
		/// Insert the each element in the current Elements set into the other Elements's element's before position.
		///
		/// ```
		/// use visdom::Vis;
		/// use visdom::types::BoxDynError;
		/// fn main()-> Result<(), BoxDynError>{
		///   let html = r##"
		///     <html>
		///       <head>
		///         <title>document</title>
		///       </head>
		///       <body>
		///         <dl>
		///           <dt>Title</dt>
		///           <dd><span>item1</span></dd>
		///           <dd class="item2"><span>item2</span></dd>
		///           <dd class="item3"><!--comment-->item3</dd>
		///         </dl>
		///       </body>
		///     </html>
		///   "##;
		///   let doc = Vis::load(html)?;
		///   let mut dl = doc.find("dl");
		///   let items = dl.children("");
		///   assert_eq!(items.length(), 4);
		///   assert_eq!(items.last().is(".item3"), true);
		///   // now insert item3 before item2
		///   let mut item2 = items.filter(".item2");
		///   let mut item3 = items.filter(".item3");
		///   item3.insert_before(&mut item2);
		///   let now_items = dl.children("");
		///   assert_eq!(now_items.last().is(".item2"), true);
		///   // insert a new item0
		///   let mut insert_dd = Vis::load(r#"<dd class="item0">item0</dd>"#)?;
		///   let mut first_dd = dl.children("dd").first();
		///   insert_dd.insert_before(&mut first_dd);
		///   let now_dds = dl.children("dd");
		///   assert_eq!(now_dds.first().is(".item0"), true);
		///   Ok(())
		/// }
		/// ```
		pub fn insert_before(&mut self, elements: &mut Elements) -> &mut Self {
			elements.before(self);
			self
		}
		/// Same as `insert_before`, but exchange the caller and the parameter target.
		pub fn before(&mut self, elements: &mut Elements) -> &mut Self {
			// insert the elements before self
			self.insert(elements, &InsertPosition::BeforeBegin);
			self
		}
		/// Insert the each element in the current Elements set into the other Elements's element's after position.
		pub fn insert_after(&mut self, elements: &mut Elements) -> &mut Self {
			elements.after(self);
			self
		}
		/// Same as `insert_after`, but exchange the caller and the parameter target.
		pub fn after(&mut self, elements: &mut Elements) -> &mut Self {
			// insert the elements after self
			self.insert(elements, &InsertPosition::AfterEnd);
			self
		}
		///  Replace each element in the set of matched elements with the provided new set of elements.
		///
		/// ```
		/// use visdom::Vis;
		/// use visdom::types::BoxDynError;
		/// fn main()-> Result<(), BoxDynError>{
		///   let html = r##"
		///     <html>
		///       <head>
		///         <title>document</title>
		///       </head>
		///       <body>
		///         <dl>
		///           <dt>Title</dt>
		///           <dd><span>item1</span></dd>
		///           <dd class="item2"><span>item2</span></dd>
		///           <dd class="item3"><!--comment-->item3</dd>
		///         </dl>
		///       </body>
		///     </html>
		///   "##;
		///   let doc = Vis::load(html)?;
		///   let mut dl = doc.find("dl");
		///   let mut dt = dl.children("dt");
		///   assert_eq!(dt.length(), 1);
		///   assert_eq!(dl.children("dd").length(), 3);
		///   // now replace dt with dd
		///   let mut new_dd = Vis::load("<dd>replace</dd>")?;
		///   dt.replace_with(&mut new_dd);
		///   assert_eq!(dl.children("dd").length(), 4);
		///   assert_eq!(dl.children("dt").length(), 0);
		///   assert_eq!(dl.children("dd").eq(0).text(), "replace");
		///   // replace with exist dd
		///   let dds = dl.children("");
		///   let mut first_dd = dds.first();
		///   let mut last_dd = dds.last();
		///   last_dd.replace_with(&mut first_dd);
		///   assert_eq!(dl.children("").length(), 3);
		///   assert_eq!(dl.children("").last().text(), "replace");
		///   Ok(())
		/// }
		/// ```
		pub fn replace_with(&mut self, elements: &mut Elements) -> &mut Self{
			self.replace(elements);
			self
		}
	}
}

impl<'a> IntoIterator for Elements<'a> {
	type Item = BoxDynElement<'a>;
	type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;
	fn into_iter(self) -> Self::IntoIter {
		Box::new(self.nodes.into_iter())
	}
}

#[cfg(test)]
mod tests {
	use std::collections::VecDeque;

	use super::{relation_of, ElementRelation};
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
