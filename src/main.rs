extern crate image;
extern crate nalgebra;

mod scene;
mod object;
mod ray;
mod color;

use image::RgbImage;
use nalgebra::Vector3;

use ray::Ray;
use scene::Scene;
use object::Sphere;
use color::Color;

fn trace(ray: Ray, scene: &Scene) -> Color {
    let mut min_dist = std::f64::INFINITY;
    let mut color = Color::new(0.0, 0.0, 0.0);
    for object in &scene.objects {
        let dist = object.intersect(ray);
        if dist.unwrap_or(std::f64::INFINITY) < min_dist {
            let object_color = dist.unwrap() / 2.0;
            color.set_color(object_color, object_color, object_color);
            min_dist = dist.unwrap();
        }
    }
    color
}

/// x is scaled from -0.5 to 0.5, y is scaled by same factor
fn pixel_color(x: f64, y: f64, scene: &Scene) -> Color {
    let ray = Ray { origin: Vector3::new(0.0, 0.0, 0.0), direction: Vector3::new(x, y, 1.0).normalize() };

    trace(ray, scene)
}

fn render_image(width: u32, height: u32, scene: Scene) {
    let mut image = RgbImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let xs = (x as f64 / width as f64) - 0.5;
            let ys = (y as f64 / width as f64) - ((0.5 * height as f64) / width as f64);
            image.get_pixel_mut(x, y).data = pixel_color(xs, ys, &scene).to_u8();
        }
    }
    image.save("/tmp/out.png").unwrap();
}

fn main() {
    let thing1 = Sphere { origin: Vector3::new(0.0, 0.0, 3.0), diameter: 1.0 };
    let thing2 = Sphere { origin: Vector3::new(1.7, 0.5, 5.0), diameter: 1.0 };
    let scene = Scene { objects: vec![&thing1, &thing2] };
    render_image(400, 300, scene);
}
