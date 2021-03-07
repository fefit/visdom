// attr class
pub const ATTR_CLASS: &str = "class";
// default elements initial node length
pub const DEF_NODES_LEN: usize = 5;
// priorities
/*
** different from css selector priority
** most time, the name selector parse faster than attribute selector
** https://developer.mozilla.org/en-US/docs/Web/CSS/Specificity
*/
pub const PRIORITY_ALL_SELECTOR: u32 = 0;
pub const PRIORITY_ATTR_SELECTOR: u32 = 10;
pub const PRIORITY_PSEUDO_SELECTOR: u32 = 10;
pub const PRIORITY_NAME_SELECTOR: u32 = 100;
pub const PRIORITY_CLASS_SELECTOR: u32 = 1000;
pub const PRIORITY_ID_SELECTOR: u32 = 10000;
// selector names
pub const NAME_SELECTOR_ALL: &str = "all";
pub const NAME_SELECTOR_ATTR: &str = "attr";
pub const NAME_SELECTOR_NAME: &str = "name";
pub const NAME_SELECTOR_CLASS: &str = "class";
pub const NAME_SELECTOR_ID: &str = "id";
// alias selectors
pub const SELECTOR_ALIAS_NAME_HEADER: (&str, &str) = (":header", "h1,h2,h3,h4,h5,h6");
pub const SELECTOR_ALIAS_NAME_SUBMIT: (&str, &str) =
	(":submit", "input[type='submit'],button[type='submit']");
pub const SELECTOR_ALIAS_NAME_INPUT: (&str, &str) = (":input", "input,select,textarea,button");
