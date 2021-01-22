extern crate libc;
use libc::{size_t, c_uchar, c_void};
use libc::{memcpy};


/*

XXX this is an internal function. can remove "pub" qualifier, once all callers are in rust.

static void pkcs12_fill_buffer( unsigned char *data, size_t data_len,
                                const unsigned char *filler, size_t fill_len )
*/
#[no_mangle]
pub extern "C" fn pkcs12_fill_buffer( data: *mut c_uchar, mut data_len: size_t,
    filler: *const c_uchar, fill_len: size_t) {


    let mut p = data;
    let mut use_len: size_t;

    while data_len > 0 {
        use_len = if data_len > fill_len { fill_len } else { data_len };
        unsafe { memcpy( p as *mut c_void , filler as *mut c_void, use_len ) };
        p = unsafe { p.offset(use_len as isize) }; //p += use_len;
        data_len -= use_len;
    }

    return;
}

