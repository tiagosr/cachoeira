use cgmath::{
	Angle, Array,
	Basis2,
	Deg, Rad,
	ElementWise, EuclideanSpace,
	Euler, InnerSpace,
	Matrix3, Matrix4,
	One, Zero,
	Point2, Point3,
	Quaternion,
	Rotation, Rotation2, Rotation3,
	Transform as CgTransform,
	Vector2, Vector3, Vector4,
};

use orientation::Orientation;
use specs::prelude::{
	Component,
	DenseVecStorage,
	FlaggedStorage,
};


#[derive(Clone, Debug, PartialEq)]
pub struct Transform {
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
    pub translation: Vector3<f32>
}

impl Transform {
	#[inline]
	pub fn look_at(&mut self, position: Point3<f32>, up: Vector3<f32>) -> &mut Self {
		self.rotation = Quaternion::look_at(Point3::from_vec(self.translation) - position, up);
		debug_assert!(
			self.rotation.s.is_finite()
			&& self.rotation.v.x.is_finite()
			&& self.rotation.v.y.is_finite()
			&& self.rotation.v.z.is_finite(),
			"'look_at' should be finite to be useful"
		);
		self
	}

	#[inline]
	pub fn matrix(&self) -> Matrix4<f32> {
		let quat: Matrix3<f32> = self.rotation.into();
		let x = Vector4 {
			x: quat.x.x * self.scale.x,
			y: quat.x.y * self.scale.y,
			z: quat.x.z * self.scale.z,
			w: 0.0,
		};
		let y = Vector4 {
			x: quat.y.x * self.scale.x,
			y: quat.y.y * self.scale.y,
			z: quat.y.z * self.scale.z,
			w: 0.0,
		};
		let z = Vector4 {
			x: quat.z.x * self.scale.x,
			y: quat.z.y * self.scale.y,
			z: quat.z.z * self.scale.z,
			w: 0.0,
		};
		let mat = Matrix4 {
			x, y, z, w: self.translation.extend(1.0)
		};
		mat
	}

	pub fn orientation(&self) -> Orientation {
		Orientation::from(Matrix3::from(self.rotation))
	}

	#[inline]
	pub fn move_global(&mut self, translation: Vector3<f32>) -> &mut Self {
		self.translation += translation;
		self
	}

	#[inline]
	pub fn move_local(&mut self, translation: Vector3<f32>) -> &mut Self {
		self.translation += self.rotation * translation;
		self
	}

	#[inline]
	pub fn move_along_global(&mut self, direction: Vector3<f32>, distance: f32) -> &mut Self {
		if !ulps_eq!(direction, Zero::zero()) {
			self.translation += direction.normalize_to(distance);
		}
		self
	}

	#[inline]
	pub fn move_forward(&mut self, amount: f32) -> &mut Self {
		self.move_local(Vector3::new(0.0, 0.0, -amount));
	}

	#[inline]
	pub fn move_backward(&mut self, amount: f32) -> &mut Self {
		self.move_local(Vector3::new(0.0, 0.0, amount));
	}

	#[inline]
	pub fn move_left(&mut self, amount: f32) -> &mut Self {
		self.move_local(Vector3::new(-amount, 0.0, 0.0));
	}

	#[inline]
	pub fn move_right(&mut self, amount: f32) -> &mut Self {
		self.move_local(Vector3::new(amount, 0.0, 0.0));
	}

	#[inline]
	pub fn move_up(&mut self, amount: f32) -> &mut Self {
		self.move_local(Vector3::new(0.0, amount, 0.0));
	}

	#[inline]
	pub fn move_down(&mut self, amount: f32) -> &mut Self {
		self.move_local(Vector3::new(0.0, -amount, 0.0));
	}

	#[inline]
	pub fn pitch_global(&mut self, angle: Deg<f32>) -> &mut Self {
		self.rotate_global(Vector3::unit_x(), angle)
	}

	#[inline]
	pub fn pitch_local(&mut self, angle: Deg<f32>) -> &mut Self {
		self.rotate_local(Vector3::unit_x(), angle)
	}



	#[inline]
	pub fn rotate_global<A: Into<Rad<f32>>>(&mut self, axis: Vector3<f32>, angle: A) -> &mod Self {
		debug_assert!(
			!ulps_eq!(axis.magnitude2(), Zero::zero()),
			"Axis of rotation must not be zero"
		);
		let q = Quaternion::from_axis_angle(axis.normalize(), angle);
		self.rotation = q * self.rotation;
		self
	}

	#[inline]
	pub fn rotate_local<A: Into<Rad<f32>>>(&mut self, axis: Vector3<f32>, angle: A) -> &mod Self {
		debug_assert!(
			!ulps_eq!(axis.magnitude2(), Zero::zero()),
			"Axis of rotation must not be zero"
		);
		let q = Quaternion::from_axis_angle(axis.normalize(), angle);
		self.rotation =self.rotation * q;
		self
	}

