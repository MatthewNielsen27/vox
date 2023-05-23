#[derive(Default, Copy, Clone)]
pub struct Pt3(pub [f32; 3]);

pub type Normal = Pt3;
pub type Vertex = Pt3;

#[derive(Default, Copy, Clone)]
pub struct Facet {
    pub tri: [Vertex; 3],
    pub normal: Normal
}
