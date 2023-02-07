use crate::ffi;

/// Error correction mode
#[derive(Clone, Copy)]
#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
pub enum ErrorCorrectionMode {
    /// Skips error correction pass
    Disabled = ffi::msdfgen_ErrorCorrectionConfig_Mode_DISABLED,

    /// Corrects all discontinuities of the distance field regardless if edges are adversely affected
    Indiscriminate = ffi::msdfgen_ErrorCorrectionConfig_Mode_INDISCRIMINATE,

    /// Corrects artifacts at edges and other discontinuous distances only if it does not affect edges or corners
    EdgePriority = ffi::msdfgen_ErrorCorrectionConfig_Mode_EDGE_PRIORITY,

    /// Only corrects artifacts at edges
    EdgeOnly = ffi::msdfgen_ErrorCorrectionConfig_Mode_EDGE_ONLY,
}

impl Default for ErrorCorrectionMode {
    fn default() -> Self {
        Self::EdgePriority
    }
}

/// Configuration of whether to use an algorithm that computes the exact shape distance at the positions of suspected artifacts. This algorithm can be much slower
#[derive(Clone, Copy)]
#[cfg_attr(target_os = "windows", repr(i32))]
#[cfg_attr(not(target_os = "windows"), repr(u32))]
pub enum DistanceCheckMode {
    /// Never computes exact shape distance
    DoNotCheck = ffi::msdfgen_ErrorCorrectionConfig_DistanceCheckMode_DO_NOT_CHECK_DISTANCE,

    /// Only computes exact shape distance at edges. Provides a good balance between speed and precision
    CheckAtEdge = ffi::msdfgen_ErrorCorrectionConfig_DistanceCheckMode_CHECK_DISTANCE_AT_EDGE,

    /// Computes and compares the exact shape distance for each suspected artifact
    AlwaysCheck = ffi::msdfgen_ErrorCorrectionConfig_DistanceCheckMode_ALWAYS_CHECK_DISTANCE,
}

impl Default for DistanceCheckMode {
    fn default() -> Self {
        Self::CheckAtEdge
    }
}

/// Error correction config
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ErrorCorrectionConfig {
    raw: ffi::msdfgen_ErrorCorrectionConfig,
}

impl AsRef<ErrorCorrectionConfig> for ErrorCorrectionConfig {
    fn as_ref(&self) -> &ErrorCorrectionConfig {
        self
    }
}

impl AsMut<ErrorCorrectionConfig> for ErrorCorrectionConfig {
    fn as_mut(&mut self) -> &mut ErrorCorrectionConfig {
        self
    }
}

impl Default for ErrorCorrectionConfig {
    fn default() -> Self {
        Self {
            raw: ffi::msdfgen_ErrorCorrectionConfig {
                mode: ErrorCorrectionMode::default() as _,
                distanceCheckMode: DistanceCheckMode::default() as _,
                minDeviationRatio: unsafe {
                    ffi::msdfgen_ErrorCorrectionConfig_defaultMinDeviationRatio
                },
                minImproveRatio: unsafe {
                    ffi::msdfgen_ErrorCorrectionConfig_defaultMinImproveRatio
                },
                buffer: core::ptr::null_mut(),
            },
        }
    }
}

impl ErrorCorrectionConfig {
    pub(crate) fn as_raw(&self) -> &ffi::msdfgen_ErrorCorrectionConfig {
        &self.raw
    }

    pub(crate) fn into_raw(self) -> ffi::msdfgen_ErrorCorrectionConfig {
        self.raw
    }

    /// Get operation mode
    #[inline(always)]
    pub fn get_mode(&self) -> ErrorCorrectionMode {
        assert!((ffi::msdfgen_ErrorCorrectionConfig_Mode_DISABLED
            ..ffi::msdfgen_ErrorCorrectionConfig_Mode_EDGE_ONLY)
            .contains(&self.raw.mode));
        unsafe { core::mem::transmute(self.raw.mode) }
    }

