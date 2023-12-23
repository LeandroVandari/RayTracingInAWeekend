
pub trait Material {
    fn scatter<'a>(&self, _ray_in: &crate::ray::Ray, hit_record: &'a mut crate::hittable::HitRecord, attenuation: &mut crate::vec3::Color, scattered: &mut crate::ray::Ray<'a>, rng_gen: &mut rand::rngs::SmallRng) -> bool;
}

pub struct Metal { albedo: super::vec3::Color, fuzziness: f64}

impl Metal {
    pub const fn new(albedo: super::vec3::Color, fuzziness: f64) -> Self {
        Self{albedo, fuzziness}
    }
}

impl Material for Metal {
    fn scatter<'a>(&self, ray_in: &crate::ray::Ray, hit_record: &'a mut crate::hittable::HitRecord, attenuation: &mut crate::vec3::Color, scattered: &mut crate::ray::Ray<'a>, rng_gen: &mut rand::rngs::SmallRng) -> bool {
        let reflected = super::vec3::Vec3::reflect(&ray_in.dir().unit_vector(), &hit_record.normal);
        scattered.set_origin(&hit_record.point);
        scattered.set_dir(reflected + self.fuzziness*crate::vec3::Vec3::random_unit_vector(rng_gen));

        *attenuation = self.albedo.clone();

        scattered.dir().dot(&hit_record.normal) > 0.0
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
    fn scatter<'a>(&self, _ray_in: &crate::ray::Ray, hit_record: &'a mut crate::hittable::HitRecord, attenuation: &mut crate::vec3::Color, scattered: &mut crate::ray::Ray<'a>, rng_gen: &mut rand::rngs::SmallRng) -> bool {
        let mut scatter_direction = &hit_record.normal + &super::vec3::Vec3::random_unit_vector(rng_gen);
        if scatter_direction.near_zero() {
            scatter_direction=hit_record.normal.clone();
        }
       // let p_ref = unsafe { &*(&hit_record.point as *const crate::vec3::Vec3)};
        scattered.set_origin(&hit_record.point);
        scattered.set_dir(scatter_direction);

        *attenuation = self.albedo.clone();

        true
    }
}