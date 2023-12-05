fn main() {
    let img = PPMImage::new(256, 256, 255);
    println!("{}", String::from(img));
}

struct PPMImage {
    height: usize,
    width: usize,
    color_encoding: String,
    max_color: u32,

    color_codes: Vec<Vec<(f64, f64, f64)>>
}

impl PPMImage{
    pub fn new(img_height: usize, img_width: usize, max_color: u32) -> PPMImage{
        PPMImage { height: img_height, width: img_width, color_encoding: String::from("P3"), max_color, color_codes: Self::create_filled_image_list(img_height, img_width) }
    }
    fn create_filled_image_list(img_height: usize, img_width: usize) -> Vec<Vec<(f64, f64, f64)>> {
        let mut list = Vec::with_capacity(img_height);
        for i in 0..img_height {

            list.push(Vec::with_capacity(img_width));
            for j in 0..img_width {

                let r =  j as f64 / (img_width-1) as f64;
                let g = i as f64 / (img_height-1) as f64;
                let b = 0.0;
                list[i].push((r, g, b));
            }
        }
        list
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
                let (r, g, b) = (color.0*max, color.1*max, color.2*max);
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

