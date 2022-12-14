use std::iter::{zip};

use std::mem;

use nalgebra as na;
use nalgebra::Point3;
use nalgebra_glm::pi;

use crate::geometry;
use crate::fwd::{Vertex3Ndc, Vertex3};
use crate::model::Model;
use crate::raster;
use crate::raster::linspace_sample;
use crate::surface::Surface;
use crate::camera::CameraInfo;
use crate::clipping::{BoundingSphere, clip_triangle, ClippedTriangle, ClipType, get_clip_type, get_clipping_planes};
use crate::geometry::Triangle;

pub fn line_between(p1: raster::Pixel, p2: raster::Pixel) -> Vec<raster::Pixel> {
    return if (p2.y - p1.y).abs() < (p2.x - p1.x).abs() {
        if p2.x > p1.x {
            pll(p1, p2)
        } else {
            pll(p2, p1)
        }
    } else {
        if p2.y > p1.y {
            plh(p1, p2)
        } else {
            plh(p2, p1)
        }
    };
}

fn pll(p1: raster::Pixel, p2: raster::Pixel) -> Vec<raster::Pixel> {
    let dx  = (p2.x - p1.x) as i32;
    let mut dy = (p2.y - p1.y) as i32;
    let mut yi :i32 = 1;
    if dy < 0 {
        yi = -1;
        dy = -1 * dy;
    }

    let mut d = (2 * dy) - dx;
    let mut y = p1.y as i32;

    let mut points = vec![];

    for x in p1.x..=p2.x {
        points.push(raster::Pixel{x, y});

        if d > 0 {
            y += yi;
            d += 2 * (dy - dx);
        } else {
            d += 2 * dy;
        }
    }

    points
}

fn plh(p1: raster::Pixel, p2: raster::Pixel) -> Vec<raster::Pixel> {
    let mut dx  = (p2.x - p1.x) as i32;
    let dy = (p2.y - p1.y) as i32;
    let mut xi :i32 = 1;
    if dx < 0 {
        xi = -1;
        dx = -1 * dx;
    }

    let mut d = (2 * dx) - dy;
    let mut x = p1.x as i32;

    let mut points = vec![];

    for y in p1.y..=p2.y {
        points.push(raster::Pixel{x, y});

        if d > 0 {
            x += xi;
            d += 2 * (dx - dy);
        } else {
            d += 2 * dx;
        }
    }

    points
}

/// Fill a triangle
pub fn fill(raw_tri: &raster::Triangle2D) -> Vec<raster::ScanlineH> {
    // --
    // Now let's sort the vertices in ascending order
    let tri = {
        let mut i0 = 0;
        let mut i1 = 1;
        let mut i2 = 2;

        if raw_tri.points[i1].y > raw_tri.points[i0].y { mem::swap(&mut i1, &mut i0); }
        if raw_tri.points[i2].y > raw_tri.points[i0].y { mem::swap(&mut i2, &mut i0); }
        if raw_tri.points[i2].y > raw_tri.points[i1].y { mem::swap(&mut i2, &mut i1); }

        raster::Triangle2D{
            points: [
                raw_tri.points[i2],
                raw_tri.points[i1],
                raw_tri.points[i0]
            ]
        }
    };

    // We need to assert these equality conditions...
    assert!(tri.points[2].y >= tri.points[1].y);
    assert!(tri.points[1].y >= tri.points[0].y);

    let p012 = {
        let mut tmp = raster::interp_pixels(tri.points[0], tri.points[1]);
        tmp.pop(); // This is because tmp[-1] == tmp_12[0]
        let mut tmp_12 = raster::interp_pixels(tri.points[1], tri.points[2]);
        tmp.append(&mut tmp_12);
        tmp
    };

    let p02 = raster::interp_pixels(tri.points[0], tri.points[2]);

    assert_eq!(p012.len(), p02.len());

    // --
    // Now we need to determine the left and right sets of points
    let mut lefts = p012;
    let mut rights = p02;

    let mid = rights.len() / 2;
    if rights[mid].x < lefts[mid].x {
        mem::swap(&mut lefts, &mut rights);
    }

    let tmp = zip(lefts, rights).map(|p| raster::ScanlineH{l: p.0, r: p.1}).collect();
    tmp
}

/// [returns] triangle scanlines.
pub fn scanlines(&raw_tri: &raster::Triangle2D) -> Vec<raster::ScanlineH> {
    let tri = {
        let is = raw_tri.get_indices_sorted();
        raster::Triangle2D {
            points: [
                raw_tri.points[is.2],
                raw_tri.points[is.1],
                raw_tri.points[is.0]
            ]
        }
    };

    let (l, r) = tri.get_sides();
    zip(l, r).map(|points| raster::ScanlineH{l: points.0, r: points.1}).collect()
}

