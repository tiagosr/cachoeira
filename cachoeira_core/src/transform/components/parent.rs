use specs::prelude::{
	Component,
	DenseVecStorage,
	Entity,
	FlaggedStorage,
};
use specs_hierarchy::{
	Hierarchy,
	Parent as HParent,
};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Parent {
    pub entity: Entity,
}

impl Component for Parent {
	type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl HParent for Parent {
	fn parent_entity(&self) -> Entity { self.entity }
}