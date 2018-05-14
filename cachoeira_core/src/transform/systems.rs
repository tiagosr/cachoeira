use hibitset::BitSet;

use specs::Prelude::{
	Entities,
	InsertedFlag,
	Join,
	ModifiedFlag,
	ReadExpect,
	ReadStorage,
	ReaderId,
	Resources,
	System,
	WriteStorage
};

use transform::{
	GlobalTransform,
	HierarchyEvent,
	Parent,
	ParentHierarchy,
	Transform
};


pub struct TransformSystem {
	local_modified: BitSet,
	global_modified: BitSet,
	
	inserted_local_id: Option<ReaderId<InsertedFlag>>,
	modified_local_id: Option<ReaderId<ModifiedFlag>>,

	parent_events_id: Option<ReaderId<HierarchyEvent>>,
}

impl TransformSystem {
	pub fn new() -> TransformSystem {
		TransformSystem {
			inserted_local_id: None,
			modified_local_id: None,
			parent_events_id: None,
			local_modified: BitSet::default(),
			global_modified: BitSet::default(),
		}
	}
}

impl<'a> System<'a> for TransformSystem {
	type SystemData = {
		Entities<'a>,
		ReadExpect<'a, ParentHierarchy>,
		ReadStorage<'a, Transform>,
		ReadStorage<'a, Parent>,
		WriteStorage<'a, GlobalTransform>,
	};

	fn run(&mut self, (entities, hierarchy, locals, parents, mut globals): Self::SystemData) {
		#[cfg(feature="profiler")]
		profile_scope!("transform_system");

		self.local_modified.clear();
		self.global_modified.clear();

		locals.populate_inserted(
			self.inserted_local_id.as_mut().unwrap(),
			&mut self.local_modified,
		);
		locals.populate_modified(
			self.modified_local_id.as_mut().unwrap(),
			&mut self.local_modified,
		);

		for event in hierarchy
			.changed()
			.read(self.parent_events_id.as_mut().unwrap())
		{
			match *event {
				// remove all entities tagged to be removed
				HierarchyEvent::Removed(entity) => {
					if let Err(err) = entities.delete(entity) {
						error!("Failure removing entity {:?}: {}", entity, err);
					}
				}
				// collect all modified entities
				HierarchyEvent::Modified(entity) => {
					self.local_modified.add(entity.id());
				}
			}
		}

		// chain children to parents that were transformed previously
		for (entity, _, local, global, _) in (
			&*entities,
			&self.local_modified,
			&locals,
			&mut globals,
			!&parents,
		).join()
		{
			self.global_modified.add(entity.id());
			global.0 = local.matrix();
			debug_assert!(
				global.is_finite(),
				format!("Entity {:?} had a non-finite Transform", entity)
			);
		}

		// compute the modified transforms with the parents
		for entity in hierarchy.all() {
			let self_dirty = self.local_modified.contains(entity.id());
			match (parents.get(*entity), locals.get(*entity)) {
				(Some(parent), Some(local)) => {
					let parent_dirty = self.global_modified.contains(parent.entity.id());
					if parent_dirty || self_dirty {
						let combined_transform =
							if let Some(parent_global) = globals.get(parent.entity) {
								(parent_global.0 * local.matrix()).into()
							} else {
								local.matrix()
							};

						if let Some(global) = globals.get_mut(*entity) {
							self.global_modified.add(entity.id());
							global.0 = combined_transform.into();
						}
					}
				}
				_ => (),
			}
		}
	}

	fn setup(&mut self, res: &mut Resources) {
		use specs::Prelude::SystemData;
		Self::SystemData::Setup(res);
		let mut hierarchy = res.fetch_mut::<ParentHierarchy>();
		let mut locals = WriteStorage::<Transform>::fetch(res);
		self.parent_events_id = Some(hierarchy.track());
		self.inserted_local_id = Some(locals.track_inserted());
		self.modified_local_id = Some(locals.track_modified());
	}
}

