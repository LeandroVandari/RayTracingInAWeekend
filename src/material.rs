pub trait Material {
    fn scatter(&self, ray_in: &super::ray::Ray, hit_record: & mut super::hittable::HitRecord, attenuation: &mut super::vec3::Color, scattered: &mut super::ray::Ray) -> bool;
}

pub struct Metal { }

impl Metal {
    pub const fn new() -> Self {
        Self{}
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &crate::ray::Ray, hit_record: & mut crate::hittable::HitRecord, attenuation: &mut crate::vec3::Color, scattered: & mut crate::ray::Ray) -> bool {
        true
    }
}

pub struct Lambertian {
    albedo: super::vec3::Color
}

impl Lambertian {
    pub const fn new(albedo: super::vec3::Color) -> Self {
        Self { albedo }
    
    }
}

impl Material for Lambertian {
    fn scatter<'a>(&self, _ray_in: &crate::ray::Ray, hit_record: &mut crate::hittable::HitRecord, attenuation: &mut crate::vec3::Color, scattered: &mut crate::ray::Ray) -> bool {
        let scatter_direction = &hit_record.normal + &super::vec3::Vec3::random_unit_vector();
        let p_ref = unsafe { &*(&hit_record.point as *const crate::vec3::Vec3)};
        scattered.set_origin(p_ref);
        scattered.set_dir(scatter_direction);

        *attenuation = self.albedo.clone();

        true
    }
}