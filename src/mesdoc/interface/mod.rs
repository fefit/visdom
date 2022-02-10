// node trait
mod node;
pub use node::{BoxDynNode, IEnumTyped, INodeTrait, INodeType};
// element trait
mod element;
pub use element::{BoxDynElement, IAttrValue, IElementTrait, IFormValue, MaybeElement};
cfg_feat_insertion! {
	pub use element::InsertPosition;
}
// text trait
mod text;
pub use text::{BoxDynText, ITextTrait};
// document trait
mod document;
pub use document::{IDocumentTrait, IErrorHandle, MaybeDoc};
// uncare
mod uncare;
pub use uncare::{BoxDynUncareNode, IUncareNodeTrait};
// texts
cfg_feat_text! {
	mod texts;
	pub use texts::Texts;
}
// elements
mod elements;
pub use elements::Elements;
