namespace msdfgen {
  Contour Contour_constructor();

  void Contour_destructor(Contour &self);

  void Shape_destructor(Shape &self);

  void Scanline_destructor(Scanline &self);

  void EdgeHolder_setSegment(EdgeHolder &self, const EdgeSegment &segment);

  enum SegmentKind {
    LINEAR,
    QUADRATIC,
    CUBIC,
  };

  SegmentKind EdgeSegment_getKind(const EdgeSegment &self);
}
