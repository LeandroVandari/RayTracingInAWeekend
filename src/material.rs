pub trait Material {
    fn scatter(&self, ray_in: &super::ray::Ray, hit_record: &mut super::hittable::HitRecord, attenuation: &mut super::vec3::Color, scattered: &mut super::ray::Ray) -> bool;
}