use crate::{EdgeColor, EdgeHolder, FontExt, Point2, Shape};

impl FontExt for font::Font {
    type Glyph = char;

    fn glyph_shape(&self, glyph: Self::Glyph) -> Option<Shape> {
        let glyph = self.draw(glyph).ok()??;
        let mut shape = Shape::default();

        for contour in glyph.iter() {
            let last_contour = shape.add_contour_mut();
            let font::Offset(x, y) = contour.offset;
            let mut last_point = Point2::new(x as f64, y as f64);

            for segment in contour.iter() {
                match *segment {
                    font::Segment::Linear(font::Offset(x, y)) => {
                        let point = Point2::new(x as f64, y as f64);
                        last_contour.add_edge(&EdgeHolder::new_linear(
                            last_point,
                            point,
                            EdgeColor::default(),
                        ));
                        last_point = point;
                    }
                    font::Segment::Quadratic(font::Offset(cx, cy), font::Offset(x, y)) => {
                        let cpoint = Point2::new(cx as f64, cy as f64);
                        let point = Point2::new(x as f64, y as f64);
                        last_contour.add_edge(&EdgeHolder::new_quadratic(
                            last_point,
                            cpoint,
                            point,
                            EdgeColor::default(),
                        ));
                        last_point = point;
                    }
                    font::Segment::Cubic(
                        font::Offset(c1x, c1y),
                        font::Offset(c2x, c2y),
                        font::Offset(x, y),
                    ) => {
                        let c1point = Point2::new(c1x as f64, c1y as f64);
                        let c2point = Point2::new(c2x as f64, c2y as f64);
                        let point = Point2::new(x as f64, y as f64);
                        last_contour.add_edge(&EdgeHolder::new_cubic(
                            last_point,
                            c1point,
                            c2point,
                            point,
                            EdgeColor::default(),
                        ));
                        last_point = point;
                    }
                }
            }
        }

        shape.into()
    }
}

// #[cfg(test)]
// mod test {
//     use font::Font;
//     use notosans::REGULAR_TTF;
//     use super::*;

//     #[test]
//     fn glyph_shape() {
//         let font = Font::read(&mut std::io::Cursor::new(&REGULAR_TTF)).unwrap();

//         let mut shapes = 0;

//         for glyph in "abcdefghABCDEFGH".chars() {
//             if let Some(_shape) = font.glyph_shape(glyph) {
//                 shapes += 1;
//             }
//         }

//         assert_eq!(shapes, 16);
//     }
// }
