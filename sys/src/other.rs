#[cfg(all(not(feature = "generate-bindings"), target_arch = "arm"))]
include!("bindings_arm.rs");

#[cfg(all(not(feature = "generate-bindings"), target_arch = "aarch64"))]
include!("bindings_aarch64.rs");

#[cfg(all(not(feature = "generate-bindings"), target_arch = "mips"))]
include!("bindings_mips.rs");

#[cfg(all(not(feature = "generate-bindings"), target_arch = "mips64"))]
include!("bindings_mips64.rs");

#[cfg(all(not(feature = "generate-bindings"), target_arch = "powerpc"))]
include!("bindings_powerpc.rs");

#[cfg(all(not(feature = "generate-bindings"), target_arch = "powerpc64"))]
include!("bindings_powerpc64.rs");

//#[cfg(all(not(feature = "generate-bindings"), target_arch = "sparc"))]
//include!("bindings_sparc.rs");

#[cfg(all(not(feature = "generate-bindings"), target_arch = "sparc64"))]
include!("bindings_sparc64.rs");

#[cfg(all(not(feature = "generate-bindings"), target_arch = "x86"))]
include!("bindings_x86.rs");

#[cfg(all(not(feature = "generate-bindings"), any(target_arch = "x86_64", feature = "rustdoc")))]
include!("bindings_x86_64.rs");

#[cfg(all(
    not(feature = "generate-bindings"),
    not(target_arch = "x86"),
    not(target_arch = "x86_64"),
    not(target_arch = "arm"),
    not(target_arch = "aarch64"),
    not(target_arch = "mips"),
    not(target_arch = "mips64"),
    not(target_arch = "powerpc"),
    not(target_arch = "powerpc64"),
    //not(target_arch = "sparc"),
    not(target_arch = "sparc64"),
))]
compile_error!("Missing pre-generated bindings for target arch. Try enable 'generate-bindings' feature.");
