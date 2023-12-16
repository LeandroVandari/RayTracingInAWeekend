use ray_tracing::{hittable, vec3};

#[allow(clippy::assertions_on_constants)]
fn main() {
    let blue_lambertian = ray_tracing::material::Lambertian::new(vec3::Color::new(0.0, 0.0,1.0));
    let bl_rc = std::rc::Rc::new(blue_lambertian);
    let sphere_one = hittable::shapes::Sphere::new(vec3::Point3::new(0.0, 0.0, -1.0), 0.5, bl_rc.clone());
    let sphere_two = hittable::shapes::Sphere::new(
        vec3::Point3::new(0.0, -100.5, -1.0),
        100.0,
        bl_rc
    );
    let mut world = hittable::HittableObjects::new();

    world.add_hittable(std::rc::Rc::new(sphere_one));
    world.add_hittable(std::rc::Rc::new(sphere_two));

    let mut camera = ray_tracing::camera::Camera::default();

    camera.img_width = 400;
    camera.aspect_ratio = 16.0 / 9.0;
    camera.samples_per_pixel = 100;
    camera.max_bounces = 100;

    camera.render(&world);
}
