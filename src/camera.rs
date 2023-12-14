use rand::Rng;

use crate::{consts, hittable, lerp, ray, vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub img_width: usize,
    img_height: usize,
    center: vec3::Point3,
    pixel_0_loc: vec3::Point3,
    pixel_delta_u: vec3::Vec3,
    pixel_delta_v: vec3::Vec3,
    pub samples_per_pixel: u32,
    pub max_bounces: u32,
}

impl Camera {
    pub fn render(&mut self, world: &hittable::HittableObjects) {
        let samples_per_pixel = self.samples_per_pixel as f64;
        use std::io::Write;
        let mut write_buffer = std::io::BufWriter::new(std::io::stdout());
        self.initialize();
        println!("P3");
        println!("{} {}", self.img_width, self.img_height);
        println!("255");
        for i in 0..self.img_height {
            eprint!("\rRemaining lines: {} ", self.img_height - i);
            // img.color_codes.push(Vec::with_capacity(IMG_WIDTH));
            for j in 0..self.img_width {
                let mut pixel_color = vec3::Vec3::zeroed();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);

                    pixel_color += Self::ray_color(&ray, self.max_bounces, world);
                }
                pixel_color.write_color(&mut write_buffer, samples_per_pixel);
                //  img.color_codes[i].push(pixel_color);
            }
        }
        write_buffer.flush().unwrap();
    }

    fn initialize(&mut self) {
        self.img_height = (self.img_width as f64 / self.aspect_ratio) as usize;
        debug_assert!(self.img_height >= 1, "Image height must be at least 1");
        eprintln!("Size: {} x {}", self.img_width, self.img_height);

        const FOCAL_LENGTH: f64 = 1.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        let viewport_width: f64 =
            VIEWPORT_HEIGHT * ((self.img_width as f64) / (self.img_height as f64));

        let viewport_u: vec3::Vec3 = vec3::Vec3::new(viewport_width, 0.0, 0.0);
        const VIEWPORT_V: vec3::Vec3 = vec3::Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

        self.pixel_delta_u = &viewport_u / self.img_width as f64;
        self.pixel_delta_v = VIEWPORT_V / self.img_height as f64;

        let viewport_upper_left: vec3::Point3 = &self.center
            - vec3::Vec3::new(0.0, 0.0, FOCAL_LENGTH)
            - (viewport_u / 2.0)
            - (VIEWPORT_V / 2.0);

        self.pixel_0_loc =
            viewport_upper_left + (0.5 * (&self.pixel_delta_u + &self.pixel_delta_v));

        /*         let viewport_upper_left: vec3::Point3 = CAMERA_CENTER
        - vec3::Vec3::new(0.0, 0.0, FOCAL_LENGTH)
        - (viewport_u / 2.0)
        - (VIEWPORT_V / 2.0); */
        /*
        let pixel_0_loc = viewport_upper_left + (0.5 * (&pixel_delta_u + &pixel_delta_v)); */
    }

    fn ray_color(ray: &ray::Ray, depth: u32, world: &hittable::HittableObjects) -> vec3::Color {
        if depth == 0 {
            return vec3::Color::zeroed();
        }
        let mut hit_record = hittable::HitRecord::new();
        if world.hit(
            ray,
            consts::Interval::new(0.001, consts::INFINITY),
            &mut hit_record,
        ) {
            let direction = hit_record.normal + vec3::Vec3::random_unit_vector();
            return 0.5
                * Self::ray_color(
                    &ray::Ray::new(&hit_record.point, direction),
                    depth - 1,
                    world,
                );
        }
        let unit_direction = ray.dir().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0); // Normalize values from -1 to 1 to 0 to 1

        lerp(
            a,
            vec3::Color::new(1.0, 1.0, 1.0),
            vec3::Color::new(0.5, 0.7, 1.0),
        )
    }

    fn get_ray(&self, i: usize, j: usize) -> ray::Ray {
        let pixel_center =
            (i as f64 * &self.pixel_delta_v) + (j as f64 * &self.pixel_delta_u) + &self.pixel_0_loc;

        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_dir = &pixel_sample - &self.center;

        return ray::Ray::new(&self.center, ray_dir);
    }

    fn pixel_sample_square(&self) -> vec3::Vec3 {
        let point_x: f64 = -0.5 + rand::thread_rng().gen::<f64>();
        let point_y: f64 = -0.5 + rand::thread_rng().gen::<f64>();

        (point_x * &self.pixel_delta_u) + (point_y * &self.pixel_delta_v)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            img_width: 1600,
            aspect_ratio: 16.0 / 9.0,
            img_height: 900,
            center: vec3::Vec3::zeroed(),
            pixel_0_loc: vec3::Vec3::zeroed(),
            pixel_delta_u: vec3::Vec3::zeroed(),
            pixel_delta_v: vec3::Vec3::zeroed(),
            samples_per_pixel: 100,
            max_bounces: 10,
        }
    }
}
