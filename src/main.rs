use ray_tracing::image;
fn main() {
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMG_WIDTH: u32 = 400;
    const IMG_HEIGHT: u32 = u32::from(f64::floor(IMG_WIDTH as f64/ASPECT_RATIO));

    let img = image::PPMImage::new(256, IMG_WIDTH, 255);
    img.print_image()
}
