extern crate libc;
use libc::{size_t, c_uchar};

#[no_mangle]
pub extern "C" fn pkcs12_fill_buffer(
    _data: *mut c_uchar,
    _data_len: size_t,
    _filler: *const c_uchar,
    _fill_len: size_t) {


    println!("Hello, world!");
    return;
}

