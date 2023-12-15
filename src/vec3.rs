use rand::Rng;

use crate::consts;

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

    pub fn random() -> Self {
        let mut rand_gen = rand::thread_rng();
        Self::new(rand_gen.gen(), rand_gen.gen(), rand_gen.gen())
    }
    pub fn random_between(distribution: &once_cell::sync::Lazy<rand::distributions::Uniform<f64>>) -> Self {
        use rand::distributions::Distribution;
        let mut rand_gen = rand::thread_rng();

        Self::new(
            distribution.sample(&mut rand_gen),
            distribution.sample(&mut rand_gen),
            distribution.sample(&mut rand_gen),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let point = Self::random_between(&consts::MINUS_ONE_TO_ONE);
            if point.length_squared() < 1.0 {
                //If the point is in the unit sphere
                break point;
            }
        }
    }
    // Random vector on unit sphere's surface
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let on_sphere = Self::random_unit_vector();

        if normal.dot(&on_sphere) > 0.0 {
            // Same hemisphere
            on_sphere
        } else {
            -on_sphere
        }
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

    pub fn write_color(
        &self,
        buffer: &mut std::io::BufWriter<std::io::Stdout>,
        samples_per_pixel: f64,
    ) {
        use super::linear_space_to_gamma_space;

        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        let scale = 1.0 / samples_per_pixel;
        r *= scale;
        g *= scale;
        b *= scale;

        r = linear_space_to_gamma_space(r);
        g = linear_space_to_gamma_space(g);
        b = linear_space_to_gamma_space(b);

        let intensity = consts::Interval::new(0.0, 0.999);
        use std::io::Write;
        write!(
            buffer,
            "{} {} {} ",
            f64::floor(intensity.clamp(r) * 255.999),
            f64::floor(intensity.clamp(g) * 255.999),
            f64::floor(intensity.clamp(b) * 255.999)
        )
        .unwrap();
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
    fn sub(self, rhs: Self) -> Self::Output {
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
