use std::ops::Neg;

use image::ImageBuffer;

mod vector3d;
use vector3d::Vector3d;

const BG_COLOR: [u8; 3] = [51, 179, 204];
fn main() {
    const IVORY: Material = Material {
        diffuse_color: [100, 100, 75],
        albedo: V2f {
            x: 0.6,
            y: 0.3,
            z: 0.0,
        },
        specular_exponent: 50.0,
    };
    const RED_RUBBER: Material = Material {
        diffuse_color: [75, 26, 26],
        albedo: V2f {
            x: 0.9,
            y: 0.1,
            z: 0.0,
        },
        specular_exponent: 10.0,
    };

    let spheres = vec![
        Sphere::new(V3f::new(-3.0, 0.0, -16.0), 2.0, IVORY),
        Sphere::new(V3f::new(-1.0, -1.5, -12.0), 2.0, RED_RUBBER),
        Sphere::new(V3f::new(1.5, -0.5, -18.0), 3.0, RED_RUBBER),
        Sphere::new(V3f::new(7.0, 5.0, -18.0), 4.0, IVORY),
    ];

    let lights = vec![
        Light::new(V3f::new(-20.0, 20.0, 20.0), 1.5),
        Light::new(V3f::new(30.0, 50.0, -25.0), 1.8),
        Light::new(V3f::new(30.0, 20.0, 30.0), 1.7),
    ];
    render(&spheres, &lights);
}

fn render(spheres: &[Sphere], lights: &[Light]) {
    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 768;
    // const FOV: f64 = std::f64::consts::FRAC_PI_2;
    const FOV: f64 = 1.0;
    let mut imgbuf = ImageBuffer::new(WIDTH, HEIGHT);

    for (i, j, pixel) in imgbuf.enumerate_pixels_mut() {
        let x: f64 =
            (2.0 * (i as f64 + 0.5) / (WIDTH as f64) - 1.0) * f64::tan(FOV / 2.0) * (WIDTH as f64)
                / (HEIGHT as f64);
        let y: f64 = -(2.0 * (j as f64 + 0.5) / (HEIGHT as f64) - 1.0) * f64::tan(FOV / 2.0);
        let dir = V3f::new(x, y, -1.0).normalize();
        *pixel = cast_ray(&V3f::with_value(0.0), &dir, spheres, lights);
    }

    imgbuf.save("renders/out.png").unwrap();
}

fn scene_intersect<'a>(
    origin: &V3f,
    dir: &V3f,
    spheres: &'a [Sphere],
) -> Option<(V3f, &'a Sphere)> {
    let mut spheres_dist: f64 = f64::MAX;
    let mut result = None;
    for sphere in spheres {
        if let Some(dist) = sphere.ray_intersect(*origin, *dir) {
            if dist < spheres_dist {
                spheres_dist = dist;
                let hit = *origin + *dir * dist;
                result = Some((hit, sphere))
            }
        }
    }
    result
}

fn cast_ray(origin: &V3f, dir: &V3f, spheres: &[Sphere], lights: &[Light]) -> image::Rgb<u8> {
    if let Some((point, sphere)) = scene_intersect(origin, dir, spheres) {
        let mut diffuse_light_intensity = 0.0;
        let mut specular_light_intensity = 0.0;
        for light in lights {
            let light_dir = (light.position - point).normalize();
            let normal_vector = sphere.norm(&point);
            diffuse_light_intensity += light.intensity * (light_dir * normal_vector).max(0.0);
            specular_light_intensity += (-reflect(&-light_dir, &normal_vector) * *dir)
                .max(0.0)
                .powf(sphere.material.specular_exponent)
                * light.intensity;
        }
        let mut pixel = sphere.material.diffuse_color;
        for subpixel in pixel.iter_mut() {
            *subpixel = (*subpixel as f64 * diffuse_light_intensity * sphere.material.albedo.x
                + 255.0 * specular_light_intensity * sphere.material.albedo.y)
                as u8;
        }
        image::Rgb(pixel)
    } else {
        image::Rgb(BG_COLOR)
    }
}

fn reflect(incident: &V3f, normal: &V3f) -> V3f {
    *incident - *normal * (2.0 * (*incident * *normal))
}

type V3f = Vector3d<f64>;
#[derive(Clone, Copy, PartialEq)]
struct Sphere {
    center: V3f,
    radius: f64,
    material: Material,
}

impl Sphere {
    fn new(center: V3f, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    fn ray_intersect(&self, origin: V3f, direction: V3f) -> Option<f64> {
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

    fn norm(&self, point: &V3f) -> V3f {
        (*point - self.center).normalize()
    }
}

type V2f = Vector3d<f64>;
#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Material {
    diffuse_color: [u8; 3],
    albedo: V2f,
    specular_exponent: f64,
}

impl Material {
    fn new(diffuse_color: [u8; 3], albedo: V2f, specular_exponent: f64) -> Self {
        Self {
            diffuse_color,
            albedo,
            specular_exponent,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Light {
    position: V3f,
    intensity: f64,
}

impl Light {
    fn new(position: V3f, intensity: f64) -> Self {
        Self {
            position,
            intensity,
        }
    }
}
