use super::*;
use std::ffi::CString;

pub struct NetlinkSocket {
  pub ptr: *mut nl_sock,
}

impl NetlinkSocket {
  pub fn alloc() -> Result<Self, ()> {
    let ptr = unsafe { nl_socket_alloc() };
    if ptr.is_null() {
      Err(())
    } else {
      Ok(Self { ptr })
    }
  }

  pub fn genl_connect(&self) -> Result<(), i32> {
    let ret = unsafe { genl_connect(self.ptr) };
    if ret == 0 {
      Ok(())
    } else {
      Err(ret)
    }
  }

  pub fn genl_ctrl_resolve(&self, name: &str) -> Result<i32, i32> {
    let nl80211_id = {
      let s = CString::new(name).unwrap();
      unsafe { genl_ctrl_resolve(self.ptr, s.as_ptr()) }
    };
    if nl80211_id < 0 {
      Err(nl80211_id)
    } else {
      Ok(nl80211_id)
    }
  }

  pub fn nl_send_auto(&self, netlink_message: &NetlinkMessage) -> Result<i32, i32> {
    let err = unsafe { nl_send_auto(self.ptr, netlink_message.ptr) };
    if err < 0 {
      Err(err)
    } else {
      Ok(err)
    }
  }

  pub fn nl_recvmsgs(&self, netlink_callback: &NetlinkCallback) -> Result<i32, i32> {
    let err = unsafe { nl_recvmsgs(self.ptr, netlink_callback.ptr) };
    if err < 0 {
      Err(err)
    } else {
      Ok(err)
    }
  }
}

impl Drop for NetlinkSocket {
  fn drop(&mut self) {
    unsafe {
      nl_socket_free(self.ptr);
    }
  }
}
