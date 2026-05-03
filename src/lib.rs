#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
/// Re-exports all stuff located in `libxxhash.h`
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use ffi::*;

#[cfg(test)]
mod tests {
    use super::ffi::XXH64;

    #[test]
    fn xxh64_works() {
        let dead = String::from("0xdead");
        let hash = unsafe { XXH64(dead.as_ptr() as *const std::os::raw::c_void, dead.len(), 1) };
        assert!(hash == 11058709446519869363);
    }
}
