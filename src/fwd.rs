extern crate nalgebra as na;

pub type Vertex3 = na::Point3<f32>;

pub struct Vertex3Ndc(pub Vertex3);

impl Vertex3Ndc {
    /// [returns] true if all points are within [0.0, 1.0]
    pub fn is_valid(&self) -> bool {
            (-1.0 <= self.0.x && self.0.x <= 1.0)
        &&  (-1.0 <= self.0.x && self.0.x <= 1.0)
        &&  (-1.0 <= self.0.y && self.0.y <= 1.0)
        &&  (-1.0 <= self.0.y && self.0.y <= 1.0)
        &&  (-1.0 <= self.0.z && self.0.z <= 1.0)
        &&  (-1.0 <= self.0.z && self.0.z <= 1.0)
    }
}
