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
}
