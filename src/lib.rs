
#[derive(Copy, Clone)]
pub struct Vertex2D {
    x: i32,
    y: i32
}

#[derive(Copy, Clone)]
pub struct Triangle2D((Vertex2D,Vertex2D,Vertex2D));

#[derive(Copy, Clone)]
pub struct HLine((Vertex2D,Vertex2D));

#[derive(Copy, Clone)]
pub struct LinearIntensity {
    l: f32,
    r: f32
}

pub mod grr {
    use std::borrow::Borrow;
    use std::fmt::Debug;
    use std::iter::{zip};
    use crate::{Vertex2D, Triangle2D, HLine, LinearIntensity};

    pub fn line_between(p1: Vertex2D, p2: Vertex2D) -> Vec<Vertex2D> {
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

    fn pll(p1: Vertex2D, p2: Vertex2D) -> Vec<Vertex2D> {
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

        for x in p1.x..p2.x {
            points.push(Vertex2D{x, y});

            if d > 0 {
                y += yi;
                d += 2 * (dy - dx);
            } else {
                d += 2 * dy;
            }
        }

        points
    }

    fn plh(p1: Vertex2D, p2: Vertex2D) -> Vec<Vertex2D> {
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

        for y in p1.y..p2.y {
            points.push(Vertex2D{x, y});

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

    pub fn interp_points(p0: Vertex2D, p1: Vertex2D) -> Vec<Vertex2D> {
        linspace_sample(p0.y, p0.x as f32, p1.y, p1.x as f32)
            .iter()
            .map(|(y,x)| Vertex2D{x: *x as i32, y: *y})
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
            return Vec::from([(x1, y1)]);
        }

        // reorder if need be
        if x1 < x0 {
            mem::swap(&mut x1, &mut x0);
            mem::swap(&mut y1, &mut y0);
        }

        let m = (y1 - y0) / (x1 - x0) as f32;
        let b = y0 - (m * x0 as f32);

        (x0..x1).map(|x| {
            (x, ((m * x as f32) + b))
        }).collect()
    }

    /// Fill a triangle
    pub fn fill(raw_tri: &Triangle2D) -> Vec<HLine> {
        // --
        // Now let's sort the vertices in ascending order
        let tri = {
            let mut tri = raw_tri.clone();

            let y0 = raw_tri.0.0.y;
            let y1 = raw_tri.0.1.y;
            let y2 = raw_tri.0.2.y;

            if y1 < y0 { mem::swap(&mut tri.0.1, &mut tri.0.0); }
            if y2 < y0 { mem::swap(&mut tri.0.2, &mut tri.0.0); }
            if y2 < y1 { mem::swap(&mut tri.0.2, &mut tri.0.1); }

            tri
        };

        let p012 = {
            let mut tmp = interp_points(tri.0.0, tri.0.1);
            let mut tmp_12 = interp_points(tri.0.1, tri.0.2);
            tmp.append(&mut tmp_12);
            tmp
        };

        let p02 = interp_points(tri.0.0, tri.0.2);

        assert_eq!(p012.len(), p02.len());

        // --
        // Now we need to determine the left and right sets of points
        let mut lefts = p012;
        let mut rights = p02;

        let mid = rights.len() / 2;
        if rights[mid].x < lefts[mid].x {
            mem::swap(&mut lefts, &mut rights);
        }

        zip(lefts, rights).map(|p| HLine(p)).collect()
    }

    pub fn fill_shader(raw_tri: &Triangle2D, raw_shader: (f32, f32, f32)) -> Vec<(HLine, LinearIntensity)> {
        // --
        // Now let's sort the vertices/shader in ascending order
        let (tri, shader) = {
            let mut tri = raw_tri.clone();
            let mut shader = raw_shader.clone();

            let y0 = raw_tri.0.0.y;
            let y1 = raw_tri.0.1.y;
            let y2 = raw_tri.0.2.y;

            if y1 < y0 {
                mem::swap(&mut tri.0.1, &mut tri.0.0);
                mem::swap(&mut shader.1, &mut shader.0);
            }
            if y2 < y0 {
                mem::swap(&mut tri.0.2, &mut tri.0.0);
                mem::swap(&mut shader.2, &mut shader.0);
            }
            if y2 < y1 {
                mem::swap(&mut tri.0.2, &mut tri.0.1);
                mem::swap(&mut shader.2, &mut shader.1);
            }

            (tri, shader)
        };

        let p012 = {
            let mut tmp = interp_points(tri.0.0, tri.0.1);
            let mut tmp_12 = interp_points(tri.0.1, tri.0.2);
            tmp.append(&mut tmp_12);
            tmp
        };

        let h012 = {
            let mut tmp = linspace_sample(tri.0.0.y, shader.0, tri.0.1.y, shader.1);
            let mut tmp_12 = linspace_sample(tri.0.1.y, shader.1, tri.0.2.y, shader.2);
            tmp.append(&mut tmp_12);
            tmp
        };

        let p02 = interp_points(tri.0.0, tri.0.2);
        let h02 = linspace_sample(tri.0.0.y, shader.0, tri.0.2.y, shader.2);

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
                HLine(points),
                LinearIntensity{l: intensities.0.1, r: intensities.1.1}
            )
        }).collect()
    }
}

