/*
libnl-3-dev
/lib/x86_64-linux-gnu/libnl-3.so
libnl-genl-3-dev
/lib/x86_64-linux-gnu/libnl-genl-3.so
*/

#[cfg(feature = "bindgen")]
mod builder {
  #[cfg(target_os = "linux")]
  pub const OS: &str = "linux";

  use bindgen;
  fn make_bindings(path: &str) -> bindgen::Bindings {
    let bindings = bindgen::builder()
      .raw_line("#![allow(non_snake_case)]")
      .raw_line("#![allow(non_camel_case_types)]")
      .raw_line("#![allow(non_upper_case_globals)]")
      .raw_line("#![allow(clippy::const_static_lifetime)]")
      .raw_line("#![allow(clippy::unreadable_literal)]")
      .raw_line("#![allow(clippy::cyclomatic_complexity)]")
      .whitelist_var("LIBNL.*")
      .whitelist_var("NETLINK.*")
      .whitelist_var("NL.*")
      .whitelist_var("nl.*")
      .whitelist_function("nl.*")
      .whitelist_type("nl.*")
      .whitelist_function("genl.*")
      .whitelist_type("genl.*")
      .header(path)
      .clang_arg("-I/usr/include/libnl3")
      .clang_arg("-I/usr/lib/gcc/x86_64-linux-gnu/8/include");

    bindings.generate().unwrap()
  }

  pub fn build_bindings() {
    std::fs::create_dir_all(&format!("./src/os/{}/", OS)).unwrap();

    make_bindings("/usr/include/libnl3/netlink/netlink.h")
      .write_to_file(&format!("./src/os/{}/netlink.rs", OS))
      .unwrap();

    make_bindings("/usr/include/libnl3/netlink/genl/genl.h")
      .write_to_file(&format!("./src/os/{}/genl.rs", OS))
      .unwrap();
  }
}

fn main() {
  // use std::env;
  // if let Ok(libdir) = env::var("NETLINK_LIBDIR") {
  //   println!("cargo:rustc-link-search=native={}", libdir);
  // }

  #[cfg(feature = "bindgen")]
  self::builder::build_bindings();
}
