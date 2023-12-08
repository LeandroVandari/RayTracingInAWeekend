pub mod image {
    pub struct PPMImage<'a>  {
        height: usize,
        width: usize,
        color_encoding: String,
        max_color: u32,

        pub color_codes: &'a mut Vec<Vec<super::vec3::Color>>,
    }

    impl<'a> PPMImage<'a> {
        pub fn new(pixels_list: &'a mut Vec<Vec<super::vec3::Color>>, width: usize, height: usize, max_color: u32) -> PPMImage {
            PPMImage {
                height,
                width,
                color_encoding: String::from("P3"),
                max_color,
                color_codes: pixels_list,
            }
        }

        pub fn print_image(&self) {
            println!("{}", self.color_encoding);
            println!("{} {}", self.width, self.height);
            println!("{}", self.max_color);

            for row in self.color_codes.iter() {
                for color in row {
                    let max = self.max_color as f64;

                    let (r, g, b) = ((color.x() * max) as usize, (color.y() * max) as usize, (color.z() * max) as usize);
                    println!("{} {} {}", r, g, b)
                }
            }
        }
    }

    impl<'a> From<PPMImage<'a>> for String {
        fn from(value: PPMImage) -> Self {
            let mut str = value.color_encoding.clone();

            let mut width_and_height_str = value.width.to_string();
            width_and_height_str.push(' ');
            width_and_height_str.push_str(value.height.to_string().as_str());

            str.push('\n');
            str.push_str(&width_and_height_str);
            str.push('\n');
            str.push_str(&value.max_color.to_string());
            str.push('\n');

            for row in value.color_codes {
                for color in row {
                    let max = value.max_color as f64;

                    let (r, g, b) = (color.x() * max, color.y() * max, color.z() * max);
                    str.push_str(&(r as u32).to_string());
                    str.push(' ');
                    str.push_str(&(g as u32).to_string());
                    str.push(' ');
                    str.push_str(&(b as u32).to_string());
                    str.push(' ');

                    // str.push('\n');
                }
                //str.push('\n');
            }

            str
        }
    }
}

pub mod vec3 {
    pub type Color = Vec3;
    pub type Point3 = Vec3;

    #[derive(Clone)]
    pub struct Vec3 {
        pub points: [f64; 3],
    }

    impl Vec3 {
        pub fn x(&self) -> f64 {
            self.points[0]
        }
        pub fn y(&self) -> f64 {
            self.points[1]
        }
        pub fn z(&self) -> f64 {
            self.points[2]
        }
        pub const fn zeroed() -> Self {
            Self {
                points: [0.0, 0.0, 0.0],
            }
        }
        pub const fn new(x: f64, y: f64, z: f64) -> Self {
            Self { points: [x, y, z] }
        }

        pub fn length_squared(&self) -> f64 {
            self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
        }
        pub fn length(&self) -> f64 {
            f64::sqrt(self.length_squared())
        }

        pub fn dot(&self, other: &Self) -> f64 {
            self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
        }
        pub fn cross(&self, other: &Self) -> Self {
            Self::new(
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            )
        }

        pub fn unit_vector(&self) -> Self {
            self / self.length()
        }

        pub fn write_color(&self) {
            println!(
                "{} {} {}",
                f64::floor(self.x() * 255.999),
                f64::floor(self.y() * 255.999),
                f64::floor(self.z() * 255.999)
            );
        }
    }

