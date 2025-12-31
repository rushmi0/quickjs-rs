use std::{env, path::PathBuf};

const QUICKJS_HEADERS: &[&str] = &[
    "builtin-array-fromasync.h",
    "dtoa.h",
    "libregexp-opcode.h",
    "libregexp.h",
    "libunicode-table.h",
    "libunicode.h",
    "list.h",
    "quickjs-atom.h",
    "quickjs-opcode.h",
    "quickjs-c-atomics.h",
    "quickjs.h",
    "quickjs-libc.h",
    "cutils.h",
];

const QUICKJS_SOURCES: &[&str] = &[
    "quickjs-libc.c",
    "quickjs.c",
    "libunicode.c",
    "libregexp.c",
    "cutils.c",
    "dtoa.c",
];

fn main() {
    println!("cargo:rerun-if-changed=quickjs.bridge.h");
    println!("cargo:rerun-if-changed=quickjs.bridge.c");

    for h in QUICKJS_HEADERS {
        println!("cargo:rerun-if-changed=quickjs/{h}");
    }
    for c in QUICKJS_SOURCES {
        println!("cargo:rerun-if-changed=quickjs/{c}");
    }

    build_quickjs();

    if env::var("CARGO_FEATURE_NOBINDGEN").is_err() {
        generate_bindings();
    }
}

fn build_quickjs() {
    let mut cc = cc::Build::new();

    for src in QUICKJS_SOURCES {
        cc.file(format!("quickjs/{src}"));
    }

    cc.file("quickjs.bridge.c")
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
        .header("quickjs.bridge.h")
        .clang_arg("-Iquickjs")
        .clang_arg("-std=c99")
        .derive_default(true)
        .generate_comments(true)
        .layout_tests(false)
        .allowlist_function("js_.*")
        .allowlist_function("JS_.*")
        .allowlist_type("JS.*")
        .allowlist_var("JS_.*")
        .blocklist_item("FP_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate QuickJS bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