	pub fn set_position(&mut self, position: Vector3<f32>) -> &mut Self {
		self.translation = position;
		self
	}

	pub fn set_rotation<A>(&mut self, x: A, y: A, z: A) -> &mut Self
	where
		A: Angle<Unitless = f32>,
		Rad<f32>: From<A>,
	{
		self.rotation = Quaternion::from(Euler {x, y, z});
		self
	}

	pub fn view_matrix(&self) -> Matrix4<f32> {
		use cgmath::SquareMatrix;
		self.matrix().invert().unwrap()
	}
}

impl Default for Transform {
	fn default() -> Self {
		Transform {
			translation: Vector3::zero(),
			rotation: Quaternion::one(),
			scale: Vector3::from_value(1.),
		}
	}
}


impl Component for Transform {
	type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl CgTransform<Point3<f32>> for Transform {
	fn one() -> Self { Default::default() }

	fn look_at(eye: Point3<f32>, center: Point3<f32>, up: Vector3<f32>) -> Self {
		let rotation = Quaternion::look_at(center - eye, up);
		let translation = rotation.rotate_vector(Point3::origin() - eye);
		let scale = Vector3::from_value(1.);
		Self { scale, rotation, translation }
	}

	fn transform_vector(&self, vec: Vector3<f32>) -> Vector3<f32> {
		self.rotation.rotate_vector(vec.mul_element_wise(self.scale))
	}

	fn inverse_transform_vector(&self, vec: Vector3<f32>) -> Option<Vector3<f32>> {
		if ulps_eq!(self.scale, &Vector3::zero()) {
			None
		} else {
			Some(self.rotation.invert().rotate_vector(vec.div_element_wise(self.scale)))
		}
	}

	fn transform_point(&self, point: Point3<f32>) -> Point3<f32> {
		let p = Point3::from_vec(point.to_vec().mul_element_wise(self.scale));
		self.rotation.rotate_point(p) + self.translation
	}

	fn concat(&self, other: &Self) -> Self {
		Self {
			scale: self.scale.mul_element_wise(other.scale),
			rotation: self.rotation * other.rotation,
			translation: self.rotation.rotate_vector(other.translation.mul_element_wise(self.scale)) + self.translation
		}
	}

	fn inverse_transform(&self) -> Option<Self> {
		if ulps_eq!(self.scale, Vector3::zero()) {
			None
		} else {
			let scale = 1. / self.scale;
			let rotation = self.rotation.invert();
			let translation = rotation.rotate_vector(self.translation).mul_element_wise(-scale);
			Some(Self { translation, rotation, scale })
		}
	}
}

impl CgTransform<Point2<f32>> for Transform {
	fn one() -> Self { Default::default() }
	fn look_at(_eye: Point2<f32>, _center: Point2<f32>, _up: Vector2<f32>) -> Self {
		panic!("Can't compute look at for 2D")
	}
	fn transform_vector(&self, vec: Vector2<f32>) -> Vector2<f32> {
		let rot: Basis2<f32> = Rotation2::from_angle(-Euler::from(self.rotation).z);
		rot.rotate_vector(vec.mul_element_wise(self.scale.truncate()))
	}
	fn inverse_transform_vector(&self, vec: Vector2<f32>) -> Option<Vector2<f32>> {
		if ulps_eq!(self.scale, &Vector2::zero()) {
			None
		} else {
			let rot: Basis2<f32> = Rotation2::from_angle(-Euler::from(self.rotation).z);
			Some(rot.rotate_vector(vec.div_element_wise(self.scale.truncate())))
		}
	}
	fn transform_point(&self, point: Point2<f32>) -> Point2<f32> {
		let p = Point2::from_vec(point.to_vec().mul_element_wise(self.scale.truncate()));
		let rot: Basis2<f32> = Rotation2::from_angle(-Euler::from(self.rotation).z);
		rot.rotate_point(p) + self.translation.truncate()
	}
	fn concat(&self, other: &Self) -> Self {
		Self {
			scale: self.scale.mul_element_wise(other.scale),
			rotation: self.rotation * other.rotation,
			translation: self.rotation.rotate_vector(other.translation.mul_element_wise(self.scale))
							+ self.translation,
		}
	}
	fn inverse_transform(&self) -> Option<Self> {
		if ulps_eq!(self.scale, Vector2::zero()) {
			None
		} else {
			let scale = 1. / self.scale;
			let rotation = self.rotation.invert();
			let translation = rotation.rotate_vector(self.translation).mul_element_wise(-scale);
			Some(Self { translation, rotation, scale })
		}
	}
}
