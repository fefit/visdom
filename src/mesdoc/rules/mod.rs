use crate::mesdoc::selector::rule::{self, add_rules, RuleItem};
pub(crate) mod all;
pub(crate) mod attr;
pub(crate) mod class;
pub(crate) mod id;
pub(crate) mod name;
pub(crate) mod pseudo;
pub(crate) fn init() {
	// init rule
	rule::init();
	// add rules
	let mut rules: Vec<RuleItem> = Vec::with_capacity(20);
	// keep the init order
	class::init(&mut rules);
	id::init(&mut rules);
	name::init(&mut rules);
	attr::init(&mut rules);
	pseudo::init(&mut rules);
	all::init(&mut rules);
	add_rules(rules);
}
