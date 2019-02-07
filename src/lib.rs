mod os;

pub use self::os::*;

#[test]
fn test_linkage() {
  unsafe {
    println!("{:#?}", nl_ver_num);
  }
}
