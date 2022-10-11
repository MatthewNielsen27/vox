use std::iter::{zip};
use image::{Pixel, Rgb};

use crate::fwd::{raster, LinearIntensity};

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

use std::mem;
use crate::fwd::raster::Triangle2D;

pub fn interp_points(p0: raster::Pixel, p1: raster::Pixel) -> Vec<raster::Pixel> {
    linspace_sample(p0.y, p0.x as f32, p1.y, p1.x as f32)
        .iter()
        .map(|(y,x)| raster::Pixel{x: *x as i32, y: *y})
        .collect()
}

/// linspace sampling between the 2 points
pub fn linspace_sample(
    mut x0: i32, mut y0: f32,
    mut x1: i32, mut y1: f32
)
    -> Vec<(i32, f32)>
{
    if x1 == x0 {
        // While we could return either value in this case, we should return the first value because
        // it works better for us.
        //
        // For example: We would get discontinuity when sampling the line segments AC and ABC of a
        //              triangle.
        //
        //                                      a
        //                                      |\
        //                                      | \
        //                                      |  \
        //                                    b |___\ c
        //
        //        why?  Say that ABC is on the left of AC and the line BC is horizontal. If we
        //              returned p1 instead of p0, then the last sample would jump across the
        //              triangle to vertex C. Since the line AC ends on C, then we'd be missing
        //              the last scanline of the triangle because the 2 points are the same.
        //
        return Vec::from([(x0, y0)]);
    }

    // reorder if need be
    if x1 < x0 {
        mem::swap(&mut x1, &mut x0);
        mem::swap(&mut y1, &mut y0);
    }

    let m = (y1 - y0) / (x1 - x0) as f32;
    let b = y0 - (m * x0 as f32);

    (x0..=x1).map(|x| {
        (x, ((m * x as f32) + b))
    }).collect()
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

        Triangle2D{
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
        let mut tmp = interp_points(tri.points[0], tri.points[1]);
        tmp.pop(); // This is because tmp[-1] == tmp_12[0]
        let mut tmp_12 = interp_points(tri.points[1], tri.points[2]);
        tmp.append(&mut tmp_12);
        tmp
    };

    let p02 = interp_points(tri.points[0], tri.points[2]);

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

pub fn fill_shader(raw_tri: &raster::Triangle2D, raw_shader: (f32, f32, f32)) -> Vec<(raster::ScanlineH, LinearIntensity)> {
    // --
    // Now let's sort the vertices/shader in ascending order
    let (tri, shader) = {
        let mut shader = raw_shader.clone();

        let y0 = raw_tri.points[0].y;
        let y1 = raw_tri.points[1].y;
        let y2 = raw_tri.points[2].y;
        let mut i0 = 0;
        let mut i1 = 1;
        let mut i2 = 2;

        if y1 > y0 {
            mem::swap(&mut i1, &mut i0);
            mem::swap(&mut shader.1, &mut shader.0);
        }
        if y2 > y0 {
            mem::swap(&mut i2, &mut i0);
            mem::swap(&mut shader.2, &mut shader.0);
        }
        if y2 > y1 {
            mem::swap(&mut i2, &mut i1);
            mem::swap(&mut shader.2, &mut shader.1);
        }

        let tri = Triangle2D{
            points: [
                raw_tri.points[i2],
                raw_tri.points[i1],
                raw_tri.points[i0]
            ]
        };

        (tri, shader)
    };

    assert!(tri.points[2].y >= tri.points[1].y);
    assert!(tri.points[1].y >= tri.points[0].y);

    let p012 = {
        let mut tmp = interp_points(tri.points[0], tri.points[1]);
        tmp.pop(); // This is because tmp[-1] == tmp_12[0]
        let mut tmp_12 = interp_points(tri.points[1], tri.points[2]);
        tmp.append(&mut tmp_12);
        tmp
    };

    let h012 = {
        let mut tmp = linspace_sample(tri.points[0].y, shader.0, tri.points[1].y, shader.1);
        tmp.pop(); // This is because tmp[-1] == tmp_12[0]
        let mut tmp_12 = linspace_sample(tri.points[1].y, shader.1, tri.points[2].y, shader.2);
        tmp.append(&mut tmp_12);
        tmp
    };

    let p02 = interp_points(tri.points[0], tri.points[2]);
    let h02 = linspace_sample(tri.points[0].y, shader.0, tri.points[2].y, shader.2);

    assert_eq!(p012.len(), p02.len());
    assert_eq!(h012.len(), h02.len());

    // --
    // Now we need to determine the left and right sets of points
    let mut lefts = p012;
    let mut rights = p02;

    let mut lefts_h = h012;
    let mut rights_h = h02;

    let mid = rights.len() / 2;
    if rights[mid].x < lefts[mid].x {
        mem::swap(&mut lefts, &mut rights);
        mem::swap(&mut lefts_h, &mut rights_h);
    }

    // --
    // Now we need to zip these things together!
    zip(
        zip(lefts, rights),
        zip(lefts_h, rights_h)
    ).map(|(points, intensities)| {
        (
            raster::ScanlineH{l: points.0, r: points.1},
            LinearIntensity{l: intensities.0.1, r: intensities.1.1}
        )
    }).collect()
}

// render pixel to image
pub fn render_pixel(img: &mut image::RgbImage, v: raster::Pixel, c: Rgb<u8>) {
    if (v.x < 0 || v.x >= img.width() as i32) || (v.y < 0 || v.y >= img.height() as i32) {
        return;
    }

    img.put_pixel(v.x as u32, v.y as u32, c);
}

pub fn render_line(img: &mut image::RgbImage, p1: raster::Pixel, p2: raster::Pixel) {
    for p in line_between(p1, p2) {
        render_pixel(img, p, Rgb([0, 255, 0]));
    }

    // todo: see if we need these...
    render_pixel(img, p1, Rgb([255, 0, 0]));
    render_pixel(img, p2, Rgb([255, 0, 0]));
}

pub fn render_triangle(
    img: &mut image::RgbImage,
    tri: &raster::Triangle2D,
    col: Rgb<u8>
) {
    render_pixel(img, tri.points[0], Rgb([255, 0, 0]));
    render_pixel(img, tri.points[1], Rgb([255, 0, 0]));
    render_pixel(img, tri.points[2], Rgb([255, 0, 0]));
    render_triangle_fill(img, tri, col);
    // render_triangle_wireframe(img, tri, col);
}

// todo: see if we need to render the wireframe as well...
pub fn render_triangle_shader(
    img: &mut image::RgbImage,
    tri: &raster::Triangle2D,
    col: Rgb<u8>,
    shader: (f32, f32, f32)
) {
    render_triangle_fill_shader(img, tri, col, shader);
}

fn render_triangle_fill(
    img: &mut image::RgbImage,
    tri: &raster::Triangle2D,
    col: Rgb<u8>
) {
    for line in fill(tri) {
        for x in line.l.x..=line.r.x {
            render_pixel(img, raster::Pixel{x, y: line.l.y}, col)
        }
    }
}

fn render_triangle_fill_shader(
    img: &mut image::RgbImage,
    tri: &raster::Triangle2D,
    col: Rgb<u8>,
    shader: (f32, f32, f32)
) {
    for (line, intensity) in fill_shader(tri, shader) {
        for (x, h) in linspace_sample(line.l.x, intensity.l as f32, line.r.x, intensity.r as f32) {

            // Come up with a new color
            let mut c = col.clone();
            c.apply(|chan| (chan as f32 * h) as u8);

            render_pixel(img, raster::Pixel{x, y: line.l.y}, c);
        }
    }
}

pub fn render_triangle_wireframe(
    img: &mut image::RgbImage,
    tri: &raster::Triangle2D,
    col: Rgb<u8>
) {
    [
        line_between(tri.points[0], tri.points[1]),
        line_between(tri.points[1], tri.points[2]),
        line_between(tri.points[2], tri.points[0]),
    ].map( |line| {
        line.iter().for_each(|point| render_pixel(img, *point, col));
    });
}
