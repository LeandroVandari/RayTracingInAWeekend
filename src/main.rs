use ray_tracing::{hittable, vec3};

#[allow(clippy::assertions_on_constants)]
fn main() {
    let mut world = hittable::HittableObjects::new();

    world.add_hittable(std::rc::Rc::new(hittable::shapes::Sphere::new(
        vec3::Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add_hittable(std::rc::Rc::new(hittable::shapes::Sphere::new(
        vec3::Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let mut camera = ray_tracing::camera::Camera::default();

    camera.img_width = 1600;
    camera.aspect_ratio = 16.0 / 9.0;
    camera.samples_per_pixel = 100;

    camera.render(&world);
}
