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
  pub fn build_bindings() {
    bindgen::builder()
      .raw_line("#![allow(non_snake_case)]")
      .raw_line("#![allow(non_camel_case_types)]")
      .raw_line("#![allow(non_upper_case_globals)]")
      .raw_line("#![allow(clippy::const_static_lifetime)]")
      .raw_line("#![allow(clippy::unreadable_literal)]")
      .raw_line("#![allow(clippy::cyclomatic_complexity)]")
      .raw_line("#![allow(clippy::useless_transmute)]")
      .whitelist_var("LIBNL.*")
      .whitelist_var("NETLINK.*")
      .whitelist_var("NL.*")
      .whitelist_var("nl.*")
      .whitelist_function("nl.*")
      .whitelist_type("nl.*")
      .whitelist_function("genl.*")
      .whitelist_type("genl.*")
      .whitelist_function("if_nametoindex")
      .header_contents(
        "input.h",
        "
        #include <netlink/netlink.h>
        #include <netlink/msg.h>
        #include <netlink/attr.h>
        #include <netlink/genl/genl.h>
        #include <netlink/genl/family.h>
        #include <netlink/genl/ctrl.h>
        #include <net/if.h>
        #include <linux/nl80211.h>
        ",
      )
      .clang_arg("-I/usr/include/libnl3")
      .clang_arg("-I/usr/lib/gcc/x86_64-linux-gnu/8/include")
      .generate()
      .unwrap()
      .write_to_file(&format!("./src/os/{}.rs", OS))
      .unwrap();

    // genl_ctrl_resolve
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
