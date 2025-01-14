use hittable::shapes::Sphere;
use ray_tracing::material::{Dieletric, Lambertian, Metal};
use ray_tracing::{hittable, vec3};
use std::rc::Rc;
use vec3::Point3;

#[allow(clippy::assertions_on_constants)]
fn main() {
    let mut world = hittable::HittableObjects::new();

    let material_ground = Rc::new(Lambertian::new(vec3::Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(vec3::Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dieletric::new(1.5));
    let material_right = Rc::new(Metal::new(vec3::Color::new(0.8, 0.6, 0.2), 0.0));

    world.add_hittable(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add_hittable(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add_hittable(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add_hittable(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    )));
    world.add_hittable(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut camera = ray_tracing::camera::Camera::default();

    camera.img_width = 1280;
    camera.aspect_ratio = 16.0 / 9.0;
    camera.samples_per_pixel = 64;
    camera.max_bounces = 32;

    camera.render(&world);
}
