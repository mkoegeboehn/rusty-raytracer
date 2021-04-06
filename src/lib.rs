use image::ImageBuffer;

mod vector3d;
use vector3d::Vector3d;
pub mod material;
use material::Material;
pub mod light;
use light::Light;
pub mod entity;
use entity::{Entity, Intersect, Matter, NormalVector, Sphere};

pub type V3f = Vector3d<f64>;

const BG_COLOR: [u8; 3] = [51, 179, 204];

pub fn render(entities: &[Entity], lights: &[Light]) {
    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 768;
    // const FOV: f64 = std::f64::consts::FRAC_PI_3;
    const FOV: f64 = 1.0;
    let mut imgbuf = ImageBuffer::new(WIDTH, HEIGHT);

    for (i, j, pixel) in imgbuf.enumerate_pixels_mut() {
        let dir_x: f64 = (i as f64 + 0.5) - WIDTH as f64 / 2.0;
        let dir_y: f64 = -(j as f64 + 0.5) + HEIGHT as f64 / 2.0;
        let dir_z: f64 = -(HEIGHT as f64) / (2.0 * (FOV / 2.0).tan());
        let dir = V3f::new(dir_x, dir_y, dir_z).normalize();
        *pixel = cast_ray(&V3f::with_value(0.0), &dir, entities, lights, 4);
    }

    imgbuf.save("renders/out.png").unwrap();
}

fn scene_intersect<'a>(
    origin: &V3f,
    dir: &V3f,
    entities: &'a [Entity],
) -> Option<(V3f, &'a Entity)> {
    let mut spheres_dist: f64 = f64::MAX;
    let mut result = None;
    for entity in entities {
        if let Some(dist) = entity.intersect(*origin, *dir) {
            if dist < spheres_dist {
                spheres_dist = dist;
                let hit = *origin + *dir * dist;
                result = Some((hit, entity))
            }
        }
    }
    result
}

fn cast_ray(
    origin: &V3f,
    dir: &V3f,
    entities: &[Entity],
    lights: &[Light],
    recursion_depth: i32,
) -> image::Rgb<u8> {
    const PERTURB: f64 = 1e-3;
    if recursion_depth < 0 {
        image::Rgb(BG_COLOR)
    } else if let Some((point, entity)) = scene_intersect(origin, dir, entities) {
        let normal_vector = entity.norm(&point);
        let reflect_dir = reflect(dir, &normal_vector).normalize();
        let refract_dir = refract(
            &dir.normalize(),
            &normal_vector,
            entity.material().refractive_index,
        )
        .normalize();
        let reflect_orig = if reflect_dir * normal_vector < 0.0 {
            point - normal_vector * PERTURB
        } else {
            point + normal_vector * PERTURB
        };
        let refract_orig = if refract_dir * normal_vector < 0.0 {
            point - normal_vector * PERTURB
        } else {
            point + normal_vector * PERTURB
        };
        let reflect_color = cast_ray(
            &reflect_orig,
            &reflect_dir,
            entities,
            lights,
            recursion_depth - 1,
        );
        let refract_color = cast_ray(
            &refract_orig,
            &refract_dir,
            entities,
            lights,
            recursion_depth - 1,
        );

        let mut diffuse_light_intensity = 0.0;
        let mut specular_light_intensity = 0.0;
        for light in lights {
            let light_dir = light.position - point;
            let light_distance = light_dir.length();
            let light_dir = light_dir / light_distance;
            let shadow_orig = if light_dir * normal_vector < 0.0 {
                point - normal_vector * PERTURB
            } else {
                point + normal_vector * PERTURB
            };
            if let Some((shadow_point, _)) = scene_intersect(&shadow_orig, &light_dir, entities) {
                if (shadow_point - shadow_orig).length() < light_distance {
                    continue;
                }
            }
            diffuse_light_intensity += light.intensity * (light_dir * normal_vector).max(0.0);
            specular_light_intensity += (-reflect(&-light_dir, &normal_vector) * *dir)
                .max(0.0)
                .powf(entity.material().specular_exponent)
                * light.intensity;
        }
        let mut pixel = entity.material().diffuse_color;
        for (i, subpixel) in pixel.iter_mut().enumerate() {
            *subpixel = (*subpixel as f64 * diffuse_light_intensity * entity.material().albedo[0]
                + 255.0 * specular_light_intensity * entity.material().albedo[1]
                + reflect_color[i] as f64 * entity.material().albedo[2]
                + refract_color[i] as f64 * entity.material().albedo[3])
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

fn refract(incident: &V3f, normal: &V3f, refractive_index: f64) -> V3f {
    let inc = incident.normalize();
    let mut n = normal.normalize();
    let mut cos_i = -inc * n;
    let mut eta_i = 1.0;
    let mut eta_t = refractive_index;

    if cos_i < 0.0 {
        cos_i = -cos_i;
        std::mem::swap(&mut eta_i, &mut eta_t);
        n = -n;
    }
    let eta = eta_i / eta_t;
    let k = 1.0 - (eta * eta * (1.0 - (cos_i * cos_i)));
    if k < 0.0 {
        V3f::with_value(0.0)
    } else {
        (inc * eta) + (n * (eta * cos_i - k.sqrt()))
    }
}
