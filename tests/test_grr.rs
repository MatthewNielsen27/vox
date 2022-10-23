use vox::raster::{Pixel, ScanlineH, Triangle2D};
use vox::grr;

/// returns true if the scanlines are monotonically increasing
fn validate_scanline_monotonicity(items: &[ScanlineH]) -> bool {
    let mut prev = items[0];
    for next in items {
        assert_eq!(next.l.y, next.r.y);
        // todo: this could really be, assert!(y_prev + 1 == y_current)
        assert!(prev.l.y <= next.l.y);
        assert!(prev.r.y <= next.r.y);
        prev = *next;
    }
    true
}

#[test]
fn test_raster_fill() {
    // [Scenario] an isosceles triangle (pointing upwards)
    {
        let tri = Triangle2D::from_points(&[(20, 20), (10, 40), (30, 40)]);
        let scanlines = grr::scanlines(&tri);

        assert_eq!(scanlines.len(), 21);

        // Validate the start and end of the collection since these are known values
        assert_eq!(scanlines[0].l, Pixel{x: 20, y: 20});
        assert_eq!(scanlines[0].r, Pixel{x: 20, y: 20});
        assert_eq!(scanlines[scanlines.len()-1].l, Pixel{x: 10, y: 40});
        assert_eq!(scanlines[scanlines.len()-1].r, Pixel{x: 30, y: 40});

        // Validate that the scanlines increase monotonically
        assert!(validate_scanline_monotonicity(&scanlines[..]));
    }

    // [Scenario] an isosceles triangle (pointing downwards)
    {
        let tri = Triangle2D::from_points(&[(10, 20), (30, 20), (20, 40)]);
        let scanlines = grr::scanlines(&tri);

        assert_eq!(scanlines.len(), 21);

        // Validate the start and end of the collection since these are known values
        assert_eq!(scanlines[0].l, Pixel{x: 10, y: 20});
        assert_eq!(scanlines[0].r, Pixel{x: 30, y: 20});
        assert_eq!(scanlines[scanlines.len()-1].l, Pixel{x: 20, y: 40});
        assert_eq!(scanlines[scanlines.len()-1].r, Pixel{x: 20, y: 40});

        // Validate that the scanlines increase monotonically
        assert!(validate_scanline_monotonicity(&scanlines[..]));
    }

    // [Scenario] a scalene triangle, off-axis
    {
        let tri = Triangle2D::from_points(&[(10, 20), (27, 32), (14, 80)]);
        let scanlines = grr::scanlines(&tri);

        assert_eq!(scanlines.len(), 61);

        // Validate the start and end of the collection since these are known values
        //
        // In this case, the scanlines should start and end at the same points
        assert_eq!(scanlines[0].l, scanlines[0].r);
        assert_eq!(scanlines[0].l, Pixel{x: 10, y: 20});
        assert_eq!(scanlines[scanlines.len()-1].l, scanlines[scanlines.len()-1].r);
        assert_eq!(scanlines[scanlines.len()-1].r, Pixel{x: 14, y: 80});

        // Validate that the scanlines increase monotonically
        assert!(validate_scanline_monotonicity(&scanlines[..]));
    }
}

#[test]
fn test_scanlines_with_attributes() {
    // [Scenario] an isosceles triangle (pointing upwards)
    {
        let tri = Triangle2D::from_points(&[(20, 20), (10, 40), (30, 40)]);
        let attr = [1.0, 1.0, 1.0];
        let scanlines = grr::scanlines_with_attributes(&tri, &attr);

        assert_eq!(scanlines.len(), 21);

        // Validate the start and end of the collection since these are known values
        assert_eq!(scanlines[0].0.l, Pixel{x: 20, y: 20});
        assert_eq!(scanlines[0].0.r, Pixel{x: 20, y: 20});
        assert_eq!(scanlines[scanlines.len()-1].0.l, Pixel{x: 10, y: 40});
        assert_eq!(scanlines[scanlines.len()-1].0.r, Pixel{x: 30, y: 40});

        // Validate that the attributes were correctly calculated
        assert_eq!(scanlines[0].1.0, 1.0);
        assert_eq!(scanlines[0].1.1, 1.0);
        assert_eq!(scanlines[scanlines.len()-1].1.0, 1.0);
        assert_eq!(scanlines[scanlines.len()-1].1.1, 1.0);

        // Validate that the scanlines increase monotonically
        let scanlines : Vec<ScanlineH> = scanlines.into_iter().map(|(s, _)| s).collect();
        assert!(validate_scanline_monotonicity(&scanlines[..]));
    }

    // [Scenario] an isosceles triangle (pointing downwards)
    {
        let tri = Triangle2D::from_points(&[(10, 20), (30, 20), (20, 40)]);
        let scanlines = grr::scanlines(&tri);

        assert_eq!(scanlines.len(), 21);

        // Validate the start and end of the collection since these are known values
        assert_eq!(scanlines[0].l, Pixel{x: 10, y: 20});
        assert_eq!(scanlines[0].r, Pixel{x: 30, y: 20});
        assert_eq!(scanlines[scanlines.len()-1].l, Pixel{x: 20, y: 40});
        assert_eq!(scanlines[scanlines.len()-1].r, Pixel{x: 20, y: 40});

        // Validate that the scanlines increase monotonically
        assert!(validate_scanline_monotonicity(&scanlines[..]));
    }

    // [Scenario] a scalene triangle, off-axis
    {
        let tri = Triangle2D::from_points(&[(10, 20), (27, 32), (14, 80)]);
        let scanlines = grr::scanlines(&tri);

        assert_eq!(scanlines.len(), 61);

        // Validate the start and end of the collection since these are known values
        //
        // In this case, the scanlines should start and end at the same points
        assert_eq!(scanlines[0].l, scanlines[0].r);
        assert_eq!(scanlines[0].l, Pixel{x: 10, y: 20});
        assert_eq!(scanlines[scanlines.len()-1].l, scanlines[scanlines.len()-1].r);
        assert_eq!(scanlines[scanlines.len()-1].r, Pixel{x: 14, y: 80});

        // Validate that the scanlines increase monotonically
        assert!(validate_scanline_monotonicity(&scanlines[..]));
    }
}
