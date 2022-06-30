use crate::ffi;

/// Error correction mode
#[derive(Clone, Copy)]
#[repr(u32)]
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
#[repr(u32)]
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
#[repr(transparent)]
pub struct ErrorCorrectionConfig {
    raw: ffi::msdfgen_ErrorCorrectionConfig,
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

    /// Get operation mode
    #[inline(always)]
    pub fn get_mode(&self) -> ErrorCorrectionMode {
        assert!(self.raw.mode <= ffi::msdfgen_ErrorCorrectionConfig_Mode_EDGE_ONLY);
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
            self.raw.distanceCheckMode
                <= ffi::msdfgen_ErrorCorrectionConfig_DistanceCheckMode_ALWAYS_CHECK_DISTANCE
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
