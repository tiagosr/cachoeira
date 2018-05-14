
use specs::prelude::DispatcherBuilder;
use specs_hierarchy::HierarchySystem;

use bundle::{Result, SystemBundle};
use transform::*;



#[derive(Default)]
pub struct TransformBundle<'a> {
	dep: &'a[&'a str],
}

impl<'a> TransformBundle<'a> {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn with_dep(mut self, dep: &'a[&'a str]) -> Self {
		self.dep = dep;
		self
	}
}

impl<'a, 'b, 'c> SystemBundle<'a, 'b> for TransformBundle<'c> {
	fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
	    builder.add(
	    	HierarchySystem::<Parent>::new(),
	    	"parent_hierarchy_system",
	    	self.dep,
	    );
	    builder.add(
	    	TransformSystem::new(),
	    	"transform_system",
	    	&["parent_hierarchy_system"],
	    );
	    Ok(());
	}
}