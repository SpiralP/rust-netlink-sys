mod os;

pub use self::os::*;

#[test]
fn test_linkage() {
  unsafe {
    println!("{:#?}", netlink::nl_ver_num);
  }
}
