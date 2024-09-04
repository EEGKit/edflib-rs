use std::{ ffi::CString, os::raw::c_char };

use edflib_sys::*;

pub fn main() {
    let mut edfhdr: edf_hdr_struct;

    let c_str = CString::new("").unwrap();
    let path: *const c_char = c_str.as_ptr() as *const c_char;

    // unsafe {
    //     edfopen_file_readonly(path, edfhdr, EDFLIB_READ_ALL_ANNOTATIONS);
    // }

    println!("edflib-sys version {}", edflib_sys::EDFLIBSYS_VERSION.unwrap());
    let version = unsafe { edflib_sys::edflib_version() };
    println!("edflib version {}", (version as f64) / (100 as f64));
}
