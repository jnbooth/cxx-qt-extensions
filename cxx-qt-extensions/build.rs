fn main() {
    let qt_modules = vec!["Core".to_owned()];
    let qtbuild = qt_build_utils::QtBuild::new(qt_modules).expect("Could not find Qt installation");

    // Required for tests
    qt_build_utils::setup_linker();

    // Find the Qt version and tell the Rust compiler
    // this allows us to have conditional Rust code
    println!(
        "cargo:rustc-cfg=qt_version_major=\"{}\"",
        qtbuild.version().major
    );

    let bridges = vec!["core/quuid", "core/qvariant/qvariant_quuid"];

    for bridge in &bridges {
        println!("cargo:rerun-if-changed=src/{bridge}.rs");
    }

    for include_path in qtbuild.include_paths() {
        cxx_build::CFG
            .exported_header_dirs
            .push(include_path.as_path());
    }

    let mut builder = cxx_build::bridges(bridges.iter().map(|bridge| format!("src/{bridge}.rs")));

    qtbuild.cargo_link_libraries(&mut builder);

    let cpp_files = vec!["core/quuid", "core/qvariant/qvariant", "qt_types"];

    for bridge in &cpp_files {
        builder.file(format!("src/{bridge}.cpp"));
        println!("cargo:rerun-if-changed=src/{bridge}.cpp");
    }
    println!("cargo:rerun-if-changed=src/assertion_utils.h");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    cxx_qt_lib_headers::write_headers(format!("{out_dir}/cxx-qt-lib"));
    cxx_qt_extensions_headers::write_headers(format!("{out_dir}/cxx-qt-extensions"));
    builder.include(out_dir);

    builder.cpp(true);
    // MSVC
    builder.flag_if_supported("/std:c++17");
    builder.flag_if_supported("/Zc:__cplusplus");
    builder.flag_if_supported("/permissive-");
    builder.flag_if_supported("/bigobj");
    // GCC + Clang
    builder.flag_if_supported("-std=c++17");
    // MinGW requires big-obj otherwise debug builds fail
    builder.flag_if_supported("-Wa,-mbig-obj");

    builder.compile("cxx-qt-extensions");
}