    /// Set operation mode
    #[inline(always)]
    pub fn set_mode(&mut self, mode: ErrorCorrectionMode) {
        self.raw.mode = mode as _;
    }

    /// Configure operation mode
    #[inline(always)]
    pub fn with_mode(mut self, mode: ErrorCorrectionMode) -> Self {
        self.set_mode(mode);
        self
    }

    /// Get distance check mode
    #[inline(always)]
    pub fn get_distance_check_mode(&self) -> DistanceCheckMode {
        assert!(
            (ffi::msdfgen_ErrorCorrectionConfig_DistanceCheckMode_DO_NOT_CHECK_DISTANCE
                ..ffi::msdfgen_ErrorCorrectionConfig_DistanceCheckMode_ALWAYS_CHECK_DISTANCE)
                .contains(&self.raw.distanceCheckMode)
        );
        unsafe { core::mem::transmute(self.raw.distanceCheckMode) }
    }

    /// Set distance check mode
    #[inline(always)]
    pub fn set_distance_check_mode(&mut self, mode: DistanceCheckMode) {
        self.raw.distanceCheckMode = mode as _;
    }

    /// Configure distance check mode
    #[inline(always)]
    pub fn with_distance_check_mode(mut self, mode: DistanceCheckMode) -> Self {
        self.set_distance_check_mode(mode);
        self
    }

    /// Get min deviation ratio
    #[inline(always)]
    pub fn get_min_deviation_ratio(&self) -> f64 {
        self.raw.minDeviationRatio as _
    }

    /// Set min deviation ratio
    #[inline(always)]
    pub fn set_min_deviation_ratio(&mut self, ratio: f64) {
        self.raw.minDeviationRatio = ratio as _;
    }

    /// Configure min deviation ratio
    #[inline(always)]
    pub fn with_min_deviation_ratio(mut self, ratio: f64) -> Self {
        self.set_min_deviation_ratio(ratio);
        self
    }

    /// Get min improve ratio
    #[inline(always)]
    pub fn get_min_improve_ratio(&self) -> f64 {
        self.raw.minImproveRatio as _
    }

    /// Set min improve ratio
    #[inline(always)]
    pub fn set_min_improve_ratio(&mut self, ratio: f64) {
        self.raw.minImproveRatio = ratio as _;
    }

    /// Configure min improve ratio
    #[inline(always)]
    pub fn with_min_improve_ratio(mut self, ratio: f64) -> Self {
        self.set_min_improve_ratio(ratio);
        self
    }
}

/// Base generator config
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct GeneratorConfig {
    raw: ffi::msdfgen_GeneratorConfig,
}

impl AsRef<GeneratorConfig> for GeneratorConfig {
    fn as_ref(&self) -> &GeneratorConfig {
        self
    }
}

impl AsMut<GeneratorConfig> for GeneratorConfig {
    fn as_mut(&mut self) -> &mut GeneratorConfig {
        self
    }
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            raw: ffi::msdfgen_GeneratorConfig {
                overlapSupport: true,
            },
        }
    }
}

impl GeneratorConfig {
    pub(crate) fn as_raw(&self) -> &ffi::msdfgen_GeneratorConfig {
        &self.raw
    }

    pub(crate) fn into_raw(self) -> ffi::msdfgen_GeneratorConfig {
        self.raw
    }

    /// Get overlap support
    #[inline(always)]
    pub fn get_overlap_support(&self) -> bool {
        self.raw.overlapSupport
    }

    /// Set overlap support
    #[inline(always)]
    pub fn set_overlap_support(&mut self, overlap_support: bool) {
        self.raw.overlapSupport = overlap_support;
    }

    /// Configure overlap support
    #[inline(always)]
    pub fn with_overlap_support(mut self, overlap_support: bool) -> Self {
        self.set_overlap_support(overlap_support);
        self
    }
}

