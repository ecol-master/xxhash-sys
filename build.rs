use std::path::PathBuf;

// Generates the bindings for `libxxhash` C library.
fn main() {
    println!("cargo::rerun-if-changed=wrapper.h");

    let mut builder = bindgen::builder()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    let lib = pkg_config::Config::new()
        .exactly_version("0.8.3")
        .statik(true)
        .probe("libxxhash")
        .expect("Couldn't find `libxxhash` package");

    for path in lib.include_paths.iter() {
        builder = builder.clang_arg(format!("-I{}", path.display()));
    }

    let bindings = builder.generate().expect("Unable to generate bindings");
    let bindings_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs");

    println!("cargo:rustc-env=BINDINGS_PATH={}", bindings_path.display());
    bindings
        .write_to_file(bindings_path)
        .expect("Couldn't wite bindings!");
}
