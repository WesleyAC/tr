use nalgebra::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>
}