/// [returns] triangle scanlines with attributes interpolated at the endpoints.
pub fn scanlines_with_attributes(&raw_tri: &raster::Triangle2D, raw_attr: &[f32; 3]) -> Vec<(raster::ScanlineH, (f32, f32))> {
    // --
    // Step 1: preprocess the input by sorting it by Y value of `raw_tri`.
    let (tri, attr) = {
        let is = raw_tri.get_indices_sorted();
        (
            raster::Triangle2D {
                points: [
                    raw_tri.points[is.2],
                    raw_tri.points[is.1],
                    raw_tri.points[is.0]
                ]
            },
            [raw_attr[is.2], raw_attr[is.1], raw_attr[is.0]]
        )
    };

    let (sides, attributes) = tri.get_sides_with_attr(&(attr[0], attr[1], attr[2]));

    // --
    // Now we need to zip these things together!
    zip(
        zip(sides.0, sides.1),
        zip(attributes.0, attributes.1)
    ).map(|(points, intensities)| {
        (
            raster::ScanlineH{l: points.0, r: points.1},
            intensities
        )
    }).collect()
}

// pub fn render_line(img: &mut image::RgbImage, p1: raster::Pixel, p2: raster::Pixel) {
//     for p in line_between(p1, p2) {
//         render_pixel(img, p, Rgb([0, 255, 0]));
//     }
//
//     render_pixel(img, p1, Rgb([255, 0, 0]));
//     render_pixel(img, p2, Rgb([255, 0, 0]));
// }

// pub fn render_triangle(
//     img: &mut image::RgbImage,
//     tri: &raster::Triangle2D,
//     col: Rgb<u8>
// ) {
//     render_pixel(img, tri.points[0], Rgb([255, 0, 0]));
//     render_pixel(img, tri.points[1], Rgb([255, 0, 0]));
//     render_pixel(img, tri.points[2], Rgb([255, 0, 0]));
//     render_triangle_fill(img, tri, col);
//     // render_triangle_wireframe(img, tri, col);
// }

// todo: see if we need to render the wireframe as well...
// pub fn render_triangle_shader(
//     img: &mut image::RgbImage,
//     tri: &raster::raster::Triangle2D,
//     col: Rgb<u8>,
//     shader: (f32, f32, f32)
// ) {
//     render_triangle_fill_shader(img, tri, col, shader);
// }

// fn render_triangle_fill(
//     img: &mut image::RgbImage,
//     tri: &raster::raster::Triangle2D,
//     col: Rgb<u8>
// ) {
//     for line in fill(tri) {
//         for x in line.l.x..=line.r.x {
//             render_pixel(img, raster::Pixel{x, y: line.l.y}, col)
//         }
//     }
// }

// fn render_triangle_fill_shader(
//     img: &mut image::RgbImage,
//     tri: &raster::raster::Triangle2D,
//     col: Rgb<u8>,
//     shader: (f32, f32, f32)
// ) {
//     for (line, intensity) in fill_shader(tri, shader) {
//         for (x, h) in raster::linspace_sample(line.l.x, intensity.l as f32, line.r.x, intensity.r as f32) {
//
//             // Come up with a new color
//             let mut c = col.clone();
//             c.apply(|chan| (chan as f32 * h) as u8);
//
//             render_pixel(img, raster::Pixel{x, y: line.l.y}, c);
//         }
//     }
// }

// pub fn render_triangle_wireframe(
//     surface: &mut Surface,
//     tri: &raster::Triangle2D,
//     col: &[u8; 3]
// ) {
//     [
//         line_between(tri.points[0], tri.points[1]),
//         line_between(tri.points[1], tri.points[2]),
//         line_between(tri.points[2], tri.points[0]),
//     ].map( |line| {
//         line.iter().for_each(|point| surface.set_pixel(point.x as usize, point.y as usize, col));
//     });
// }

// pub fn render_tri_wireframe(
//     surface: &mut Surface,
//     tri: &geometry::Triangle<Vertex3Ndc>,
//     col: &[u8; 3]
// ) {
//     let (p0, z0) = surface.to_pixel(&tri.0[0]);
//     let (p1, z1) = surface.to_pixel(&tri.0[1]);
//     let (p2, z2) = surface.to_pixel(&tri.0[2]);
// }

