use ray_tracing::{vec3, ray};

#[allow(clippy::assertions_on_constants)]
fn main() {
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMG_WIDTH: usize = 400;
    const IMG_HEIGHT: usize = (IMG_WIDTH as f64/ASPECT_RATIO) as usize;
    debug_assert!(IMG_HEIGHT >= 1, "Image height must be at least 1");
    eprintln!("Size: {} x {}", IMG_WIDTH, IMG_HEIGHT);


 //   let mut image_list = Vec::with_capacity(IMG_HEIGHT);

    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ((IMG_WIDTH as f64)/(IMG_HEIGHT as f64) );
    const CAMERA_CENTER: vec3::Point3 = vec3::Point3::zeroed();

    const VIEWPORT_U: vec3::Vec3 = vec3::Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    const VIEWPORT_V: vec3::Vec3 = vec3::Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    let pixel_delta_u: vec3::Vec3 = VIEWPORT_U / IMG_WIDTH as f64;
    let pixel_delta_v: vec3::Vec3 = VIEWPORT_V / IMG_HEIGHT as f64;

    let viewport_upper_left: vec3::Point3 = CAMERA_CENTER - vec3::Vec3::new(0.0, 0.0, FOCAL_LENGTH) - (VIEWPORT_U/2.0) - (VIEWPORT_V/2.0);

    let pixel_0_loc = viewport_upper_left + (0.5 *(&pixel_delta_u+&pixel_delta_v));


   // let img = ray_tracing::image::PPMImage::new(&mut image_list, IMG_WIDTH, IMG_HEIGHT, 255);
    println!("P3");
    println!("{} {}", IMG_WIDTH, IMG_HEIGHT);
    println!("255");
    for i in 0..IMG_HEIGHT {
        eprint!("\rRemaining lines: {} ", IMG_HEIGHT  - i);
      // img.color_codes.push(Vec::with_capacity(IMG_WIDTH));
        for j in 0..IMG_WIDTH {
            let pixel_center =  (i as f64 * &pixel_delta_v) + (j as f64 *&pixel_delta_u) + &pixel_0_loc;
            let ray_direction = pixel_center - CAMERA_CENTER;

            let ray = ray_tracing::ray::Ray::new(CAMERA_CENTER, ray_direction);
            let pixel_color = ray_color(&ray);/* vec3::Color::new(
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

fn ray_color(ray: &ray_tracing::ray::Ray) ->vec3::Color {
    let t = hit_sphere(&vec3::Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal = (ray.at(t) - vec3::Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5*(vec3::Color::new(normal.x()+1.0, normal.y()+1.0, normal.z()+1.0))
    }
    let unit_direction = ray.dir().unit_vector();
    let a = 0.5*(unit_direction.y() + 1.0); // Normalize values from -1 to 1 to 0 to 1

    lerp(a, vec3::Color::new(1.0, 1.0, 1.0), vec3::Color::new(0.5, 0.7, 1.0))
}

fn lerp(t: f64, start_value: vec3::Color, end_value: vec3::Color) -> vec3::Color {
    (1.0-t)*start_value + t*end_value
} 

fn hit_sphere(center: &vec3::Point3, radius: f64, ray: &ray::Ray) -> f64 {
    let oc = ray.origin() - center;

    let a = ray.dir().dot(ray.dir());
    let b = 2.0 * oc.dot(ray.dir());
    let c = oc.dot(&oc) - radius * radius;

    let discriminant = b*b - 4.0*a*c;

    if discriminant <0.0 {
        return -1.0;
    } 

    (-b - f64::sqrt(discriminant)) / 2.0*a
}
