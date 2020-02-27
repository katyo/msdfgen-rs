#[cfg(target_arch = "x86")]
include!("darwin/x86.rs");

#[cfg(target_arch = "x86_64")]
include!("darwin/x86_64.rs");
