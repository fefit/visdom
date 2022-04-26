pub mod pattern;
pub mod rule;
use self::{pattern::BoxDynPattern, rule::Matcher};
use crate::mesdoc::{constants::NAME_SELECTOR_ALL, error::Error};
use lazy_static::lazy_static;
pub use pattern::MatchedQueue;
use pattern::{exec, Matched};
use rule::{Rule, RULES};
use std::{
	str::FromStr,
	sync::{Arc, Mutex},
};

lazy_static! {
	static ref SPLITTER: Mutex<Vec<BoxDynPattern>> =
		Mutex::new(Rule::get_queues(r##"{regexp#(\s*[>,~+]\s*|\s+)#}"##));
	static ref ALL_RULE: Mutex<Option<Arc<Rule>>> = Mutex::new(None);
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Combinator {
	// descendants
	ChildrenAll,
	// children
	Children,
	// reverse for child
	Parent,
	// reverse for childrens
	ParentAll,
	// next all siblings
	NextAll,
	// next sibling
	Next,
	// reverse for next siblings
	PrevAll,
	// reverse for next sibling
	Prev,
	// siblings
	Siblings,
	// chain selectors
	Chain,
}

// change string to combinator
impl From<&str> for Combinator {
	fn from(comb: &str) -> Self {
		use Combinator::*;
		match comb {
			"" => ChildrenAll,
			">" => Children,
			"~" => NextAll,
			"+" => Next,
			_ => panic!("Not supported combinator string '{}'", comb),
		}
	}
}

impl Combinator {
	pub fn reverse(&self) -> Self {
		use Combinator::*;
		match self {
			ChildrenAll => ParentAll,
			Children => Parent,
			NextAll => PrevAll,
			Next => Prev,
			Chain => Chain,
			_ => panic!("Not supported combinator reverse for '{:?}'", self),
		}
	}
}

pub type SelectorSegment = (Matcher, Combinator);
#[derive(Default, Debug)]
pub struct QueryProcess {
	pub should_in: Option<SelectorGroupsItem>,
	pub query: SelectorGroupsItem,
}

#[derive(Default, Debug)]
pub struct Selector {
	pub process: Vec<QueryProcess>,
}

type SelectorGroupsItem = Vec<Vec<SelectorSegment>>;
type SelectorGroups = Vec<SelectorGroupsItem>;
impl Selector {
	pub fn new() -> Self {
		Selector {
			process: Vec::with_capacity(1),
		}
	}
	pub fn from_str(context: &str, use_lookup: bool) -> Result<Self, Error> {
		let chars: Vec<char> = context.chars().collect();
		let total_len = chars.len();
		let mut selector = Selector::new();
		if total_len > 0 {
			let mut index: usize = 0;
			let mut comb = Combinator::ChildrenAll;
			let mut prev_in = PrevInSelector::Begin;
			let mut last_in = prev_in;
			let mut groups: SelectorGroups = Vec::new();
			let splitter = SPLITTER.lock().unwrap();
			let rules = RULES.lock().unwrap();
			Selector::add_group(&mut groups);
			while index < total_len {
				let next_chars = &chars[index..];
				// first check if combinator
				if let Some((matched, len, _)) = Rule::exec_queues(&splitter, next_chars) {
					let op = matched[0].chars.iter().collect::<String>();
					let op = op.trim();
					if prev_in == PrevInSelector::Splitter {
						// wrong multiple combinator
						return Err(Error::InvalidSelector {
							context: String::from(context),
							reason: format!(
								"Wrong combinator '{}' at index {}",
								matched[0].chars.iter().collect::<String>(),
								index
							),
						});
					}
					// find the match
					index += len;
					// set combinator
					if op == "," {
						if prev_in != PrevInSelector::Selector {
							return Err(Error::InvalidSelector {
								context: String::from(context),
								reason: format!("Wrong empty selector before ',' at index  {}", index),
							});
						}
						Selector::add_group(&mut groups);
						comb = Combinator::ChildrenAll;
					} else {
						comb = Combinator::from(op);
					}
					// set prev is splitter
					if op.is_empty() {
						last_in = prev_in;
						prev_in = PrevInSelector::Splitter;
					} else {
						prev_in = PrevInSelector::Splitter;
						last_in = prev_in;
					}
					continue;
				}
				// then it must match a selector rule
				let mut is_new_item = true;
				if prev_in == PrevInSelector::Selector {
					comb = Combinator::Chain;
					is_new_item = false;
				} else {
					prev_in = PrevInSelector::Selector;
					last_in = prev_in;
				}
				let mut finded = false;
				for (_, r) in rules.iter() {
					if let Some((mut matched, len, queue_num)) = r.exec(next_chars) {
						// find the rule
						index += len;
						let queues = &r.queues;
						if queue_num == queues.len() {
							// push to selector
							Selector::add_group_item(&mut groups, (r.make(matched), comb), is_new_item);
							finded = true;
						} else if queues[queue_num].is_nested() {
							// nested selector
							let (len, nested_matched) = Selector::parse_until(
								&chars[index..],
								&queues[queue_num + 1..],
								&rules,
								&splitter,
								0,
							)?;
							index += len;
							matched.extend(nested_matched);
							Selector::add_group_item(&mut groups, (r.make(matched), comb), is_new_item);
							finded = true;
						}
						break;
					}
				}
				if !finded {
					// no splitter, no selector rule
					return Err(Error::InvalidSelector {
						context: String::from(context),
						reason: format!(
							"Unrecognized selector '{}' at index {}",
							next_chars.iter().collect::<String>(),
							index
						),
					});
				}
			}
			if last_in != PrevInSelector::Selector {
				return Err(Error::InvalidSelector {
					context: String::from(context),
					reason: String::from("Wrong selector rule at last"),
				});
			}
			// optimize groups to query process
			selector.optimize(groups, use_lookup);
		}
		Ok(selector)
	}
	// add a selector group, splitted by ','
	fn add_group(groups: &mut SelectorGroups) {
		groups.push(Vec::with_capacity(2));
	}
	// add a selector group item
	fn add_group_item(groups: &mut SelectorGroups, item: SelectorSegment, is_new: bool) {
		if let Some(last_group) = groups.last_mut() {
			if is_new {
				last_group.push(vec![item]);
			} else if let Some(last) = last_group.last_mut() {
				last.push(item);
			}
		}
	}
	// optimize the parse process
	fn optimize(&mut self, groups: SelectorGroups, use_lookup: bool) {
		let mut process: Vec<QueryProcess> = Vec::with_capacity(groups.len());
		for mut group in groups {
			// first optimize the chain selectors, the rule who's priority is bigger will apply first
			let mut max_index: usize = 0;
			let mut max_priority: u32 = 0;
			for (index, r) in group.iter_mut().enumerate() {
				let mut total_priority = 0;
				if r.len() > 1 {
					let chain_comb = r[0].1;
					r.sort_by(|a, b| b.0.priority.partial_cmp(&a.0.priority).unwrap());
					let mut now_first = &mut r[0];
					if now_first.1 != chain_comb {
						now_first.1 = chain_comb;
						total_priority += now_first.0.priority;
						for n in &mut r[1..] {
							n.1 = Combinator::Chain;
							total_priority += n.0.priority;
						}
					}
				}
				if use_lookup {
					total_priority = r.iter().map(|p| p.0.priority).sum();
					if total_priority > max_priority {
						max_priority = total_priority;
						max_index = index;
					}
				}
			}
			// if the first combinator is child, and the max_index > 1, use the max_index's rule first
			if use_lookup && max_index > 0 {
				let is_child = matches!(
					group[0][0].1,
					Combinator::Children | Combinator::ChildrenAll
				);
				if is_child {
					let query = group.split_off(max_index);
					let should_in = Some(group);
					process.push(QueryProcess { should_in, query });
					continue;
				}
			}
			process.push(QueryProcess {
				should_in: None,
				query: group,
			});
		}
		self.process = process;
	}
	// change the combinator
	pub fn head_combinator(&mut self, comb: Combinator) {
		for p in &mut self.process {
			let v = if let Some(should_in) = &mut p.should_in {
				should_in
			} else {
				&mut p.query
			};
			if let Some(rule) = v.get_mut(0) {
				let first_comb = rule[0].1;
				match first_comb {
					Combinator::ChildrenAll => rule[0].1 = comb,
					_ => {
						let segment = Selector::make_comb_all(comb);
						v.insert(0, vec![segment]);
					}
				};
			}
		}
	}
	// make '*' with combinator
	pub fn make_comb_all(comb: Combinator) -> SelectorSegment {
		let mut all_rule = ALL_RULE.lock().unwrap();
		if all_rule.is_none() {
			let rules = RULES.lock().unwrap();
			for (name, rule) in &rules[..] {
				if *name == NAME_SELECTOR_ALL {
					*all_rule = Some(Arc::clone(rule));
					break;
				}
			}
		}
		let cur_rule = Arc::clone(all_rule.as_ref().expect("All rule must add to rules"));
		let matcher = cur_rule.make(vec![]);
		(matcher, comb)
	}
	// build a selector from a segment
	pub fn from_segment(segment: SelectorSegment) -> Self {
		let process = QueryProcess {
			query: vec![vec![segment]],
			should_in: None,
		};
		Selector {
			process: vec![process],
		}
	}
	// parse until
	pub fn parse_until(
		chars: &[char],
		until: &[BoxDynPattern],
		rules: &[(&str, Arc<Rule>)],
		splitter: &[BoxDynPattern],
		level: usize,
	) -> Result<(usize, MatchedQueue), Error> {
		let mut index = 0;
		let total = chars.len();
		let mut matched: MatchedQueue = Vec::with_capacity(until.len() + 1);
		while index < total {
			let next_chars = &chars[index..];
			if let Some((_, len, _)) = Rule::exec_queues(splitter, next_chars) {
				index += len;
				continue;
			}
			let mut finded = false;
			for (_, r) in rules.iter() {
				if let Some((_, len, queue_num)) = r.exec(next_chars) {
					let queues = &r.queues;
					// find the rule
					index += len;
					if queue_num == queues.len() {
						// push to selector
						finded = true;
					} else {
						let (nest_count, _) = Selector::parse_until(
							&chars[index..],
							&queues[queue_num + 1..],
							rules,
							splitter,
							level + 1,
						)?;
						index += nest_count;
					}
					break;
				}
			}
			if !finded {
				if level == 0 {
					matched.push(Matched {
						chars: chars[0..index].to_vec(),
						name: "selector",
						..Default::default()
					});
				}
				if !until.is_empty() {
					let (util_matched, count, queue_num, _) = exec(until, &chars[index..]);
					if queue_num != until.len() {
						let context = chars[index..].iter().collect::<String>();
						return Err(Error::InvalidSelector {
							context,
							reason: format!("Nested selector parse error at index {}", index),
						});
					} else {
						index += count;
						if level == 0 {
							matched.extend(util_matched);
						}
					}
				}
				break;
			}
		}
		Ok((index, matched))
	}
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum PrevInSelector {
	Begin,
	Splitter,
	Selector,
}

impl FromStr for Selector {
	type Err = Error;
	fn from_str(selector: &str) -> Result<Self, Self::Err> {
		Selector::from_str(selector, true)
	}
}

#[cfg(test)]
mod tests {
	use super::{Combinator, QueryProcess, Selector};
	#[test]
	fn test_default() {
		let def_selector = Selector::default();
		assert!(def_selector.process.is_empty());
		let def_process = QueryProcess::default();
		assert!(def_process.should_in.is_none());
		assert!(def_process.query.is_empty());
	}
	#[test]
	fn test_combinator() {
		let comb: Combinator = ">".into();
		assert_eq!(comb, Combinator::Children);
		assert_eq!(comb.reverse(), Combinator::Parent);
	}

	#[test]
	fn test_combinator_reverse() {
		assert_eq!(Combinator::Chain.reverse(), Combinator::Chain);
	}

	#[test]
	#[should_panic]
	fn test_combinator_unable_reverse() {
		let _ = Combinator::Parent.reverse();
	}

	#[test]
	#[should_panic]
	fn test_wrong_combinator_string() {
		let _: Combinator = "<".into();
	}
}
