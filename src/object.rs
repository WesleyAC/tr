use nalgebra::Vector3;

use ray::Ray;

pub trait Object {
    fn intersect(&self, ray: Ray) -> Option<f64>;
}

pub struct Sphere {
    pub origin: Vector3<f64>,
    pub diameter: f64
}

impl Object for Sphere {
    fn intersect(&self, ray: Ray) -> Option<f64> {
        let dist = &ray.origin - self.origin;
        let b = 2.0 * &ray.direction.dot(&dist);
        let c = &dist.dot(&dist) - (self.diameter / 2.0).powi(2);
        let desc = b*b - 4.0*c;
        if desc < 0.0 {
            None
        } else {
            if desc == 0.0 {
                Some(-b / 2.0)
            } else {
                let i1 = (-b + desc.sqrt()) / 2.0;
                let i2 = (-b - desc.sqrt()) / 2.0;
                Some(f64::min(i1, i2))
            }
        }
    }
}
