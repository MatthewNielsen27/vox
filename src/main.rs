use std::path::Path;
use image::{ImageBuffer, RgbImage, Rgb};
use nalgebra::{Point3, Vector3};
use vox::camera::Viewport;

use vox::grr;
use vox::fwd::{raster, Vertex3D};
use vox::camera;

use vox::stl::{mesh_from_stl};

use rand::Rng;

fn main() {
    let mut img: RgbImage = ImageBuffer::new(1600, 900);

    let mut rng = rand::thread_rng();

    let viewport = Viewport{
        z_minmax: (0.0, 1.0),
        size: (1600, 900),
        upper_left: (0, 0)
    };

    let mut random_col = || -> Rgb<u8> {
        let r: u8 = rng.gen();
        let g: u8 = rng.gen();
        let b: u8 = rng.gen();
        Rgb::from([r, g, b])
    };

    // Our camera looks toward the point (1.0, 0.0, 0.0).
    // It is located at (0.0, 0.0, 1.0).
    let eye    = Point3::new(4.0, -1.5, -2.0);
    let target = Point3::new(0.0, 0.0, 0.0);
    let view   = nalgebra::Isometry3::look_at_rh(&eye, &target, &Vector3::y());

    // A perspective projection.
    let projection = nalgebra::Perspective3::new(16.0 / 9.0, 3.14 / 2.0, 1.0, 1000.0);

    let mesh = mesh_from_stl(Path::new("resources/cube_ascii.stl")).unwrap();

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
        let p0 = model_view_projection.transform_point(&mesh.get_vertex(face.vertices[0]).0);
        let p1 = model_view_projection.transform_point(&mesh.get_vertex(face.vertices[1]).0);
        let p2 = model_view_projection.transform_point(&mesh.get_vertex(face.vertices[2]).0);

        println!("{}", p0);
        println!("{}", p1);
        println!("{}", p2);

        let p0 = viewport.transform_point(&p0);
        let p1 = viewport.transform_point(&p1);
        let p2 = viewport.transform_point(&p2);

        let p0 = raster::Pixel{x: p0.x as i32, y: p0.y as i32};
        let p1 = raster::Pixel{x: p1.x as i32, y: p1.y as i32};
        let p2 = raster::Pixel{x: p2.x as i32, y: p2.y as i32};
        // println!("triangle: <{},{}> <{},{}> <{},{}>", p0.x, p0.y, p1.x, p1.y, p2.x, p2.y);

        //
        let tri = raster::Triangle2D{points: (p0, p1, p2)};
        //
        grr::render_triangle(&mut img, &tri, random_col());
    }

    img.save("scene.png").unwrap();
}
