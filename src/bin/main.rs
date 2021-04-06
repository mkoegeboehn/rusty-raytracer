use rusty_raytracer::entity::Sphere;
use rusty_raytracer::light::Light;
use rusty_raytracer::material::Material;
use rusty_raytracer::render;
use rusty_raytracer::V3f;

fn main() {
    let entities = vec![
        Sphere::new(V3f::new(-3.0, 0.0, -16.0), 2.0, Material::IVORY).into(),
        Sphere::new(V3f::new(-1.0, -1.5, -12.0), 2.0, Material::GLASS).into(),
        Sphere::new(V3f::new(1.5, -0.5, -18.0), 3.0, Material::RED_RUBBER).into(),
        Sphere::new(V3f::new(7.0, 5.0, -18.0), 4.0, Material::MIRROR).into(),
    ];

    let lights = vec![
        Light::new(V3f::new(-20.0, 20.0, 20.0), 1.5),
        Light::new(V3f::new(30.0, 50.0, -25.0), 1.8),
        Light::new(V3f::new(30.0, 20.0, 30.0), 1.7),
    ];
    render(&entities, &lights);
}
