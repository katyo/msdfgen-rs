use freetype;
use crate::{Shape, EdgeHolder, EdgeColor, Point2, FontExt};

impl FontExt for freetype::face::Face {
    type Glyph = usize;

    fn glyph_shape(&self, glyph: Self::Glyph) -> Option<Shape> {
        self.load_char(glyph, freetype::face::LoadFlag::NO_SCALE).ok()?;
        let glyph = self.glyph();
        let outline = glyph.outline()?;

        let mut shape = Shape::default();

        for contour in outline.contours_iter() {
            let p = contour.start();
            let last_contour = shape.add_contour_mut();
            let mut last_point = Point2::new(p.x as f64, p.y as f64);

            for curve in contour {
                match curve {
                    freetype::outline::Curve::Line(p) => {
                        let point = Point2::new(p.x as f64, p.y as f64);
                        last_contour.add_edge(&EdgeHolder::new_linear(last_point, point, EdgeColor::default()));
                        last_point = point;
                    },
                    freetype::outline::Curve::Bezier2(c, p) => {
                        let cpoint = Point2::new(c.x as f64, c.y as f64);
                        let point = Point2::new(p.x as f64, p.y as f64);
                        last_contour.add_edge(&EdgeHolder::new_quadratic(last_point, cpoint, point, EdgeColor::default()));
                        last_point = point;
                    },
                    freetype::outline::Curve::Bezier3(c1, c2, p) => {
                        let c1point = Point2::new(c1.x as f64, c1.y as f64);
                        let c2point = Point2::new(c2.x as f64, c2.y as f64);
                        let point = Point2::new(p.x as f64, p.y as f64);
                        last_contour.add_edge(&EdgeHolder::new_cubic(last_point, c1point, c2point, point, EdgeColor::default()));
                        last_point = point;
                    },
                }
            }
        }

        shape.into()
    }
}
