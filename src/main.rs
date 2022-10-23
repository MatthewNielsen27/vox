use std::path::Path;
use image::{Rgb};
use nalgebra::{Point3, Vector3};

use vox::grr;
use vox::geometry;

use vox::stl::{mesh_from_stl};

use rand::Rng;
use vox::fwd::Vertex3Ndc;
use vox::surface::Surface;

fn main() {
    let mut surface = Surface::new(1600, 900);

    let mut rng = rand::thread_rng();

    let mut random_col = || -> Rgb<u8> {
        let r: u8 = rng.gen();
        let g: u8 = rng.gen();
        let b: u8 = rng.gen();
        Rgb::from([r, g, b])
    };

    // https://carmencincotti.com/2022-05-02/homogeneous-coordinates-clip-space-ndc/

    // Our camera looks toward the point (1.0, 0.0, 0.0).
    // It is located at (0.0, 0.0, 1.0).
    let eye    = Point3::new(1.5, 1.5, 4.0);
    let target = Point3::new(0.0, 0.0, 0.0);
    let view   = nalgebra::Isometry3::look_at_rh(&eye, &target, &Vector3::y());

    // A perspective projection.
    let projection = nalgebra::Perspective3::new(16.0 / 9.0, 3.14 / 2.0, 0.1, 100.0);

    let mesh = mesh_from_stl(Path::new("/Users/matthewnielsen/Documents/sphere-ascii.stl")).unwrap();
    // let mesh = mesh_from_stl(Path::new("resources/cube_ascii.stl")).unwrap();

    let world_from_model = nalgebra::Isometry3::translation(0.0, 0.0, 0.0);

    // The combination of the model with the view is still an isometry.
    let model_view = view * world_from_model;

    // Convert everything to a `Matrix4` so that they can be combined.
    let mat_model_view = model_view.to_homogeneous();

    // Combine everything.
    let model_view_projection = projection.as_matrix() * mat_model_view;

    println!("{}", model_view_projection);

    for face in &mesh.faces {
        // This transforms the coordinate into NDC space (normalized-device-coordinate space)
        let p0_view = mat_model_view.transform_point(&mesh.get_vertex(face.vertices[0]).0);
        let p1_view = mat_model_view.transform_point(&mesh.get_vertex(face.vertices[1]).0);
        let p2_view = mat_model_view.transform_point(&mesh.get_vertex(face.vertices[2]).0);

        // This is back-face culling
        if grr::is_back_facing(&[p0_view, p1_view, p2_view], &eye) {
           continue
        }

        // Step 1: clip triangles in view
        // Step 2: discard back-facing triangles
        // Step 3:

        let p0 = model_view_projection.transform_point(&mesh.get_vertex(face.vertices[0]).0);
        let p1 = model_view_projection.transform_point(&mesh.get_vertex(face.vertices[1]).0);
        let p2 = model_view_projection.transform_point(&mesh.get_vertex(face.vertices[2]).0);

        let tri = geometry::Triangle([Vertex3Ndc(p0), Vertex3Ndc(p1), Vertex3Ndc(p2)]);

        grr::render_tri(&mut surface, &tri, &random_col().0);
    }

    surface.to_img().save("foo.png").unwrap();
}
