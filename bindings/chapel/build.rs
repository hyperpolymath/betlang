// SPDX-License-Identifier: MIT OR Apache-2.0
//! Build script to generate C header for Chapel

fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("BET_CHAPEL_H")
        .with_documentation(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/betlang.h");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
