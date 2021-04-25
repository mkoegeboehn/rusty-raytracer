use crate::material::Material;
use crate::V3f;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(Intersect, Matter, NormalVector)]
pub enum Entity {
    Sphere,
    Triangle,
}
#[enum_dispatch]
pub trait Intersect {
    fn intersect(&self, origin: V3f, direction: V3f) -> Option<f64>;
}
#[enum_dispatch]
pub trait NormalVector {
    fn norm(&self, point: &V3f) -> V3f;
}

#[enum_dispatch]
pub trait Matter {
    fn material(&self) -> &Material;
}
#[derive(Clone, Copy, PartialEq)]
pub struct Triangle {
    pub vertices: [V3f; 3],
    pub material: Material,
}

impl Triangle {
    pub fn new(vertices: [V3f; 3], material: Material) -> Self {
        Self { vertices, material }
    }
}

impl NormalVector for Triangle {
    fn norm(&self, _: &V3f) -> V3f {
        (self.vertices[1] - self.vertices[0])
            .cross(self.vertices[2] - self.vertices[1])
            .normalize()
    }
}

impl Intersect for Triangle {
    fn intersect(&self, _: V3f, _: V3f) -> Option<f64> {
        todo!()
    }
}

impl Matter for Triangle {
    fn material(&self) -> &Material {
        &self.material
    }
}
#[derive(Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: V3f,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: V3f, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Intersect for Sphere {
    fn intersect(&self, origin: V3f, direction: V3f) -> Option<f64> {
        let ptc = self.center - origin;
        let dir = direction.normalize();
        let proj_length = ptc * dir;
        if proj_length < 0.0 {
            if ptc.length() > self.radius {
                None
            } else if ptc.length() == self.radius {
                Some(0.0)
            } else {
                let dist = (ptc.length_squared() - proj_length * proj_length).sqrt();
                Some((self.radius * self.radius - dist * dist).sqrt() + proj_length)
            }
        } else {
            let proj: V3f = dir * proj_length;
            let distance = (ptc - proj).length();
            if distance == self.radius {
                Some(proj_length)
            } else if distance < self.radius {
                let offset = (self.radius * self.radius - distance * distance).sqrt();
                if ptc.length() < self.radius {
                    Some(proj_length + offset)
                } else {
                    Some(proj_length - offset)
                }
            } else {
                None
            }
        }
    }
}

impl NormalVector for Sphere {
    fn norm(&self, point: &V3f) -> V3f {
        (*point - self.center).normalize()
    }
}

impl Matter for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }
}
