use nalgebra::{Vector3, norm_squared};

use ray::Ray;

pub trait Object {
    fn intersect(&self, ray: Ray) -> bool;
}

pub struct Sphere {
    pub origin: Vector3<f64>,
    pub diameter: f64
}

impl Object for Sphere {
    fn intersect(&self, ray: Ray) -> bool {
        let l = &ray.origin - &self.origin;
        let tca = l.dot(&ray.direction);
        let d2 = norm_squared(&l) - (tca * tca);
        d2 < self.diameter
    }
}
