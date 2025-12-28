use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=quickjs");

    build_quickjs();

    if env::var("CARGO_FEATURE_NOBINDGEN").is_err() {
        generate_bindings();
    }
}

fn build_quickjs() {
    cc::Build::new()
        .files([
            "quickjs/quickjs.c",
            "quickjs/libunicode.c",
            "quickjs/libregexp.c",
            "quickjs/cutils.c",
            "quickjs/dtoa.c",
        ])
        .include("quickjs")
        .define("_GNU_SOURCE", None)
        .flag_if_supported("-std=c99")
        .flag_if_supported("-fPIC")
        .warnings(false)
        .compile("quickjs");
}

fn generate_bindings() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .derive_default(true)
        .generate_comments(false)
        .layout_tests(false)
        .clang_arg("-Iquickjs")
        .clang_arg("-std=c99")
        .allowlist_type("JS.*")
        .allowlist_function("JS_.*")
        .allowlist_var("JS_.*")
        .blocklist_item("FP_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate QuickJS bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
