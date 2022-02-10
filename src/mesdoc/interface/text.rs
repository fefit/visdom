use super::INodeTrait;
pub type BoxDynText<'a> = Box<dyn ITextTrait + 'a>;
pub trait ITextTrait: INodeTrait {
	cfg_feat_text! {
		// remove the ele
		fn remove(self: Box<Self>);
		// append text at the end
		fn append_text(&mut self, content: &str);
		// prepend text at the start
		fn prepend_text(&mut self, content: &str);
	}
}
