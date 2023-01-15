use crate::{Contour, EdgeColor, EdgeHolder, FontExt, Point2, Shape};

#[derive(Default)]
struct ContourBuilder {
    contour: Contour,
    point: Point2<f64>,
}

impl ContourBuilder {
    pub fn open_at(x: f64, y: f64) -> Self {
        Self {
            contour: Contour::default(),
            point: Point2::new(x, y),
        }
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        let point = Point2::new(x, y);
        self.contour.add_edge(&EdgeHolder::new_linear(
            self.point,
            point,
            EdgeColor::default(),
        ));
        self.point = point;
    }

    pub fn quad_to(&mut self, cx: f64, cy: f64, x: f64, y: f64) {
        let cpoint = Point2::new(cx, cy);
        let point = Point2::new(x, y);
        self.contour.add_edge(&EdgeHolder::new_quadratic(
            self.point,
            cpoint,
            point,
            EdgeColor::default(),
        ));
        self.point = point;
    }

    pub fn curve_to(&mut self, c1x: f64, c1y: f64, c2x: f64, c2y: f64, x: f64, y: f64) {
        let c1point = Point2::new(c1x, c1y);
        let c2point = Point2::new(c2x, c2y);
        let point = Point2::new(x, y);
        self.contour.add_edge(&EdgeHolder::new_cubic(
            self.point,
            c1point,
            c2point,
            point,
            EdgeColor::default(),
        ));
        self.point = point;
    }

    pub fn close(self) -> Contour {
        self.contour
    }
}

#[derive(Default)]
struct ShapeBuilder {
    shape: Shape,
    contour: Option<ContourBuilder>,
}

impl ShapeBuilder {
    pub fn build(self) -> Shape {
        self.shape
    }
}

impl ttf_parser::OutlineBuilder for ShapeBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        if self.contour.is_some() {
            panic!("Unexpected move_to");
        }

        self.contour = ContourBuilder::open_at(x as _, y as _).into();
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.contour
            .as_mut()
            .expect("Opened contour")
            .line_to(x as _, y as _);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.contour
            .as_mut()
            .expect("Opened contour")
            .quad_to(x1 as _, y1 as _, x as _, y as _);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.contour
            .as_mut()
            .expect("Opened contour")
            .curve_to(x1 as _, y1 as _, x2 as _, y2 as _, x as _, y as _);
    }

    fn close(&mut self) {
        self.shape
            .add_contour(&self.contour.take().expect("Opened contour").close());
    }
}

impl<'a> FontExt for ttf_parser::Face<'a> {
    type Glyph = ttf_parser::GlyphId;

    fn glyph_shape(&self, glyph: Self::Glyph) -> Option<Shape> {
        let mut builder = ShapeBuilder::default();

        self.outline_glyph(glyph, &mut builder)?;

        Some(builder.build())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use notosans::REGULAR_TTF;
    use ttf_parser::{Face, GlyphId};

    #[test]
    fn glyph_shape() {
        let font = Face::parse(REGULAR_TTF, 0).unwrap();

        let mut shapes = 0;

        for glyph in 0..font.number_of_glyphs() {
            let glyph = GlyphId(glyph);
            if let Some(_shape) = font.glyph_shape(glyph) {
                shapes += 1;
            }
        }

        assert_eq!(shapes, 2392);
    }
}
