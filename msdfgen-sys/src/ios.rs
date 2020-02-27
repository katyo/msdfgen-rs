#[cfg(target_arch = "armv7")]
include!("ios/armv7.rs");

#[cfg(target_arch = "aarch64")]
include!("ios/aarch64.rs");

#[cfg(target_arch = "x86")]
include!("ios/x86.rs");

#[cfg(target_arch = "x86_64")]
include!("ios/x86_64.rs");
