use ray_tracing::image;
fn main() {
    let img = image::PPMImage::new(256, 256, 255);
    img.print_image()
}
