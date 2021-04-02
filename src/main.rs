use image::ImageBuffer;

mod vector3d;
fn main() {
    render();
}

fn render() {
    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 768;
    let mut imgbuf = ImageBuffer::new(WIDTH, HEIGHT);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (255.0 * (y as f64) / (HEIGHT as f64)) as u8;
        let g = (255.0 * (x as f64) / (WIDTH as f64)) as u8;
        *pixel = image::Rgb([r, g, 0u8]);
    }

    imgbuf.save("renders/out.png").unwrap();
}

struct Sphere {
    center: (),
}
