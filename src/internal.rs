use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

use super::feature;

/// Copies up to 256 bytes (including null termination) to
/// the provided destination. If the provided source string is too long, it will be
/// truncated.
pub unsafe fn copy_to_c_buffer(mut src: String, dest: *mut c_char) {
    // Truncate to 255 bytes (256 including the null terminator)
    src.truncate(255);
    let src_c = CString::new(src).unwrap_or_else(|_| CString::new("<invalid>").unwrap());
    let src_c_length = src_c.to_bytes_with_nul().len();
    debug_assert!(src_c_length <= 256);
    ptr::copy_nonoverlapping(src_c.as_ptr(), dest, src_c_length);
}

/// Enables native paths
pub fn path_init() {
    // Feature specified to exist in SDK 2.1
    let native_path_feature =
        feature::find_feature("XPLM_USE_NATIVE_PATHS").expect("No native paths feature");
    native_path_feature.set_enabled(true);
}

/// Performs initialization required for the XPLM crate to work correctly
pub fn xplm_init() {
    path_init();
}
