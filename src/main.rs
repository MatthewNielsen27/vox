use std::path::Path;
use image::{ImageBuffer, RgbImage, Rgb};
use nalgebra::Vector3;
use vox::camera::Viewport;

use vox::grr;
use vox::fwd::{Vertex2D, Triangle2D, Vertex3D};

use vox::stl::facets_from_ascii_stl;

use rand::Rng;

fn main() {
    let mut img: RgbImage = ImageBuffer::new(512, 512);

    let mut rng = rand::thread_rng();

    // let tri = Triangle2D {
    //     points: (
    //         Vertex2D{x: 10, y: 10 },
    //         Vertex2D{ x: 400, y: 80 },
    //         Vertex2D{ x: 200, y: 300 }
    //     )
    // };

    // render_triangle(&mut img, &tri, Rgb([252, 165, 3]));
    // grr::render_triangle_shader(&mut img, &tri, Rgb([252, 165, 3]), (1.0, 0.0, 0.0));

    let view = Viewport{
        d: 1.0,
        w: 1.0,
        h: 1.0
    };

    let canvas = (512, 512);

    let mut random_col = || -> Rgb<u8> {
        let r: u8 = rng.gen();
        let g: u8 = rng.gen();
        let b: u8 = rng.gen();
        Rgb::from([r, g, b])
    };

    // front vertices
    // let vAf = view.point_projection_canvas(Vector3::from([-2.0, -0.5, 5.0]), &canvas);
    // let vBf = view.point_projection_canvas(Vector3::from([-2.0,  0.5, 5.0]), &canvas);
    // let vCf = view.point_projection_canvas(Vector3::from([-1.0,  0.5, 5.0]), &canvas);
    // let vDf = view.point_projection_canvas(Vector3::from([-1.0, -0.5, 5.0]), &canvas);
    //
    // // back vertices
    // let vAb = view.point_projection_canvas(Vector3::from([-2.0, -0.5, 6.0]), &canvas);
    // let vBb = view.point_projection_canvas(Vector3::from([-2.0,  0.5, 6.0]), &canvas);
    // let vCb = view.point_projection_canvas(Vector3::from([-1.0,  0.5, 6.0]), &canvas);
    // let vDb = view.point_projection_canvas(Vector3::from([-1.0, -0.5, 6.0]), &canvas);
    //
    // grr::render_line(&mut img, vAf, vBf);
    // grr::render_line(&mut img, vBf, vCf);
    // grr::render_line(&mut img, vCf, vDf);
    // grr::render_line(&mut img, vDf, vAf);
    //
    // grr::render_line(&mut img, vAb, vBb);
    // grr::render_line(&mut img, vBb, vCb);
    // grr::render_line(&mut img, vCb, vDb);
    // grr::render_line(&mut img, vDb, vAb);
    //
    // img.save("fractal.png").unwrap();

    let facets = facets_from_ascii_stl(Path::new("/Users/matthewnielsen/Documents/cube_ascii.stl")).unwrap();

    for facet in facets {
        let p1 = view.point_projection_canvas(facet.data[0] - Vector3::from([-1.0, 1.5, -5.0]), &canvas);
        let p2 = view.point_projection_canvas(facet.data[1] - Vector3::from([-1.0, 1.5, -5.0]), &canvas);
        let p3 = view.point_projection_canvas(facet.data[2] - Vector3::from([-1.0, 1.5, -5.0]), &canvas);

        let tri = Triangle2D{points: (p1, p2, p3)};

        // grr::render_triangle(&mut img, &tri, Rgb([64, 235, 52]));
        grr::render_triangle_wireframe(&mut img, &tri, random_col());

        // grr::render_line(&mut img, p1, p2);
        // grr::render_line(&mut img, p2, p3);
        // grr::render_line(&mut img, p3, p1);
    }

    img.save("fractal.png").unwrap();
}
