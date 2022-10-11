use std::path::Path;
use image::{ImageBuffer, Rgb, RgbImage};
use nalgebra::{Point3, Vector3};
use vox::camera::Viewport;
use vox::{grr, stl};
use vox::clipping;
use vox::clipping::ClippedTriangle;
use vox::fwd::raster;
use vox::geometry::{Plane, Triangle};
use vox::debug_utils;

#[test]
fn test_clipping_bounding_sphere() {
    // todo: We need a UnitSolids type thing
    let mesh = stl::mesh_from_stl(Path::new("resources/cube_ascii.stl")).unwrap();
    assert_eq!(mesh.faces.len(), 12);
    assert_eq!(mesh.vertices.len(), 8);

    let vertices : Vec<Point3<f32>> = mesh.vertices.iter().map(|vert| vert.vtx.0).collect();

    let bs = clipping::BoundingSphere::from(&vertices);
    assert_eq!(bs.center, Point3::from([0.5, 0.5, 0.5]));
    assert_eq!(bs.radius, 0.8660254);

    // --
    // [Scenario] Clip plane is below the model, pointing upwards
    let cp = Plane::from(
        Vector3::from([0.0, 0.0, 1.0]),
        Vector3::from([0.0, 0.0, -1.0])
    );
    assert_eq!(clipping::get_clip_type(&bs, &cp), clipping::ClipType::NopeAllFront);

    // --
    // [Scenario] Clip plane is above the model, pointing upwards
    let cp = Plane::from(
        Vector3::from([0.0, 0.0, 1.0]),
        Vector3::from([0.0, 0.0, 2.0])
    );
    assert_eq!(clipping::get_clip_type(&bs, &cp), clipping::ClipType::NopeAllBehind);

    // --
    // [Scenario] Clip plane is in the middle of the model
    let cp = Plane::from(
        Vector3::from([0.0, 0.0, 1.0]),
        Vector3::from([0.0, 0.0, 0.5])
    );
    assert_eq!(clipping::get_clip_type(&bs, &cp), clipping::ClipType::Clip);
}

#[test]
fn test_clipping_triangle() {
    let mut img: RgbImage = ImageBuffer::new(100, 100);

    // [Scenario] The triangle has 1 vertex below the plane.
    {
        let tri = Triangle([
            Point3::from([0.0, 0.0, 1.0]),
            Point3::from([-1.0, 0.0, 1.0]),
            Point3::from([1.0, 0.0, -1.0])
        ]);

        let plane = Plane::from(
            Vector3::from([0.0, 0.0, 1.0]),
            Vector3::from([0.0, 0.0, 0.0])
        );

        let (tri_a, tri_b) = clipping::clip_triangle(&plane, &tri).unwrap();
        assert!(matches!(tri_a, clipping::ClippedTriangle::SingleReplacement{ .. }));
        assert!(matches!(tri_b.unwrap(), clipping::ClippedTriangle::DoubleReplacement{ .. }));
    }

    // [Scenario] The triangle has 2 vertices below the plane.
    {
        let mut canvas = debug_utils::Canvas::from((100, 100));

        let tri = Triangle([
            Point3::from([0.0, 10.0, 20.0]),
            Point3::from([-20.0, 10.0, -20.0]),
            Point3::from([20.0, 10.0, -20.0])
        ]);

        let plane = Plane::from(
            Vector3::from([0.0, 0.0, 1.0]),
            Vector3::from([0.0, 0.0, 0.0])
        );

        let hdw = 50;

        // let p0 = tri.0[0];
        // let p1 = tri.0[1];
        // let p2 = tri.0[2];
        //
        // let p0 = raster::Pixel{x: p0.x as i32 + hdw, y: p0.z as i32 + hdw};
        // let p1 = raster::Pixel{x: p1.x as i32 + hdw, y: p1.z as i32 + hdw};
        // let p2 = raster::Pixel{x: p2.x as i32 + hdw, y: p2.z as i32 + hdw};
        //
        // let tri2d = raster::Triangle2D{points: [p0, p1, p2]};
        //
        // grr::render_triangle(&mut img, &tri2d, Rgb::from([255, 0, 0]));

        let (clip, _) = clipping::clip_triangle(&plane, &tri).unwrap();
        assert!(matches!(clip, clipping::ClippedTriangle::DoubleReplacement{ .. }));

        // match clip {
        //     ClippedTriangle::DoubleReplacement(t) => {
        //         let p0 = &t.tri.0[0];
        //         let p1 = &t.tri.0[1];
        //         let p2 = &t.tri.0[2];
        //
        //         let p0 = raster::Pixel{x: p0.x as i32 + hdw, y: p0.z as i32 + hdw};
        //         let p1 = raster::Pixel{x: p1.x as i32 + hdw, y: p1.z as i32 + hdw};
        //         let p2 = raster::Pixel{x: p2.x as i32 + hdw, y: p2.z as i32 + hdw};
        //
        //         let tri2d = raster::Triangle2D{points: [p0, p1, p2]};
        //
        //         grr::render_triangle(&mut img, &tri2d, random_col());
        //     }
        //     _ => {}
        // }

        let tri = Triangle([
            Point3::from([0.0, 0.0, 20.0]),
            Point3::from([-20.0, 0.0, 20.0]),
            Point3::from([20.0, 0.0, -20.0])
        ]);

        // canvas.render_triangle(
        //     &Triangle([tri.0[0].xzy(), tri.0[1].xzy(), tri.0[2].xzy()]),
        //     &debug_utils::random_col()
        // );

        let (tri_1, tri_2) = clipping::clip_triangle(&plane, &tri).unwrap();
        match tri_1 {
            ClippedTriangle::SingleReplacement(t) => {
                canvas.render_triangle(
                    &Triangle([t.tri.0[0].xzy(), t.tri.0[1].xzy(), t.tri.0[2].xzy()]),
                    &debug_utils::random_col()
                );
            }
            _ => {
                panic!("foo");
            }
        }

        match tri_2.unwrap() {
            ClippedTriangle::DoubleReplacement(t) => {
                // canvas.render_triangle(
                //     &Triangle([t.tri.0[0].xzy(), t.tri.0[1].xzy(), t.tri.0[2].xzy()]),
                //     &debug_utils::random_col()
                // );
            }
            _ => {
                panic!("foo");
            }
        }

        canvas.save("test_cast_1.png");
    }

    // [Scenario] The triangle is completely above the plane.
    {
        let tri = Triangle([
            Point3::from([0.0, 0.0, 1.0]),
            Point3::from([-1.0, 0.0, -1.0]),
            Point3::from([1.0, 0.0, -1.0])
        ]);

        let plane = Plane::from(
            Vector3::from([0.0, 0.0, 1.0]),
            Vector3::from([0.0, 0.0, -10.0])
        );

        let (clip, _) = clipping::clip_triangle(&plane, &tri).unwrap();
        assert!(matches!(clip, clipping::ClippedTriangle::NoClip));
    }

    // [Scenario] The triangle is completely below the plane.
    {
        let tri = Triangle([
            Point3::from([0.0, 0.0, -1.0]),
            Point3::from([-1.0, 0.0, -2.0]),
            Point3::from([1.0, 0.0, -2.0])
        ]);

        let plane = Plane::from(
            Vector3::from([0.0, 0.0, 1.0]),
            Vector3::from([0.0, 0.0, 0.0])
        );

        assert!(clipping::clip_triangle(&plane, &tri).is_none());
    }
}
