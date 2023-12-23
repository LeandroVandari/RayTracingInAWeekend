pub mod camera;
pub mod consts;
pub mod image;
pub mod material;

pub mod vec3;
pub mod ray {
    pub struct Ray<'a> {
        origin: &'a super::vec3::Point3,
        dir: super::vec3::Vec3,
    }

    impl<'a> Ray<'a> {
        pub fn new(origin: &'a super::vec3::Point3, dir: super::vec3::Vec3) -> Self {
            Self { origin, dir }
        }

        pub fn origin(&self) -> &super::vec3::Point3 {
            self.origin
        }
        pub fn dir(&self) -> &super::vec3::Vec3 {
            &self.dir
        }

        pub fn at(&self, t: f64) -> super::vec3::Vec3 {
            t * &self.dir + self.origin
        }

        pub(crate) fn set_origin(&mut self, origin: &'a super::vec3::Point3) {
            self.origin = origin;
        }
        pub(crate) fn set_dir(&mut self, dir: super::vec3::Vec3) {
            self.dir = dir;
        }
    }
}

pub mod hittable {
    use crate::{consts, ray, vec3};
    use std::rc::Rc;

    pub struct HitRecord {
        pub point: super::vec3::Point3,
        pub normal: super::vec3::Vec3,
        pub t: f64,
        pub front_face: bool,
        pub material: Option<Rc<dyn super::material::Material>>,
    }

    impl HitRecord {
        fn set_face_normal(&mut self, ray: &ray::Ray, outward_normal: vec3::Vec3) {
            self.front_face = ray.dir().dot(&outward_normal) < 0.0;
            self.normal = if self.front_face {
                outward_normal
            } else {
                -outward_normal
            }
        }
        pub fn new() -> Self {
            Self {
                point: vec3::Point3::zeroed(),
                normal: vec3::Vec3::zeroed(),
                t: f64::MAX,
                front_face: true,
                material: None
            }
        }

        fn set_params_equal_to(&mut self, other_hit_rec: &HitRecord) {
            self.point = other_hit_rec.point.clone();
            self.normal = other_hit_rec.normal.clone();
            self.t = other_hit_rec.t;
            self.front_face = other_hit_rec.front_face;
        }
    }


impl  Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}

    pub trait Hittable {
        fn hit(
            &self,
            ray: &crate::ray::Ray,
            t_interval: consts::Interval,
            hit_rec: &mut HitRecord,
        ) -> bool;
    }

    pub struct HittableObjects {
        hittables_vec: Vec<Rc<dyn Hittable>>,
    }

    impl HittableObjects {
        pub fn new() -> Self {
            Self {
                hittables_vec: Vec::new(),
            }
        }

        pub fn clear(&mut self) {
            self.hittables_vec.clear();
        }

        pub fn add_hittable(&mut self, hittable: Rc<dyn Hittable>) {
            self.hittables_vec.push(hittable);
        }

        pub fn hit(
            &self,
            ray: &ray::Ray,
            t_interval: consts::Interval,
            hit_rec: &mut HitRecord,
        ) -> bool {
            let mut temp_rec = HitRecord::new();
            let mut hit_anything = false;
            let mut closest_so_far = t_interval.max;

            for object in &self.hittables_vec {
                if object.hit(
                    ray,
                    consts::Interval::new(t_interval.min, closest_so_far),
                    &mut temp_rec,
                ) {
                    hit_anything = true;
                    closest_so_far = temp_rec.t;
                    hit_rec.set_params_equal_to(&temp_rec);
                    hit_rec.material = temp_rec.material.clone();
                }
            }

            hit_anything
        }
    }

    impl Default for HittableObjects {
        fn default() -> Self {
            Self::new()
        }
    }

    pub mod shapes {
        use crate::consts;

        use super::Hittable;

        pub struct Sphere {
            center: crate::vec3::Point3,
            radius: f64,
            material: std::rc::Rc<dyn crate::material::Material>
        }

        impl Sphere {
            pub fn new(center: crate::vec3::Point3, radius: f64, material: std::rc::Rc<dyn crate::material::Material>) -> Self {
                Self { center, radius, material }
            }
        }
        impl Hittable for Sphere {
            fn hit(
                &self,
                ray: &crate::ray::Ray,
                t_interval: consts::Interval,
                hit_rec: &mut super::HitRecord,
            ) -> bool {
                let radius_to_center = ray.origin() - &self.center;
                let a = ray.dir().length_squared();
                let half_b = radius_to_center.dot(ray.dir());
                let c = radius_to_center.length_squared() - self.radius * self.radius;

                let discriminant = half_b * half_b - a * c;
                if discriminant < 0.0 {
                    return false;
                }
                let disc_sqrt = f64::sqrt(discriminant);

                let mut root = (-half_b - disc_sqrt) / a;

                if !t_interval.surrounds(root) {
                    root = (-half_b + disc_sqrt) / a;
                    if !t_interval.surrounds(root) {
                        return false;
                    }
                }

                hit_rec.t = root;
                hit_rec.point = ray.at(root);

                let outward_normal = (&hit_rec.point - &self.center) / self.radius;
                hit_rec.set_face_normal(ray, outward_normal);
                hit_rec.material = Some(self.material.clone());

                true
            }
        }
    }
}

pub fn lerp(t: f64, start_value: vec3::Color, end_value: vec3::Color) -> vec3::Color {
    (1.0 - t) * start_value + t * end_value
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * consts::PI / 180.0
}

pub fn linear_space_to_gamma_space(linear_component: f64) -> f64 {
    f64::sqrt(linear_component)
}
