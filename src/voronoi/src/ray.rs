use cgmath::{Point3, Vector3};

pub struct Ray {
    pub dir: Vector3<f32>,
    pub orig: Point3<f32>,
}

impl Ray {
    pub fn new(orig: Point3<f32>, dir: Vector3<f32>) -> Self {
        Self { orig, dir }
    }

    pub fn at(&self, t: f32) -> Point3<f32> {
        self.orig + t * self.dir
    }
}
