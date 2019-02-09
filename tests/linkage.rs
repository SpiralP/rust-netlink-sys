use netlink_sys::*;

#[test]
fn test_netlink_linkage() {
  unsafe {
    println!("{:#?}", nl_ver_num);
  }
}

#[test]
fn test_netlink_genl_linkage() {
  unsafe {
    println!("{:#?}", genl_family_get_version(&mut std::mem::zeroed()));
  }
}

#[test]
fn test_net_if_linkage() {
  unsafe {
    let name = std::ffi::CString::new("wlan0").unwrap();
    println!("{:#?}", if_nametoindex(name.as_ptr()));
  }
}
