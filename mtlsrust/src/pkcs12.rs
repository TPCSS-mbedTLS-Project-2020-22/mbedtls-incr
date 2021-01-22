extern crate libc;
use libc::{size_t, c_uchar};
use libc::{c_int, c_uint, c_void};

/***************** defs from header files ******************/
#[derive(Debug)]
#[repr(C)]
pub struct mbedtls_asn1_buf
{
    tag: c_int,        /* ASN1 type, e.g. MBEDTLS_ASN1_UTF8_STRING. */
    len: size_t,       /* ASN1 length, in octets. */
    p: *mut c_uchar,   /* ASN1 data, e.g. in ASCII. */
}

const MBEDTLS_ERR_PKCS12_FEATURE_UNAVAILABLE: c_int = -0x1F00;
//const MBEDTLS_ERR_ERROR_CORRUPTION_DETECTED:  c_int = -0x006E;   /* This is a bug in the library */


#[derive(Debug)]
#[repr(C)]
struct mbedtls_arc4_context
{
    x: c_int,                      /* permutation index */
    y: c_int,                      /* permutation index */
    m: [c_uchar; 256],             /* permutation table */
}

/***********************************************************/
/* external dependencies */

extern {
    fn mbedtls_arc4_init( ctx: *mut mbedtls_arc4_context ) -> ();
    fn mbedtls_arc4_setup( ctx: *mut mbedtls_arc4_context, key: *const c_uchar, keylen: c_uint ) -> ();
    fn mbedtls_arc4_crypt( ctx: *mut mbedtls_arc4_context, length: size_t, input: *const c_uchar, output: *mut c_uchar ) -> c_int;
    fn mbedtls_arc4_free( ctx: *mut mbedtls_arc4_context );
}


extern {
    fn pkcs12_pbe_derive_key_iv( pbe_params: *mut mbedtls_asn1_buf, md_type: mbedtls_md_type_t,
                                     pwd: *const c_uchar,  pwdlen: size_t,
                                     key: *mut c_uchar, keylen: size_t,
                                     iv: *const c_uchar, ivlen: size_t ) -> c_int;
}


extern {
    fn mbedtls_platform_zeroize( buf: *mut c_void, len: size_t ) -> c_void;
}


#[allow(non_camel_case_types)]
type mbedtls_md_type_t = c_int;

const MBEDTLS_MD_SHA1: c_int = 4;

/*
/* from mbedtls-2.24.0/include/mbedtls/md.h */
enum mbedtls_md_type_t {
    MBEDTLS_MD_NONE=0,    /* None. */
    MBEDTLS_MD_MD2,       /* The MD2 message digest. */
    MBEDTLS_MD_MD4,       /* The MD4 message digest. */
    MBEDTLS_MD_MD5,       /* The MD5 message digest. */
    MBEDTLS_MD_SHA1,      /* The SHA-1 message digest. */
    MBEDTLS_MD_SHA224,    /* The SHA-224 message digest. */
    MBEDTLS_MD_SHA256,    /* The SHA-256 message digest. */
    MBEDTLS_MD_SHA384,    /* The SHA-384 message digest. */
    MBEDTLS_MD_SHA512,    /* The SHA-512 message digest. */
    MBEDTLS_MD_RIPEMD160, /* The RIPEMD-160 message digest. */
}
*/

/***********************************************************/



/*
int mbedtls_pkcs12_pbe_sha1_rc4_128( mbedtls_asn1_buf *pbe_params, int mode,
                             const unsigned char *pwd,  size_t pwdlen,
                             const unsigned char *data, size_t len,
                             unsigned char *output )

*/

#[no_mangle]
pub extern "C" fn mbedtls_pkcs12_pbe_sha1_rc4_128( pbe_params: *mut mbedtls_asn1_buf , _mode: c_int,
                             pwd: *const c_uchar,  pwdlen: size_t,
                             data: *const c_uchar, len: size_t,
                             output: *mut c_uchar ) -> c_int
{
    if cfg!(not(feature="MBEDTLS_ARC4_C")) {
        return MBEDTLS_ERR_PKCS12_FEATURE_UNAVAILABLE;
    }

    let mut ret: c_int;

    // unsigned char key[16];
    let key_arr = [ 0 as c_uchar; 16];
 
    let key: *const c_uchar = (&key_arr) as *const c_uchar;

    //mbedtls_arc4_context ctx;
    let mut ctx: mbedtls_arc4_context = mbedtls_arc4_context{ x: 0, y: 0, m: [0; 256], };

    unsafe { mbedtls_arc4_init( &mut ctx ) };

    const NULL: *const c_uchar = std::ptr::null();
    ret = unsafe { pkcs12_pbe_derive_key_iv( pbe_params, MBEDTLS_MD_SHA1,
            pwd, pwdlen,
            key as *mut c_uchar, 16, NULL, 0 ) };
    if ret != 0  {
        return ret ;
    }

    unsafe { mbedtls_arc4_setup( &mut ctx, key, 16 )};
    ret = unsafe { mbedtls_arc4_crypt( &mut ctx, len, data, output ) };

    let sizeof_key: size_t = key_arr.len() * std::mem::size_of::<c_uchar>();
    unsafe { mbedtls_platform_zeroize( key as *mut c_void, sizeof_key ) };
    unsafe { mbedtls_arc4_free( &mut ctx ) };

    return ret;
}



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
        unsafe{ std::ptr::copy_nonoverlapping(filler, p, use_len) };
        p = unsafe { p.offset(use_len as isize) }; //p += use_len;
        data_len -= use_len;
    }

    return;
}

