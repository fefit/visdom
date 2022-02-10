
use super::BoxDynText;

pub struct Texts<'a> {
	nodes: Vec<BoxDynText<'a>>,
}

impl<'a> Texts<'a> {
	pub fn with_capacity(cap: usize) -> Self {
		Texts {
			nodes: Vec::with_capacity(cap),
		}
	}
	pub fn length(&self) -> usize {
		self.nodes.len()
	}
	pub fn is_empty(&self) -> bool {
		self.length() == 0
	}
	// get ref
	pub fn get_ref(&self) -> &Vec<BoxDynText<'a>> {
		&self.nodes
	}
	// get mut ref
	pub fn get_mut_ref(&mut self) -> &mut Vec<BoxDynText<'a>> {
		&mut self.nodes
	}
	// for each
	pub fn for_each<F>(&mut self, mut handle: F) -> &mut Self
	where
		F: FnMut(usize, &mut BoxDynText) -> bool,
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
		F: FnMut(usize, &mut BoxDynText) -> bool,
	{
		self.for_each(handle)
	}

	// filter_by
	pub fn filter_by<F>(&self, handle: F) -> Texts<'a>
	where
		F: Fn(usize, &BoxDynText) -> bool,
	{
		let mut result: Texts = Texts::with_capacity(self.length());
		for (index, ele) in self.get_ref().iter().enumerate() {
			if handle(index, ele) {
				result.get_mut_ref().push(
					ele
						.clone_node()
						.typed()
						.into_text()
						.expect("Text ele must can use 'into_text'."),
				);
			}
		}
		result
	}
	// remove
	pub fn remove(self) {
		for ele in self.into_iter() {
			ele.remove();
		}
	}
}

impl<'a> IntoIterator for Texts<'a> {
	type Item = BoxDynText<'a>;
	type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;
	fn into_iter(self) -> Self::IntoIter {
		Box::new(self.nodes.into_iter())
	}
}
