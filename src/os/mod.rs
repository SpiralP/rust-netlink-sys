#[cfg(target_os = "linux")]
mod linux {
  pub mod genl;
  pub mod netlink;
}

#[cfg(target_os = "linux")]
pub use self::linux::*;

#[cfg_attr(target_os = "linux", link(name = "nl-3"))]
extern "C" {}
