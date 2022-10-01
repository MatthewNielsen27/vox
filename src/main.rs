use std::path::Path;
use image::{ImageBuffer, RgbImage, Rgb};
use nalgebra::{Point3, Vector3};
use vox::camera::Viewport;

use vox::grr;
use vox::fwd::raster;
use vox::camera;

use vox::stl::{mesh_from_stl};

use rand::Rng;

fn main() {
    let mut img: RgbImage = ImageBuffer::new(512, 512);

    let mut rng = rand::thread_rng();

    // let tri = Triangle2D {
    //     points: (
    //         Pixel{x: 10, y: 10 },
    //         Pixel{ x: 400, y: 80 },
    //         Pixel{ x: 200, y: 300 }
    //     )
    // };

    // render_triangle(&mut img, &tri, Rgb([252, 165, 3]));
    // grr::render_triangle_shader(&mut img, &tri, Rgb([252, 165, 3]), (1.0, 0.0, 0.0));

    let view = Viewport{
        d: 1.0,
        w: 1.0,
        h: 1.0
    };

    let camera_view = camera::Camera{ location: Point3::from([0.0,0.0,0.0]) }.looking_at(Point3::from([0.0,0.0,1.0]));

    let canvas = (512, 512);

    let mut random_col = || -> Rgb<u8> {
        let r: u8 = rng.gen();
        let g: u8 = rng.gen();
        let b: u8 = rng.gen();
        Rgb::from([r, g, b])
    };

    let mesh = mesh_from_stl(Path::new("/Users/matthewnielsen/Documents/cube_ascii.stl")).unwrap();

    // todo: we need to use nalgebra::Isometry3 transformation matrices.
    let transformation = nalgebra::Isometry3::translation(1.0, 1.5, 5.0);

    for face in &mesh.faces {
        let p1 = transformation.transform_point(mesh.get_vertex(face.vertices[0]));
        let p2 = transformation.transform_point(mesh.get_vertex(face.vertices[1]));
        let p3 = transformation.transform_point(mesh.get_vertex(face.vertices[2]));

        let p1 = view.point_projection_canvas(p1, &canvas);
        let p2 = view.point_projection_canvas(p2, &canvas);
        let p3 = view.point_projection_canvas(p3, &canvas);
        //
        let tri = raster::Triangle2D{points: (p1, p2, p3)};
        //
        // grr::render_triangle(&mut img, &tri, Rgb([64, 235, 52]));
        grr::render_triangle_wireframe(&mut img, &tri, random_col());
    }

    img.save("scene.png").unwrap();
}