use crate::{hittable, ray, vec3, consts, lerp};

pub struct Camera{
    pub aspect_ratio: f64,
    pub img_width: usize,
    img_height: usize,
    center: vec3::Point3,
    pixel_0_loc: vec3::Point3,
    pixel_delta_u: vec3::Vec3,
    pixel_delta_v: vec3::Vec3,


}

impl Camera{

    pub fn render(&mut self, world: &hittable::HittableObjects) {
        self.initialize();
        println!("P3");
    println!("{} {}", self.img_width, self.img_height);
    println!("255");
    for i in 0..self.img_height {
        eprint!("\rRemaining lines: {} ", self.img_height - i);
        // img.color_codes.push(Vec::with_capacity(IMG_WIDTH));
        for j in 0..self.img_width {
            let pixel_center =
                (i as f64 * &self.pixel_delta_v) + (j as f64 * &self.pixel_delta_u) + &self.pixel_0_loc;
            let ray_direction = &pixel_center - &self.center;

            let ray = ray::Ray::new(&self.center, &ray_direction);
            let pixel_color = Self::ray_color(&ray, world); /* vec3::Color::new(
                                                   j as f64 / (self.img_width - 1) as f64,
                                                   i as f64 / (self.img_height - 1) as f64,
                                                   0.0,
                                               ); */
            pixel_color.write_color();
            //  img.color_codes[i].push(pixel_color);
        }
    }

    }

    fn initialize(&mut self) {
        self.img_height = (self.img_width as f64/self.aspect_ratio) as usize;
        debug_assert!(self.img_height >= 1, "Image height must be at least 1");
        eprintln!("Size: {} x {}", self.img_width, self.img_height);

        const FOCAL_LENGTH: f64 = 1.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        let viewport_width: f64 = VIEWPORT_HEIGHT * ((self.img_width as f64) / (self.img_height as f64));

        let viewport_u: vec3::Vec3 = vec3::Vec3::new(viewport_width, 0.0, 0.0);
        const VIEWPORT_V: vec3::Vec3 = vec3::Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

        self.pixel_delta_u =&viewport_u / self.img_width as f64;
        self.pixel_delta_v= VIEWPORT_V / self.img_height as f64;

        let viewport_upper_left: vec3::Point3 = &self.center
            - vec3::Vec3::new(0.0, 0.0, FOCAL_LENGTH)
        - (viewport_u / 2.0)
        - (VIEWPORT_V / 2.0);

        self.pixel_0_loc = viewport_upper_left + (0.5 * (&self.pixel_delta_u + &self.pixel_delta_v));

/*         let viewport_upper_left: vec3::Point3 = CAMERA_CENTER
            - vec3::Vec3::new(0.0, 0.0, FOCAL_LENGTH)
            - (viewport_u / 2.0)
            - (VIEWPORT_V / 2.0); */
/* 
        let pixel_0_loc = viewport_upper_left + (0.5 * (&pixel_delta_u + &pixel_delta_v)); */
    }

    fn ray_color(ray: &ray::Ray, world: &hittable::HittableObjects) -> vec3::Color {
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
}

impl Default for Camera {
    fn default() -> Self {
        Self {img_width: 1600, aspect_ratio: 16.0/9.0, img_height: 900, center: vec3::Vec3::zeroed(), pixel_0_loc: vec3::Vec3::zeroed(), pixel_delta_u: vec3::Vec3::zeroed(), pixel_delta_v: vec3::Vec3::zeroed()}
    }
}