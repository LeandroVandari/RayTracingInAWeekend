pub struct PPMImage<'a> {
    height: usize,
    width: usize,
    color_encoding: String,
    max_color: u32,

    pub color_codes: &'a mut Vec<Vec<super::vec3::Color>>,
}

impl<'a> PPMImage<'a> {
    pub fn new(
        pixels_list: &'a mut Vec<Vec<super::vec3::Color>>,
        width: usize,
        height: usize,
        max_color: u32,
    ) -> PPMImage {
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

                let (r, g, b) = (
                    (color.x() * max) as usize,
                    (color.y() * max) as usize,
                    (color.z() * max) as usize,
                );
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
