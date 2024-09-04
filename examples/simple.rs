pub fn main() {
    println!("edflib-sys version {}", edflib_sys::EDFLIBSYS_VERSION.unwrap());
    let version = unsafe { edflib_sys::edflib_version() };
    println!("edflib version {}", version);
}
