use std::ffi::{c_char, CStr};


#[no_mangle]
pub extern "C" fn do_handshake(left: *const c_char, right: *const c_char) -> bool {
    println!("checking in the lib...");
    let left = unsafe {CStr::from_ptr(left)};
    let right = unsafe {CStr::from_ptr(right)};
    left == right
}