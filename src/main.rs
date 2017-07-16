extern crate image;
extern crate nalgebra;
extern crate rand;

mod scene;
mod object;
mod ray;
mod color;

use image::RgbImage;
use nalgebra::Vector3;
use rand::{thread_rng, Rng};

use ray::Ray;
use scene::Scene;
use object::Sphere;
use color::Color;

fn get_random_ray(normal: Vector3<f64>) -> Vector3<f64> {
    let u = rand::thread_rng().gen::<f64>();
    let v = rand::thread_rng().gen::<f64>();
    let theta = 2.0 * std::f64::consts::PI * u;
    let phi = (2.0 * v - 1.0).acos();
    let x = theta.sin() * phi.cos();
    let y = theta.sin() * theta.cos();
    let z = theta.cos();
    let rand_vec = Vector3::new(x, y, z).normalize();
    (normal + rand_vec).normalize()
}

fn trace(ray: Ray, scene: &Scene, depth: usize) -> Color {
    let mut min_dist = std::f64::INFINITY;
    let mut color = Color::new(0.25, 0.45, 1.0);
    if depth == 0 {
        return color;
    }
    for object in &scene.objects {
        let intercept = object.intersect(ray);
        if intercept.is_some() && intercept.unwrap().distance < min_dist {
            let intercept = &intercept.unwrap();
            let normal = intercept.normal;
            let l = get_random_ray(normal);
            let shade = normal.dot(&l);
            let bounce = trace(Ray { origin: intercept.location + (normal * 1e-8), direction: l }, scene, depth - 1);
            color.set_color(shade * intercept.color.red * bounce.red,
                            shade * intercept.color.green * bounce.green,
                            shade * intercept.color.blue * bounce.blue);
            min_dist = intercept.distance;
        }
    }
    color
}

/// x is scaled from -0.5 to 0.5, y is scaled by same factor
fn pixel_color(x: f64, y: f64, scene: &Scene) -> Color {
    let ray = Ray { origin: Vector3::new(0.0, 0.0, 0.0), direction: Vector3::new(x, y, 1.0).normalize() };
    let mut out_color = trace(ray, scene, 4);
    for _ in 0..3 {
        out_color = out_color + trace(ray, scene, 4);
    }
    out_color / 4.0
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
    let thing1 = Sphere { origin: Vector3::new(0.0, 0.0, 3.0), diameter: 1.0, color: Color::new(0.5, 0.0, 0.0) };
    let thing2 = Sphere { origin: Vector3::new(1.7, 0.5, 5.0), diameter: 1.0, color: Color::new(0.0, 0.0, 0.5) };
    let thing3 = Sphere { origin: Vector3::new(0.0, 502.0, 3.0), diameter: 1000.0, color: Color::new(0.0, 0.0, 0.5) };
    let scene = Scene { objects: vec![&thing1, &thing2, &thing3] };
    render_image(400, 300, scene);
}
