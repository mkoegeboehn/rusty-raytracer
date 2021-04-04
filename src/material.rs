use crate::vector3d::Vector3d;
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Material {
    pub diffuse_color: [u8; 3],
    pub albedo: [f64; 4],
    pub specular_exponent: f64,
    pub refractive_index: f64,
}

impl Material {
    pub const IVORY: Material = Material {
        diffuse_color: [100, 100, 75],
        albedo: [0.6, 0.3, 0.1, 0.0],
        specular_exponent: 50.0,
        refractive_index: 1.0,
    };
    pub const RED_RUBBER: Material = Material {
        diffuse_color: [75, 26, 26],
        albedo: [0.9, 0.1, 0.0, 0.0],
        specular_exponent: 10.0,
        refractive_index: 1.0,
    };

    pub const BLACK_RUBBER: Material = Material {
        diffuse_color: [10, 10, 10],
        albedo: [0.1, 0.1, 0.0, 0.0],
        specular_exponent: 10.0,
        refractive_index: 1.0,
    };

    pub const MIRROR: Material = Material {
        diffuse_color: [255, 255, 255],
        albedo: [0.0, 10.0, 0.8, 0.0],
        specular_exponent: 1425.0,
        refractive_index: 1.0,
    };

    pub const GLASS: Material = Material {
        diffuse_color: [153, 179, 204],
        albedo: [0.0, 0.5, 0.1, 0.8],
        specular_exponent: 125.0,
        refractive_index: 1.5,
    };
    pub fn new(
        diffuse_color: [u8; 3],
        albedo: [f64; 4],
        specular_exponent: f64,
        refractive_index: f64,
    ) -> Self {
        Self {
            diffuse_color,
            albedo,
            specular_exponent,
            refractive_index,
        }
    }
}
