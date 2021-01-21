extern crate libc;
use libc::{size_t, c_uchar};


/*

XXX this is an internal function. can remove "pub" qualifier, once all callers are in rust.

static void pkcs12_fill_buffer( unsigned char *data, size_t data_len,
                                const unsigned char *filler, size_t fill_len )
*/
#[no_mangle]
pub extern "C" fn pkcs12_fill_buffer(
    _data: *mut c_uchar,
    _data_len: size_t,
    _filler: *const c_uchar,
    _fill_len: size_t) {


    println!("Hello, world!");
    return;
}

