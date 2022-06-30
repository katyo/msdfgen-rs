#include <msdfgen.h>

namespace msdfgen {
  Contour Contour_constructor() {
    return Contour();
  }

  void Contour_destructor(Contour &self) {
    self.~Contour();
  }

  void Shape_destructor(Shape &self) {
    self.~Shape();
  }

  void Scanline_destructor(Scanline &self) {
    self.~Scanline();
  }

  void EdgeHolder_setSegment(EdgeHolder &self, const EdgeSegment &newSegment) {
    EdgeSegment* segment = self;
    if (segment != nullptr) {
      delete segment;
    }
    *self = newSegment;
  }

  enum SegmentKind {
    LINEAR,
    QUADRATIC,
    CUBIC,
  };

  SegmentKind EdgeSegment_getKind(const EdgeSegment &self) {
    return
      dynamic_cast<const LinearSegment*>(&self) != nullptr ? LINEAR :
      dynamic_cast<const QuadraticSegment*>(&self) != nullptr ? QUADRATIC :
      CUBIC;
  }
}
