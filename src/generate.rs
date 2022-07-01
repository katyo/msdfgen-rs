use crate::{
    ffi, Bitmap, ErrorCorrectionConfig, GeneratorConfig, Gray, MsdfGeneratorConfig, Rgb, Rgba,
    Shape, Vector2,
};

/// Framing options
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Framing<T> {
    pub projection: Projection<T>,
    pub range: T,
}

impl<T> AsRef<Framing<T>> for Framing<T> {
    fn as_ref(&self) -> &Framing<T> {
        self
    }
}

impl<T> AsMut<Framing<T>> for Framing<T> {
    fn as_mut(&mut self) -> &mut Framing<T> {
        self
    }
}

impl<T> AsRef<Projection<T>> for Framing<T> {
    fn as_ref(&self) -> &Projection<T> {
        &self.projection
    }
}

impl<T> AsMut<Projection<T>> for Framing<T> {
    fn as_mut(&mut self) -> &mut Projection<T> {
        &mut self.projection
    }
}

impl<T> core::ops::Deref for Framing<T> {
    type Target = Projection<T>;

    fn deref(&self) -> &Self::Target {
        &self.projection
    }
}

impl<T> core::ops::DerefMut for Framing<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.projection
    }
}

impl<T> Framing<T> {
    pub fn new(
        range: impl Into<T>,
        scale: impl Into<Vector2<T>>,
        translate: impl Into<Vector2<T>>,
    ) -> Self {
        Self {
            range: range.into(),
            projection: Projection::new(scale, translate),
        }
    }
}

/// Projection
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Projection<T> {
    pub scale: Vector2<T>,
    pub translate: Vector2<T>,
}

impl<T> Projection<T> {
    pub fn new(scale: impl Into<Vector2<T>>, translate: impl Into<Vector2<T>>) -> Self {
        Self {
            scale: scale.into(),
            translate: translate.into(),
        }
    }
}

impl Projection<f64> {
    /*pub(crate) fn into_raw(self) -> ffi::msdfgen_Projection {
        unsafe { core::mem::transmute(self) }
    }*/

    pub(crate) fn as_raw(&self) -> &ffi::msdfgen_Projection {
        unsafe { core::mem::transmute(self) }
    }
}

impl Shape {
    /// Generate signed distance field for shape
    pub fn generate_sdf(
        &self,
        mut output: impl AsMut<Bitmap<Gray<f32>>>,
        framing: impl AsRef<Framing<f64>>,
        config: impl AsRef<GeneratorConfig>,
    ) {
        let framing = framing.as_ref();
        unsafe {
            ffi::msdfgen_generateSDF(
                output.as_mut().as_raw_mut(),
                self.as_raw(),
                framing.projection.as_raw(),
                framing.range,
                config.as_ref().as_raw(),
            )
        }
    }

    /// Generate pseudo signed distance field for shape
    pub fn generate_pseudo_sdf(
        &self,
        mut output: impl AsMut<Bitmap<Gray<f32>>>,
        framing: impl AsRef<Framing<f64>>,
        config: impl AsRef<GeneratorConfig>,
    ) {
        let framing = framing.as_ref();
        unsafe {
            ffi::msdfgen_generatePseudoSDF(
                output.as_mut().as_raw_mut(),
                self.as_raw(),
                framing.projection.as_raw(),
                framing.range,
                config.as_ref().as_raw(),
            )
        }
    }

    /// Generate signed distance field for shape (legacy)
    pub fn generate_sdf_legacy(
        &self,
        mut output: impl AsMut<Bitmap<Gray<f32>>>,
        framing: impl AsRef<Framing<f64>>,
    ) {
        let framing = framing.as_ref();
        unsafe {
            ffi::msdfgen_generateSDF_legacy(
                output.as_mut().as_raw_mut(),
                self.as_raw(),
                framing.range,
                framing.scale.as_raw(),
                framing.translate.as_raw(),
            )
        }
    }

    /// Generate pseudo signed distance field for shape (legacy)
    pub fn generate_pseudo_sdf_legacy(
        &self,
        mut output: impl AsMut<Bitmap<Gray<f32>>>,
        framing: impl AsRef<Framing<f64>>,
    ) {
        let framing = framing.as_ref();
        unsafe {
            ffi::msdfgen_generatePseudoSDF_legacy(
                output.as_mut().as_raw_mut(),
                self.as_raw(),
                framing.range,
                framing.scale.as_raw(),
                framing.translate.as_raw(),
            )
        }
    }

    /// Generate multi-channel signed distance field for shape
    pub fn generate_msdf(
        &self,
        mut output: impl AsMut<Bitmap<Rgb<f32>>>,
        framing: impl AsRef<Framing<f64>>,
        config: impl AsRef<MsdfGeneratorConfig>,
    ) {
        let framing = framing.as_ref();
        unsafe {
            ffi::msdfgen_generateMSDF(
                output.as_mut().as_raw_mut(),
                self.as_raw(),
                framing.projection.as_raw(),
                framing.range,
                config.as_ref().as_raw(),
            )
        }
    }

    /// Generate multi-channel signed distance field for shape (legacy)
    pub fn generate_msdf_legacy(
        &self,
        mut output: impl AsMut<Bitmap<Rgb<f32>>>,
        framing: impl AsRef<Framing<f64>>,
        config: impl AsRef<ErrorCorrectionConfig>,
    ) {
        let framing = framing.as_ref();
        unsafe {
            ffi::msdfgen_generateMSDF_legacy(
                output.as_mut().as_raw_mut(),
                self.as_raw(),
                framing.range,
                framing.scale.as_raw(),
                framing.translate.as_raw(),
                *config.as_ref().as_raw(),
            )
        }
    }

    /// Generate multi-channel signed distance field for shape with true distance in the alpha channel
    pub fn generate_mtsdf(
        &self,
        mut output: impl AsMut<Bitmap<Rgba<f32>>>,
        framing: impl AsRef<Framing<f64>>,
        config: impl AsRef<MsdfGeneratorConfig>,
    ) {
        let framing = framing.as_ref();
        unsafe {
            ffi::msdfgen_generateMTSDF(
                output.as_mut().as_raw_mut(),
                self.as_raw(),
                framing.projection.as_raw(),
                framing.range,
                config.as_ref().as_raw(),
            )
        }
    }

    /// Generate multi-channel signed distance field for shape with true distance in the alpha channel (legacy)
    pub fn generate_mtsdf_legacy(
        &self,
        mut output: impl AsMut<Bitmap<Rgba<f32>>>,
        framing: impl AsRef<Framing<f64>>,
        config: impl AsRef<ErrorCorrectionConfig>,
    ) {
        let framing = framing.as_ref();
        unsafe {
            ffi::msdfgen_generateMTSDF_legacy(
                output.as_mut().as_raw_mut(),
                self.as_raw(),
                framing.range,
                framing.scale.as_raw(),
                framing.translate.as_raw(),
                *config.as_ref().as_raw(),
            )
        }
    }
}
