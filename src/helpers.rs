use super::*;
use std::ffi::CString;

pub fn interface_name_to_index(interface_name: &str) -> Option<u32> {
  let interface_index = {
    let name = CString::new(interface_name).unwrap();

    unsafe { if_nametoindex(name.as_ptr()) }
  };
  if interface_index == 0 {
    None
  } else {
    Some(interface_index)
  }
}
