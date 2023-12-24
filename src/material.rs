use rand::Rng;

pub trait Material {
    fn scatter<'a>(
        &self,
        _ray_in: &crate::ray::Ray,
        hit_record: &'a mut crate::hittable::HitRecord,
        attenuation: &mut crate::vec3::Color,
        scattered: &mut crate::ray::Ray<'a>,
        rng_gen: &mut rand::rngs::SmallRng,
    ) -> bool;
}

pub struct Metal {
    albedo: super::vec3::Color,
    fuzziness: f64,
}

impl Metal {
    pub const fn new(albedo: super::vec3::Color, fuzziness: f64) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter<'a>(
        &self,
        ray_in: &crate::ray::Ray,
        hit_record: &'a mut crate::hittable::HitRecord,
        attenuation: &mut crate::vec3::Color,
        scattered: &mut crate::ray::Ray<'a>,
        rng_gen: &mut rand::rngs::SmallRng,
    ) -> bool {
        let reflected = super::vec3::Vec3::reflect(&ray_in.dir().unit_vector(), &hit_record.normal);
        scattered.set_origin(&hit_record.point);
        scattered
            .set_dir(reflected + self.fuzziness * crate::vec3::Vec3::random_unit_vector(rng_gen));

        *attenuation = self.albedo.clone();

        scattered.dir().dot(&hit_record.normal) > 0.0
    }
}

pub struct Lambertian {
    albedo: super::vec3::Color,
}

impl Lambertian {
    pub const fn new(albedo: super::vec3::Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter<'a>(
        &self,
        _ray_in: &crate::ray::Ray,
        hit_record: &'a mut crate::hittable::HitRecord,
        attenuation: &mut crate::vec3::Color,
        scattered: &mut crate::ray::Ray<'a>,
        rng_gen: &mut rand::rngs::SmallRng,
    ) -> bool {
        let mut scatter_direction =
            &hit_record.normal + &super::vec3::Vec3::random_unit_vector(rng_gen);
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal.clone();
        }
        // let p_ref = unsafe { &*(&hit_record.point as *const crate::vec3::Vec3)};
        scattered.set_origin(&hit_record.point);
        scattered.set_dir(scatter_direction);

        *attenuation = self.albedo.clone();

        true
    }
}

pub struct Dieletric {
    index_of_refraction: f64,
}

impl Dieletric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }

    pub fn reflectance(cosine: f64, index_of_refraction: f64) -> f64 {
        let mut r0 = (1.0-index_of_refraction)/(1.0+index_of_refraction);
        r0 *= r0;
        r0 + (1.0-r0)*f64::powi(1.0-cosine, 5)
    }
}

impl Material for Dieletric {
    fn scatter<'a>(
        &self,
        ray_in: &crate::ray::Ray,
        hit_record: &'a mut crate::hittable::HitRecord,
        attenuation: &mut crate::vec3::Color,
        scattered: &mut crate::ray::Ray<'a>,
        rng_gen: &mut rand::rngs::SmallRng,
    ) -> bool {
        *attenuation = super::vec3::Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray_in.dir().unit_vector();
        let cos_theta = f64::min((-&unit_direction).dot(&hit_record.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rng_gen.gen::<f64>(){
            super::vec3::Vec3::reflect(&unit_direction, &hit_record.normal)
        } else {
            super::vec3::Vec3::refract(&unit_direction, &hit_record.normal, refraction_ratio)
        };

        scattered.set_origin(&hit_record.point);
        scattered.set_dir(direction);

        true
    }
}
