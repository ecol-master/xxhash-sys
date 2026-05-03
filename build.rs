use std::path::PathBuf;

// Generates the bindings for `libxxhash` C library.
fn main() {
    println!("cargo::rerun-if-changed=wrapper.h");

    // Generating bindings only for XXH* stuff.
    let mut builder = bindgen::builder()
        .header("wrapper.h")
        .allowlist_function("XXH.*")
        .allowlist_type("XXH.*")
        .allowlist_var("XXH.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    let lib = pkg_config::Config::new()
        .atleast_version("0.8")
        .probe("libxxhash")
        .expect("Couldn't find libxxhash >= 0.8 via pkg-config; install xxhash development headers and pkg-config");

    for path in lib.include_paths.iter() {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    let bindings = builder.generate().expect("Unable to generate bindings");
    let bindings_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs");

    bindings
        .write_to_file(bindings_path)
        .expect("Couldn't wite bindings!");
}
