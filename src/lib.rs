#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
mod ffi {
    include!(env!("BINDINGS_PATH"));
}

pub use ffi::*;
