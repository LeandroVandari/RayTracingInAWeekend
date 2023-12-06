use ray_tracing::image;
fn main() {
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMG_WIDTH: usize = 800;
    const IMG_HEIGHT: usize = (IMG_WIDTH as f64/ASPECT_RATIO) as usize;
    debug_assert!(IMG_HEIGHT >= 1, "Image height must be at least 1");

    const VEC_ZERO: ray_tracing::vec3::Color = ray_tracing::vec3::Color::zeroed();
    const VEC_ZERO_ARRAY: [ray_tracing::vec3::Color;IMG_WIDTH] = [VEC_ZERO;IMG_WIDTH];

    let mut image_list = [VEC_ZERO_ARRAY;IMG_HEIGHT];


/*     const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ((IMG_WIDTH as f64)/(IMG_HEIGHT as f64) ); */
    let img = image::PPMImage::new(&mut image_list, 255);

    for i in 0..IMG_HEIGHT {
        eprint!("\rRemaining lines: {} ", IMG_HEIGHT  - i);
        for j in 0..IMG_WIDTH {
            let pixel_color = ray_tracing::vec3::Color::new(
                j as f64 / (IMG_WIDTH - 1) as f64,
                i as f64 / (IMG_HEIGHT - 1) as f64,
                0.0,
            );
            img.color_codes[i][j] = pixel_color;
        }
    }

    img.print_image()
}
