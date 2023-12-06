pub mod image {
    pub struct PPMImage {
        height: usize,
        width: usize,
        color_encoding: String,
        max_color: u32,

        color_codes: Vec<Vec<super::vec3::Color>>,
    }

    impl PPMImage {
        pub fn new(img_height: usize, img_width: usize, max_color: u32) -> PPMImage {
            PPMImage {
                height: img_height,
                width: img_width,
                color_encoding: String::from("P3"),
                max_color,
                color_codes: Self::create_filled_image_list(img_height, img_width),
            }
        }
        fn create_filled_image_list(
            img_height: usize,
            img_width: usize,
        ) -> Vec<Vec<super::vec3::Color>> {
            let mut list = Vec::with_capacity(img_height);
            for i in 0..img_height {
                eprint!("\rRemaining lines: {} ", img_height - i);
                list.push(Vec::with_capacity(img_width));
                for j in 0..img_width {
                    let pixel_color = super::vec3::Color::new(j as f64 / (img_width - 1) as f64, i as f64 / (img_height - 1) as f64, 0.0);
                    list[i].push(pixel_color);
                }
            }
            list
        }

        pub fn print_image(&self) {
            println!("{}\n", self.color_encoding);
            println!("{} {}\n", self.width, self.height);
            println!("{}\n", self.max_color);


            for row in self.color_codes.iter() {
                for color in row {
                    let max = self.max_color as f64;
                    unsafe {
                    let (r, g, b) = (color.x() * max, color.y() * max, color.z() * max);
                    
                    println!("{} {} {}", r, g, b)}
                }

            }
        }
    }



    impl From<PPMImage> for String {
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
                    unsafe {
                    let (r, g, b) = (color.x() * max, color.y() * max, color.z() * max);
                    str.push_str(&(r as u32).to_string());
                    str.push(' ');
                    str.push_str(&(g as u32).to_string());
                    str.push(' ');
                    str.push_str(&(b as u32).to_string());
                    str.push(' ');
                    }

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

        fn length_squared(&self) -> f64 {
            unsafe { self.x() * self.x() + self.y() * self.y() + self.z() * self.z() }
        }
        pub fn length(&self) -> f64 {
            f64::sqrt(self.length_squared())
        }

        pub fn dot(&self, other: &Self) -> f64 {
            unsafe {
                self.x()*other.x() + self.y()*other.y() + self.z()*other.z()
            }
        }
        pub fn cross(&self, other: &Self) -> Self {
            unsafe {
                Self::new(self.y()*other.z() - self.z()*other.y(), self.z()*other.x()-self.x()*other.z(), self.x()*other.y()-self.y()*other.x())
            }
        }

        pub fn unit_vector(&self) -> Self {
            self / self.length()
        }

        pub fn write_color(&self) {
            unsafe {
            println!("{} {} {}", f64::floor(self.x() * 255.999), f64::floor(self.y() * 255.999), f64::floor(self.z() * 255.999));}
        }
    }

    impl std::ops::Neg for Vec3 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            unsafe { Self::new(-self.x(), -self.y(), -self.z()) }
        }
    }
    impl std::ops::Index<usize> for Vec3 {
        type Output = f64;
        fn index(&self, index: usize) -> &Self::Output {
            unsafe { &self.points[index] }
        }
    }
    impl std::ops::AddAssign for Vec3 {
        fn add_assign(&mut self, rhs: Self) {
            unsafe {
                self.points[0] += rhs.x();
                self.points[1] += rhs.y();
                self.points[2] += rhs.z();
            }
        }
    }
    impl std::ops::MulAssign<f64> for Vec3 {
        fn mul_assign(&mut self, rhs: f64) {
            unsafe {
                self.points[0] *= rhs;
                self.points[1] *= rhs;
                self.points[2] *= rhs;
            }
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
            unsafe {
            Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z()+rhs.z())
            }
        }
    }
    impl std::ops::Sub for Vec3 {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            unsafe {
            Self::new(self.x() - rhs.x(), self.y()-rhs.y(), self.z()-rhs.z())
            }
        }
    }
    impl std::ops::Mul for Vec3 {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            unsafe {
            Self::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
            }
        }
    }
    impl std::ops::Mul<f64> for Vec3 {
        type Output = Self;
        fn mul(self, rhs: f64) -> Self::Output {
            unsafe {
            Self::new(self.x() * rhs, self.y()*rhs, self.z()*rhs)
            }
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
            (1.0/rhs) * self
        }
    }
    impl std::ops::Div<f64> for &Vec3 {
        type Output = Vec3;
        fn div(self, rhs: f64) -> Self::Output {
            (1.0/rhs) * self
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
            unsafe {
            Vec3::new(self.x() * rhs, self.y()*rhs, self.z()*rhs)
            }
        }
    }


    impl std::fmt::Display for Vec3 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            unsafe { writeln!(f, "{} {} {}", self.x(), self.y(), self.z()) }
        }
    }
}
