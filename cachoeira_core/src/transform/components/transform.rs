use std::borrow::Borrow;

use cgmath::{Matrix4, One};
use specs::prelude::{
	Component,
	DenseVecStorage,
	FlaggedStorage,
};

#[derive(Debug, Copy, Clone)]
pub struct GlobalTransform(pub Matrix4<f32>);

impl GlobalTransform {
	pub fn is_finite(&self) -> bool {
		for i in 0..4 {
			for j in 0..4 {
				if !self.0[i][j].is_finite() {
					return false;
				}
			}
		}
		true
	}
}

impl Component for GlobalTransform {
	type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl Default for GlobalTransform {
	fn default() -> Self { GlobalTransform(Matrix4::one()) }
}

impl GlobalTransform {
	pub fn new() -> Self { Default::default() }
}

impl From<[[f32; 4]; 4]> for GlobalTransform {
	fn from(matrix: [[f32; 4]; 4]) -> Self { GlobalTransform(matrix.into()) }
}

impl Into<[[f32; 4]; 4]> for GlobalTransform {
	fn into(self) -> [[f32; 4]; 4] { self.0.into() }
}

impl AsRef<[[f32; 4]; 4]> for GlobalTransform {
	fn as_ref(&self) -> &[[f32; 4]; 4] { self.0.as_ref() }
}

impl Borrow<[[f32; 4]; 4]> for GlobalTransform {
	fn borrow(&self) -> &[[f32; 4]; 4] { self.0.as_ref() }
}