pub fn render_tri(
    surface: &mut Surface,
    tri: &Triangle<Vertex3Ndc>,
    col: &[u8; 3]
) {
    // Step 1: Convert the triangle into a 2D triangle with z-attributes
    let (p0, z0) = surface.to_pixel(&tri.0[0]);
    let (p1, z1) = surface.to_pixel(&tri.0[1]);
    let (p2, z2) = surface.to_pixel(&tri.0[2]);

    scanlines_with_attributes(
        &raster::Triangle2D { points: [p0, p1, p2] },
        &[z0, z1, z2]
    ).iter().for_each(|&(line, z_range)| {
        let y = line.l.y as usize;

        for (x, z) in linspace_sample(line.l.x, z_range.0, line.r.x, z_range.1) {
            let x = x as usize;
            let z = 1.0 / z;

            if x >= surface.shape.0 { continue; }
            if y >= surface.shape.1 { continue; }

            let z_current = surface.get_z(x, y);
            if z > z_current {
                surface.set_pixel(x, y, col);
                surface.set_z(x, y, z);
            }
        }
    });
}

/// [returns]   True if the triangle (in view space) is back-facing.
///
/// [note]      This is known as back-face-culling. For more information.
pub fn is_back_facing(tri: &[Vertex3; 3], eye_ray: &na::Vector3<f32>) -> bool {
    let d1 = tri[1] - tri[0];
    let d2 = tri[2] - tri[0];
    let normal = d1.cross(&d2).xyz();
    normal.dot(&(eye_ray - tri[0].coords)) <= 0.0
}

pub fn render_model(
    model: &Model,
    camera: &CameraInfo,
    proj: &na::Perspective3<f32>,
    surface: &mut Surface,
    col: &[u8; 3]
) {
    let view = camera.get_view_matrix();

    let proj = proj.to_homogeneous();

    let light = Point3::from([0.0, -100.0, 50.0]);

    let tris_view : Vec<(Point3<f32>,Point3<f32>,Point3<f32>)> = model.triangles_world().map(|tri| {
        (
            view.transform_point(&tri.0.0),
            view.transform_point(&tri.1.0),
            view.transform_point(&tri.2.0)
        )
    }).collect();

    let mut points = vec![];
    for tri in &tris_view {
        points.push(tri.0);
        points.push(tri.1);
        points.push(tri.2);
    }

    // We get to return early in this case...
    let bs = BoundingSphere::from(&points[..]);
    for plane in &get_clipping_planes(&proj) {
        let ct = get_clip_type(&bs, plane);
        if ct == ClipType::NopeAllBehind {
            return;
        }
    }

    let mut clipped : Vec<Triangle<Point3<f32>>> = tris_view.into_iter().map(|ps| {
        Triangle([ps.0, ps.1, ps.2])
    }).collect();

    for plane in &get_clipping_planes(&proj) {
        let mut retained = vec![];

        for tri in clipped {
            match clip_triangle(plane, &tri) {
                None => {},

                Some(foo) => {
                    match foo {
                        (ClippedTriangle::NoClip, None) => {
                            retained.push(tri);
                        },

                        (ClippedTriangle::DoubleReplacement(result), None) => {
                            retained.push(result.tri)
                        },

                        (ClippedTriangle::SingleReplacement(new_triangle_1), Some(ClippedTriangle::DoubleReplacement(new_triangle_2))) => {
                            retained.push(new_triangle_1.tri);
                            retained.push(new_triangle_2.tri);
                        },

                        _ => { panic!("unhandled case!")}
                    }
                }
            }
        }

        clipped = retained;
    }

    // todo: decide on the right time to operate upon indices
    for tri in clipped {
        let p0_view = tri.0[0];
        let p1_view = tri.0[1];
        let p2_view = tri.0[2];

        // This is known as back-face culling
        if is_back_facing(&[p0_view, p1_view, p2_view], &(camera.eye - camera.target)) {
            continue
        }

        // todo: this will be replaced by a fragment shader
        let d1 = p1_view - p0_view;
        let d2 = p2_view - p0_view;
        let normal = d1.cross(&d2).xyz().normalize();

        let light_ray = (p0_view - light).normalize();

        let theta = (normal.dot(&light_ray) / (normal.norm() * light_ray.norm())).acos();
        let theta_mult = theta / pi::<f32>();

        let col = [
            (col[0] as f32 * theta_mult) as u8,
            (col[1] as f32 * theta_mult) as u8,
            (col[2] as f32 * theta_mult) as u8
        ];

        // Step 2: clip triangles that are outside the frame.

        // Step 3: convert the triangle into NDC space.
        let p0 = Vertex3Ndc(proj.transform_point(&p0_view));
        let p1 = Vertex3Ndc(proj.transform_point(&p1_view));
        let p2 = Vertex3Ndc(proj.transform_point(&p2_view));

        render_tri(surface, &Triangle([p0, p1, p2]), &col);
    }
}