// Construct a new RGB ImageBuffer with the specified width and height.
mod vox {
    use image::{Pixel, Rgb};
    use crate::{grr, Triangle2D, Vertex2D};
    use crate::grr::linspace_sample;

    // render pixel to image
    fn render_pixel(img: &mut image::RgbImage, v: Vertex2D, c: Rgb<u8>) {
        img.put_pixel(v.x as u32, v.y as u32, c);
    }

    fn render_line(img: &mut image::RgbImage, p1: Vertex2D, p2: Vertex2D) {
        for p in grr::line_between(p1, p2) {
            render_pixel(img, p, Rgb([0, 255, 0]));
        }

        // todo: see if we need these...
        render_pixel(img, p1, Rgb([255, 0, 0]));
        render_pixel(img, p2, Rgb([255, 0, 0]));
    }

    fn render_triangle(
        img: &mut image::RgbImage,
        tri: &Triangle2D,
        col: Rgb<u8>
    ) {
        render_triangle_fill(img, tri, col);
        // render_triangle_wireframe(img, tri, col);
    }

    fn render_triangle_shader(
        img: &mut image::RgbImage,
        tri: &Triangle2D,
        col: Rgb<u8>,
        shader: (f32, f32, f32)
    ) {
        render_triangle_fill_shader(img, tri, col, shader);
    }

    fn render_triangle_fill(
        img: &mut image::RgbImage,
        tri: &Triangle2D,
        col: Rgb<u8>
    ) {
        for line in grr::fill(tri) {
            for x in line.0.0.x..line.0.1.x {
                render_pixel(img, Vertex2D{x, y: line.0.0.y}, col)
            }
        }
    }

    fn render_triangle_fill_shader(
        img: &mut image::RgbImage,
        tri: &Triangle2D,
        col: Rgb<u8>,
        shader: (f32, f32, f32)
    ) {
        for (line, intensity) in grr::fill_shader(tri, shader) {
            for (x, h) in linspace_sample(line.0.0.x, intensity.l as f32, line.0.1.x, intensity.r as f32) {

                // Come up with a new color
                let mut c = col.clone();
                c.apply(|chan| (chan as f32 * h) as u8);

                render_pixel(img, Vertex2D{x, y: line.0.0.y}, c);
            }
        }
    }

    fn render_triangle_wireframe(
        img: &mut image::RgbImage,
        tri: &Triangle2D,
        col: Rgb<u8>
    ) {
        [
            grr::line_between(tri.0.0, tri.0.1),
            grr::line_between(tri.0.1, tri.0.2),
            grr::line_between(tri.0.2, tri.0.0),
        ].map( |line| {
            line.iter().for_each(|point| render_pixel(img, *point, col));
        });
    }

    pub fn run() {
        let mut img: image::RgbImage = image::ImageBuffer::new(512, 512);

        let tri = Triangle2D(
            (
                Vertex2D{x: 10, y: 10},
                Vertex2D{x: 400, y: 80},
                Vertex2D{x: 200, y: 300}
            )
        );

        // render_line(&mut img, tri.0.0, tri.0.1);
        // render_line(&mut img, tri.0.1, tri.0.2);
        // render_line(&mut img, tri.0.2, tri.0.0);

        // grr::fill(&tri);

        // render_triangle(&mut img, &tri);
        //
        // grr::interp_points(tri.0.1, tri.0.2).iter().for_each(
        //     |p| {
        //         render_pixel(&mut img, *p, Rgb([252, 3, 244]))
        //     }
        // );
        //
        // grr::interp_points(tri.0.0, tri.0.2).iter().for_each(
        //     |p| {
        //         render_pixel(&mut img, *p, Rgb([252, 3, 244]))
        //     }
        // );

        // render_triangle(&mut img, &tri, Rgb([252, 165, 3]));
        render_triangle_shader(&mut img, &tri, Rgb([252, 165, 3]), (1.0, 1.0, 0.0));

        img.save("fractal.png").unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::vox;

    #[test]
    fn it_works() {
        vox::run();
    }

}
