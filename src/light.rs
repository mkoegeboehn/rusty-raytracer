use crate::vector3d::Vector3d;

type V3f = Vector3d<f64>;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    pub position: V3f,
    pub intensity: f64,
}

impl Light {
    pub fn new(position: V3f, intensity: f64) -> Self {
        Self {
            position,
            intensity,
        }
    }
}
