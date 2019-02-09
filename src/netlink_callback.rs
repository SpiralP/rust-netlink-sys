use super::*;
use std::ffi::c_void;

pub struct NetlinkCallback {
  pub ptr: *mut nl_cb,
}
impl NetlinkCallback {
  pub fn alloc() -> Result<Self, ()> {
    let ptr = unsafe { nl_cb_alloc(nl_cb_kind_NL_CB_DEFAULT) };
    if ptr.is_null() {
      Err(())
    } else {
      Ok(Self { ptr })
    }
  }

  #[allow(clippy::not_unsafe_ptr_arg_deref)]
  pub fn nl_cb_set(
    &self,
    cb_type: nl_cb_type,
    cb_kind: nl_cb_kind,
    cb: nl_recvmsg_msg_cb_t,
    data: *mut c_void,
  ) -> Result<(), i32> {
    let err = unsafe { nl_cb_set(self.ptr, cb_type, cb_kind, cb, data) };
    if err == 0 {
      Ok(())
    } else {
      Err(err)
    }
  }

  #[allow(clippy::not_unsafe_ptr_arg_deref)]
  pub fn nl_cb_err(
    &self,
    cb_kind: nl_cb_kind,
    cb: nl_recvmsg_err_cb_t,
    data: *mut c_void,
  ) -> Result<(), i32> {
    let err = unsafe { nl_cb_err(self.ptr, cb_kind, cb, data) };
    if err == 0 {
      Ok(())
    } else {
      Err(err)
    }
  }
}

impl Drop for NetlinkCallback {
  fn drop(&mut self) {
    unsafe {
      nl_cb_put(self.ptr);
    }
  }
}
