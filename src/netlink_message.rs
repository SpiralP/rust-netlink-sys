use super::*;
use std::ffi::c_void;

pub struct NetlinkMessage {
  pub ptr: *mut nl_msg,
}
impl NetlinkMessage {
  pub fn alloc() -> Result<Self, ()> {
    let ptr = unsafe { nlmsg_alloc() };
    if ptr.is_null() {
      Err(())
    } else {
      Ok(Self { ptr })
    }
  }

  pub fn genlmsg_put(&self, nl80211_id: i32, kind: u32) -> Result<*mut c_void, ()> {
    let ptr = unsafe { genlmsg_put(self.ptr, 0, 0, nl80211_id, 0, 0, kind as u8, 0) };
    if ptr.is_null() {
      Err(())
    } else {
      Ok(ptr)
    }
  }

  pub fn nla_put_u32(&self, attrtype: u32, value: u32) -> Result<(), i32> {
    let attrlen = 8;
    let data = &value as *const _ as *const c_void;
    let ret = unsafe { nla_put(self.ptr, attrtype as i32, attrlen, data) };
    if ret == 0 {
      Ok(())
    } else {
      Err(ret)
    }
  }
}

impl Drop for NetlinkMessage {
  fn drop(&mut self) {
    unsafe {
      nlmsg_free(self.ptr);
    }
  }
}
