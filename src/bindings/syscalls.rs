use crate::bindings::types::{c_char, size_t};
use libc::c_void;

unsafe extern "C" {
    pub fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char;
}
