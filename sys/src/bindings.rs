#[cfg(target_os = "macos")]
include!("darwin.rs");

#[cfg(target_os = "ios")]
include!("ios.rs");

#[cfg(not(target_vendor = "apple"))]
include!("other.rs");
