use ray_tracing::{vec3, hittable, consts};

#[allow(clippy::assertions_on_constants)]
fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: usize = 1600;
    const IMG_HEIGHT: usize = (IMG_WIDTH as f64 / ASPECT_RATIO) as usize;
    debug_assert!(IMG_HEIGHT >= 1, "Image height must be at least 1");
    eprintln!("Size: {} x {}", IMG_WIDTH, IMG_HEIGHT);

    let mut world = hittable::HittableObjects::new();

    world.add_hittable(std::rc::Rc::new(hittable::shapes::Sphere::new(vec3::Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add_hittable(std::rc::Rc::new(hittable::shapes::Sphere::new(vec3::Point3::new(0.0, -100.5, -1.0), 100.0)));
    //   let mut image_list = Vec::with_capacity(IMG_HEIGHT);

    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ((IMG_WIDTH as f64) / (IMG_HEIGHT as f64));
    const CAMERA_CENTER: vec3::Point3 = vec3::Point3::zeroed();

    const VIEWPORT_U: vec3::Vec3 = vec3::Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    const VIEWPORT_V: vec3::Vec3 = vec3::Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    let pixel_delta_u: vec3::Vec3 = VIEWPORT_U / IMG_WIDTH as f64;
    let pixel_delta_v: vec3::Vec3 = VIEWPORT_V / IMG_HEIGHT as f64;

    let viewport_upper_left: vec3::Point3 = CAMERA_CENTER
        - vec3::Vec3::new(0.0, 0.0, FOCAL_LENGTH)
        - (VIEWPORT_U / 2.0)
        - (VIEWPORT_V / 2.0);

    let pixel_0_loc = viewport_upper_left + (0.5 * (&pixel_delta_u + &pixel_delta_v));

    // let img = ray_tracing::image::PPMImage::new(&mut image_list, IMG_WIDTH, IMG_HEIGHT, 255);
    println!("P3");
    println!("{} {}", IMG_WIDTH, IMG_HEIGHT);
    println!("255");
    for i in 0..IMG_HEIGHT {
        eprint!("\rRemaining lines: {} ", IMG_HEIGHT - i);
        // img.color_codes.push(Vec::with_capacity(IMG_WIDTH));
        for j in 0..IMG_WIDTH {
            let pixel_center =
                (i as f64 * &pixel_delta_v) + (j as f64 * &pixel_delta_u) + &pixel_0_loc;
            let ray_direction = pixel_center - CAMERA_CENTER;

            let ray = ray_tracing::ray::Ray::new(CAMERA_CENTER, ray_direction);
            let pixel_color = ray_color(&ray, &world); /* vec3::Color::new(
                                                   j as f64 / (IMG_WIDTH - 1) as f64,
                                                   i as f64 / (IMG_HEIGHT - 1) as f64,
                                                   0.0,
                                               ); */
            pixel_color.write_color();
            //  img.color_codes[i].push(pixel_color);
        }
    }

    // img.print_image()
}

fn ray_color(ray: &ray_tracing::ray::Ray, world: &hittable::HittableObjects) -> vec3::Color {
    let mut hit_record = hittable::HitRecord::new();
    if world.hit(ray, consts::Interval::new(0.0, consts::INFINITY), &mut hit_record) {
        return 0.5 * (hit_record.normal + vec3::Color::new(1.0, 1.0, 1.0)); // Normalize values
    }
    let unit_direction = ray.dir().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0); // Normalize values from -1 to 1 to 0 to 1

    lerp(
        a,
        vec3::Color::new(1.0, 1.0, 1.0),
        vec3::Color::new(0.5, 0.7, 1.0),
    )
}

fn lerp(t: f64, start_value: vec3::Color, end_value: vec3::Color) -> vec3::Color {
    (1.0 - t) * start_value + t * end_value
}

