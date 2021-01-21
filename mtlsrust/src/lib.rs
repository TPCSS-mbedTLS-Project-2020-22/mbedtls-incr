

pub mod pkcs12;
pub use pkcs12::*;

#[no_mangle]
// pub unsafe extern "C" fn func_in_rust(x: i32) -> i32 {
pub extern "C" fn func_in_rust(x: i32) -> i32 {
        return x + 10;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
