extern crate libc;

extern "C" {
    fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int;
}

use std::ffi::CStr;

pub fn get_hostname() -> Option<String> {
    let len = 255;
    let mut buf = Vec::<u8>::with_capacity(len);
    let ptr = buf.as_mut_ptr() as *mut libc::c_char;

    unsafe {
        if gethostname(ptr, len as libc::size_t) != 0 {
            return None;
        }

        Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
    }
}

#[test]
fn test_get_hostname() {
    assert!(get_hostname().is_some());
    assert!(!get_hostname().unwrap().is_empty());
}