/// MSDF generator config
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct MsdfGeneratorConfig {
    raw: ffi::msdfgen_MSDFGeneratorConfig,
}

impl AsRef<MsdfGeneratorConfig> for MsdfGeneratorConfig {
    fn as_ref(&self) -> &MsdfGeneratorConfig {
        self
    }
}

impl AsMut<MsdfGeneratorConfig> for MsdfGeneratorConfig {
    fn as_mut(&mut self) -> &mut MsdfGeneratorConfig {
        self
    }
}

impl AsRef<GeneratorConfig> for MsdfGeneratorConfig {
    fn as_ref(&self) -> &GeneratorConfig {
        unsafe { core::mem::transmute(&self.raw._base) }
    }
}

impl AsMut<GeneratorConfig> for MsdfGeneratorConfig {
    fn as_mut(&mut self) -> &mut GeneratorConfig {
        unsafe { core::mem::transmute(&mut self.raw._base) }
    }
}

impl AsRef<ErrorCorrectionConfig> for MsdfGeneratorConfig {
    fn as_ref(&self) -> &ErrorCorrectionConfig {
        unsafe { core::mem::transmute(&self.raw.errorCorrection) }
    }
}

impl AsMut<ErrorCorrectionConfig> for MsdfGeneratorConfig {
    fn as_mut(&mut self) -> &mut ErrorCorrectionConfig {
        unsafe { core::mem::transmute(&mut self.raw.errorCorrection) }
    }
}

impl core::ops::Deref for MsdfGeneratorConfig {
    type Target = ErrorCorrectionConfig;

    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(&self.raw.errorCorrection) }
    }
}

impl core::ops::DerefMut for MsdfGeneratorConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::mem::transmute(&mut self.raw.errorCorrection) }
    }
}

impl Default for MsdfGeneratorConfig {
    fn default() -> Self {
        Self {
            raw: ffi::msdfgen_MSDFGeneratorConfig {
                _base: GeneratorConfig::default().into_raw(),
                errorCorrection: ErrorCorrectionConfig::default().into_raw(),
            },
        }
    }
}

impl MsdfGeneratorConfig {
    pub(crate) fn as_raw(&self) -> &ffi::msdfgen_MSDFGeneratorConfig {
        &self.raw
    }

    /*pub(crate) fn into_raw(self) -> ffi::msdfgen_MSDFGeneratorConfig {
        self.raw
    }*/

    /// Get overlap support
    #[inline(always)]
    pub fn get_overlap_support(&self) -> bool {
        (self.as_ref() as &GeneratorConfig).get_overlap_support()
    }

    /// Set overlap support
    #[inline(always)]
    pub fn set_overlap_support(&mut self, overlap_support: bool) {
        (self.as_mut() as &mut GeneratorConfig).set_overlap_support(overlap_support);
    }

    /// Configure overlap support
    #[inline(always)]
    pub fn with_overlap_support(mut self, overlap_support: bool) -> Self {
        self.set_overlap_support(overlap_support);
        self
    }

    /// Configure operation mode
    #[inline(always)]
    pub fn with_mode(mut self, mode: ErrorCorrectionMode) -> Self {
        self.set_mode(mode);
        self
    }

    /// Configure distance check mode
    #[inline(always)]
    pub fn with_distance_check_mode(mut self, mode: DistanceCheckMode) -> Self {
        self.set_distance_check_mode(mode);
        self
    }

    /// Configure min deviation ratio
    #[inline(always)]
    pub fn with_min_deviation_ratio(mut self, ratio: f64) -> Self {
        self.set_min_deviation_ratio(ratio);
        self
    }

    /// Configure min improve ratio
    #[inline(always)]
    pub fn with_min_improve_ratio(mut self, ratio: f64) -> Self {
        self.set_min_improve_ratio(ratio);
        self
    }
}
