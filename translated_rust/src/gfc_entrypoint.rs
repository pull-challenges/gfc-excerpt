use crate::src::gfc::gfc_decrypt;
use crate::src::gfc::gfc_destroy;
use crate::src::gfc::gfc_encrypt;
use crate::src::gfc::gfc_init;
use crate::src::gfc::GFC;
extern "C" {
    fn strtoull(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
}
pub(crate) unsafe fn print_decrypted(input: u64) {
    let gfc: *mut GFC = gfc_init(100, 6, 42) as *mut GFC;
    let enc: u64 = gfc_encrypt(gfc.as_ref().unwrap(), input);
    let dec: u64 = gfc_decrypt(gfc.as_ref().unwrap(), enc);
    println!("enc = {enc}");
    println!("dec = {dec}");
    gfc_destroy(gfc);
}
unsafe fn main_0(argc: i32, argv: &mut [*mut i8]) -> i32 {
    if argc < 2 {
        eprintln!(
            "Usage: {0} <number>",
            std::ffi::CStr::from_ptr((&argv[0..])[0] as _)
                .to_str()
                .unwrap()
        );
        return 1;
    }
    let mut endptr: Option<&mut i8> = None;
    let input_num: u64 = strtoull((&argv[1..])[0], &raw mut endptr as *mut *mut i8, 0);
    print_decrypted(input_num);
    0
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe { ::std::process::exit(main_0((args.len() - 1) as i32, &mut args)) }
}
