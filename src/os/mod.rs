#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use self::linux::*;

#[cfg_attr(target_os = "linux", link(name = "nl-3"))]
extern "C" {}
