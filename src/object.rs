use nalgebra::Vector3;

use ray::Ray;
use color::Color;

pub trait Object {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
}

#[derive(Copy, Clone, Debug)]
pub struct Intersection {
    pub normal: Vector3<f64>,
    pub location: Vector3<f64>,
    pub distance: f64,
    pub color: Color
}

pub struct Sphere {
    pub origin: Vector3<f64>,
    pub diameter: f64,
    pub color: Color
}

impl Object for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let dist = &ray.origin - &self.origin;
        let b = 2.0 * &ray.direction.dot(&dist);
        let c = &dist.dot(&dist) - (self.diameter / 2.0).powi(2);
        let desc = b*b - 4.0*c;

        if desc < 0.0 {
            None
        } else {
            let mut dist: f64 = 0.0;
            let i1 = (-b + desc.sqrt()) / 2.0;
            let i2 = (-b - desc.sqrt()) / 2.0;
            if (i1 > 0.0) && (i1 < i2) {
                dist = i1;
            } else if (i2 > 0.0) && (i2 <= i1) {
                dist = i2;
            } else {
                return None;
            }
            let intercept_point: Vector3<f64> = &ray.origin + (dist * &ray.direction);
            let normal: Vector3<f64> = (intercept_point - &self.origin).normalize();
            Some(Intersection { normal: normal, location: intercept_point, distance: dist, color: self.color})
        }
    }
}
