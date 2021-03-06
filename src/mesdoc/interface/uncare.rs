use super::INodeTrait;

pub type BoxDynUncareNode<'a> = Box<dyn IUncareNodeTrait + 'a>;
pub trait IUncareNodeTrait: INodeTrait {}
