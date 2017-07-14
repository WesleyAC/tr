extern crate image;
extern crate nalgebra;

mod scene;
mod object;
mod ray;

use image::RgbImage;
use nalgebra::Vector3;

use ray::Ray;
use scene::Scene;
use object::Sphere;

fn trace(ray: Ray, scene: &Scene) -> [u8; 3] {
    for object in &scene.objects {
        if object.intersect(ray) {
            return [255, 255, 255];
        }
    }
    [0, 0, 0]
}

/// x is scaled from -0.5 to 0.5, y is scaled by same factor
fn pixel_color(x: f64, y: f64, scene: &Scene) -> [u8; 3] {
    let ray = Ray { origin: Vector3::new(0.0, 0.0, 0.0), direction: Vector3::new(x, y, 1.0).normalize() };

    trace(ray, scene)
}

fn render_image(width: u32, height: u32, scene: Scene) {
    let mut image = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let xs = (x as f64 / width as f64) - 0.5;
            let ys = (y as f64 / width as f64) - ((0.5 * height as f64) / width as f64);
            image.get_pixel_mut(x, y).data = pixel_color(xs, ys, &scene);
        }
    }
    image.save("/tmp/out.png").unwrap();
}

fn main() {
    let thing1 = Sphere { origin: Vector3::new(0.0, 0.0, -5.0), diameter: 0.3 };
    let thing2 = Sphere { origin: Vector3::new(-0.4, 0.2, -3.0), diameter: 0.3 };
    let scene = Scene { objects: vec![&thing1, &thing2] };
    render_image(400, 300, scene);
}
