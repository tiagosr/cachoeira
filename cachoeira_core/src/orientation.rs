use cgmath::{
    Matrix3,
    Vector3,
};

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Orientation {
    pub forward: Vector3<f32>,
    pub right: Vector3<f32>,
    pub up: Vector3<f32>,
}

impl From<Matrix3<f32>> for Orientation {
    fn from(mat: Matrix3<f32>) -> Self {
        Orientation { forward: -mat.z, right: mat.x, up: mat.y }
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            forward: -Vector3::unit_z(),
            right: Vector3::unit_x(),
            up: Vector3::unit_y()
        }
    }
}