    impl std::ops::Neg for Vec3 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self::new(-self.x(), -self.y(), -self.z())
        }
    }
    impl std::ops::Index<usize> for Vec3 {
        type Output = f64;
        fn index(&self, index: usize) -> &Self::Output {
            &self.points[index]
        }
    }
    impl std::ops::AddAssign for Vec3 {
        fn add_assign(&mut self, rhs: Self) {
            self.points[0] += rhs.x();
            self.points[1] += rhs.y();
            self.points[2] += rhs.z();
        }
    }
    impl std::ops::MulAssign<f64> for Vec3 {
        fn mul_assign(&mut self, rhs: f64) {
            self.points[0] *= rhs;
            self.points[1] *= rhs;
            self.points[2] *= rhs;
        }
    }
    impl std::ops::DivAssign<f64> for Vec3 {
        fn div_assign(&mut self, rhs: f64) {
            *self *= 1.0 / rhs;
        }
    }
    impl std::ops::Add for Vec3 {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
        }
    }
    impl std::ops::Add<&Vec3> for Vec3 {
        type Output = Vec3;
        fn add(self, rhs: &Vec3) -> Self::Output {
            Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
        }
    }
    impl std::ops::Add<&Vec3> for &Vec3 {
        type Output = Vec3;
        fn add(self, rhs: &Vec3) -> Self::Output {
            Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
        }
    }
    impl std::ops::Sub for Vec3 {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
        }
    }
    impl std::ops::Sub<Vec3> for &Vec3 {
        type Output = Vec3;
        fn sub(self, rhs: Vec3) -> Self::Output {
            Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
        }
    }
    impl std::ops::Sub for &Vec3 {
        type Output = Vec3;
        fn sub(self, rhs:Self) -> Self::Output {
            Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
        }
    }
    impl std::ops::Mul for Vec3 {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
        }
    }
    impl std::ops::Mul<f64> for Vec3 {
        type Output = Self;
        fn mul(self, rhs: f64) -> Self::Output {
            Self::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
        }
    }
    impl std::ops::Mul<Vec3> for f64 {
        type Output = Vec3;
        fn mul(self, rhs: Vec3) -> Self::Output {
            rhs * self
        }
    }
    impl std::ops::Div<f64> for Vec3 {
        type Output = Self;
        fn div(self, rhs: f64) -> Self::Output {
            (1.0 / rhs) * self
        }
    }
    impl std::ops::Div<f64> for &Vec3 {
        type Output = Vec3;
        fn div(self, rhs: f64) -> Self::Output {
            (1.0 / rhs) * self
        }
    }
    impl std::ops::Mul<&Vec3> for f64 {
        type Output = Vec3;
        fn mul(self, rhs: &Vec3) -> Self::Output {
            rhs * self
        }
    }
    impl std::ops::Mul<f64> for &Vec3 {
        type Output = Vec3;
        fn mul(self, rhs: f64) -> Self::Output {
            Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
        }
    }

    impl std::fmt::Display for Vec3 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "{} {} {}", self.x(), self.y(), self.z())
        }
    }
}

pub  mod ray {
    pub struct Ray {
        origin: super::vec3::Point3,
        dir: super::vec3::Vec3
    }

    impl Ray {
        pub fn new (origin: super::vec3::Point3, dir: super::vec3::Vec3) -> Self {
            Self {origin, dir}
        }

        pub fn origin(&self) -> &super::vec3::Point3 {
            &self.origin
        }
        pub fn dir(&self) -> &super::vec3::Vec3 {
            &self.dir
        }

        pub fn at(&self, t:f64) -> super::vec3::Vec3 {
             t*&self.dir + &self.origin
        }
    }
}

pub mod hittable {
    use crate::{ray, vec3};


    pub struct HitRecord {
        pub point: super::vec3::Point3,
        pub normal: super::vec3::Vec3,
        pub t: f64,
        pub front_face: bool
    }

    impl HitRecord {
        fn set_face_normal(&mut self, ray: &ray::Ray, outward_normal: vec3::Vec3) {
            self.front_face = ray.dir().dot(&outward_normal) < 0.0;
            self.normal = if self.front_face { outward_normal} else {-outward_normal}
        }
    }

    pub trait Hittable {
        fn hit(&self, ray: crate::ray::Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool;
    }

    pub mod shapes {
        use super::Hittable;

        pub struct Sphere {
            center: crate::vec3::Point3,
            radius: f64
        }
        impl Hittable for Sphere {
            fn hit(&self, ray: crate::ray::Ray, t_min: f64, t_max: f64, hit_rec: &mut super::HitRecord) -> bool {
                let radius_to_center = ray.origin() - &self.center;
                let a = ray.dir().length_squared();
                let half_b = radius_to_center.dot(ray.dir());
                let c = radius_to_center.length_squared() -  self.radius * self.radius;

                let discriminant = half_b * half_b - a*c;
                if discriminant < 0.0 {return false;}
                let disc_sqrt = f64::sqrt(discriminant);

                let mut root = (-half_b - disc_sqrt) /a;
                
                if root <= t_min || root >= t_max {
                    root = (-half_b + disc_sqrt) / a;
                    if root <= t_min || root >= t_max {
                        return false;
                    }
                }

                hit_rec.t = root;
                hit_rec.point = ray.at(root);

                let outward_normal = (&hit_rec.point - &self.center) / self.radius;
                hit_rec.set_face_normal(&ray, outward_normal);

                true
            }
        }

    }
}