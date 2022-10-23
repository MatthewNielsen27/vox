use std::mem;

#[derive(Copy, Clone)]
pub struct LinearIntensity {
    pub l: f32,
    pub r: f32
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Pixel {
    pub x: i32,
    pub y: i32
}

#[derive(Copy, Clone)]
pub struct Triangle2D {
    pub points: [Pixel; 3]
}

#[derive(Copy, Clone)]
pub struct ScanlineH {
    pub l: Pixel,
    pub r: Pixel
}

pub fn interp_pixels(p0: Pixel, p1: Pixel) -> Vec<Pixel> {
    linspace_sample(p0.y, p0.x as f32, p1.y, p1.x as f32)
        .iter()
        .map(|(y,x)| Pixel{x: *x as i32, y: *y})
        .collect()
}

/// [returns] the sequence of points (X,Y) between 2 X values, with Y values linearly interpolated.
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

impl Triangle2D {
    pub fn from_points(points: &[(i32, i32); 3]) -> Self {
        Triangle2D {
            points: [
                Pixel {x: points[0].0, y: points[0].1},
                Pixel {x: points[1].0, y: points[1].1},
                Pixel {x: points[2].0, y: points[2].1}
            ]
        }
    }

    /// [returns] this triangles indices sorted by Y value (ascending)
    pub fn get_indices_sorted(&self) -> (usize, usize, usize) {
        let mut i0 = 0;
        let mut i1 = 1;
        let mut i2 = 2;

        if self.points[i1].y > self.points[i0].y { mem::swap(&mut i1, &mut i0); }
        if self.points[i2].y > self.points[i0].y { mem::swap(&mut i2, &mut i0); }
        if self.points[i2].y > self.points[i1].y { mem::swap(&mut i2, &mut i1); }

        (i0, i1, i2)
    }

    /// [returns] the (left, right) sides of the triangle.
    pub fn get_sides(&self) -> (Vec<Pixel>, Vec<Pixel>) {
        let (mut lefts,  _) = self.get_012(None);
        let (mut rights, _) = self.get_02(None);

        let mid = rights.len() / 2;
        if rights[mid].x < lefts[mid].x {
            mem::swap(&mut lefts, &mut rights);
        }

        (lefts, rights)
    }

    pub fn get_sides_with_attr(&self, attr: &(f32, f32, f32)) -> ((Vec<Pixel>, Vec<Pixel>), (Vec<f32>, Vec<f32>)) {
        let (mut l, l_attr) = self.get_012(Some(attr));
        let (mut r, r_attr) = self.get_02(Some(attr));

        let mut l_attr = l_attr.unwrap();
        let mut r_attr = r_attr.unwrap();

        let mid = l.len() / 2;
        if r[mid].x < l[mid].x {
            mem::swap(&mut l, &mut r);
            mem::swap(&mut l_attr, &mut r_attr);
        }

        ((l, r), (l_attr, r_attr))
    }

    /// [returns] the points along the lines 0 -> 1 -> 2, mapping attributes as well (if provided).
    fn get_012(&self, attr: Option<&(f32, f32, f32)>) -> (Vec<Pixel>, Option<Vec<f32>>) {
        assert!(self.points[0].y <= self.points[1].y);
        assert!(self.points[1].y <= self.points[2].y);

        // Step 1: map the points along this side.
        let buf_ps = {
            // todo: see if we can make this a single allocation...
            let mut tmp = interp_pixels(self.points[0], self.points[1]);
            tmp.pop(); // This is because tmp[-1] == tmp_12[0]
            let mut tmp_12 = interp_pixels(self.points[1], self.points[2]);
            tmp.append(&mut tmp_12);
            tmp
        };

        // Step 2: map attributes for the side (if provided).
        let buf_as = match attr {
            None => None,
            Some(&(a0, a1, a2)) => {
                // todo: see if we can make this a single allocation...
                let mut tmp = linspace_sample(self.points[0].y, a0, self.points[1].y, a1);
                tmp.pop(); // This is because tmp[-1] == tmp_12[0]
                let mut tmp_12 = linspace_sample(self.points[1].y, a1, self.points[2].y, a2);
                tmp.append(&mut tmp_12);
                Some(tmp.into_iter().map(|(_,a)| a).collect())
            }
        };

        (buf_ps, buf_as)
    }

    /// [returns] the points along the line 0 -> 2, mapping attributes as well (if provided).
    fn get_02(&self, attr: Option<&(f32, f32, f32)>) -> (Vec<Pixel>, Option<Vec<f32>>) {
        assert!(self.points[0].y <= self.points[1].y);
        assert!(self.points[1].y <= self.points[2].y);

        // Step 1: map the points along this side.
        let buf_ps = interp_pixels(self.points[0], self.points[2]);

        // Step 2: map attributes for the side (if provided).
        let buf_as = match attr {
            None => None,
            Some(&(a0, _, a2)) => {
                Some(
                    linspace_sample(self.points[0].y, a0, self.points[2].y, a2)
                        .iter()
                        .map(|(_, y)| *y)
                        .collect::<Vec<f32>>()
                )
            }
        };

        (buf_ps, buf_as)
    }
}
