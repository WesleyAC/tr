use nalgebra::{Vector3, norm_squared};

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
        let l = &ray.origin - &self.origin;
        let adj = l.dot(&ray.direction);
        let intersect_dist2 = norm_squared(&l) - (adj * adj);
        let radius2 = (self.diameter / 2.0) * (self.diameter / 2.0);
        if intersect_dist2 > radius2 {
            return None
        }
        let outer_dist = (adj - intersect_dist2).sqrt();
        let i1 = adj - outer_dist;
        let i2 = adj + outer_dist;
        if i1 < 0.0 && i2 < 0.0 {
            None
        } else {
            if i1 < i2 {
                Some(i1)
            } else {
                Some(i2)
            }
        }
    }
}
