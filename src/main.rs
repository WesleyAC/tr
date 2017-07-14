extern crate image;
extern crate nalgebra;

use image::RgbImage;
use nalgebra::{Vector3, norm_squared};

struct Sphere {
    origin: Vector3<f64>,
    diameter: f64
}

impl Sphere {
    fn intersect(self, ray: Ray) -> bool {
        let l = &ray.origin - &self.origin;
        let tca = l.dot(&ray.direction);
        let d2 = norm_squared(&l) - (tca * tca);
        d2 < self.diameter
    }
}

struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>
}

fn trace(ray: Ray) -> [u8; 3] {
    let thing = Sphere { origin: Vector3::new(0.0, 0.0, -5.0), diameter: 0.3 };
    if thing.intersect(ray) {
        [255, 255, 255]
    } else {
        [0, 0, 0]
    }
}

/// x is scaled from -0.5 to 0.5, y is scaled by same factor
fn pixel_color(x: f64, y: f64) -> [u8; 3] {
    let ray = Ray { origin: Vector3::new(0.0, 0.0, 0.0), direction: Vector3::new(x, y, 2.5).normalize() };

    trace(ray)
}

fn render_image(width: u32, height: u32) {
    let mut image = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let xs = (x as f64 / width as f64) - 0.5;
            let ys = (y as f64 / width as f64) - ((0.5 * height as f64) / width as f64);
            image.get_pixel_mut(x, y).data = pixel_color(xs, ys);
        }
    }
    image.save("/tmp/out.png").unwrap();
}

fn main() {
    render_image(400, 300);
}
