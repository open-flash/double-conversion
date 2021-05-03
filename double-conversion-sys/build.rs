fn main() {
  // Builds the project in the directory located in `libfoo`, installing it
  // into $OUT_DIR
  // let lib_out = cmake::Config::new("../../lib/double-conversion").build();
  let wrapper_out = cmake::Config::new(".").build();
  let lib_out = cmake::Config::new("double-conversion").build();

  // // Builds the project in the directory located in `libfoo`, installing it
  // // into $OUT_DIR
  // let dst = cmake::Config::new("../../lib/double-conversion")
  //     // .cflag("-fkeep-inline-functions")
  //     // .cflag("-fkeep-static-functions")
  //     // .cflag("-fno-inline-functions")
  //     // .cxxflag("-fkeep-inline-functions")
  //     // .cxxflag("-fkeep-static-functions")
  //     // .cxxflag("-fno-inline-functions")
  //     // .build_arg("-fkeep-inline-functions")
  //     .build();

  // println!("cargo:rustc-link-search={}", lib_out.display());
  println!("cargo:rustc-link-search={}", wrapper_out.display());
  println!("cargo:rustc-link-search={}", lib_out.display());
  // println!("cargo:rustc-link-search=native={}", dst.display());
  // println!("cargo:rustc-link-lib=double-conversion");
  println!("cargo:rustc-link-lib=double-conversion-wrapper");
  println!("cargo:rustc-link-lib=double-conversion");

  // The bindgen::Builder is the main entry point
  // to bindgen, and lets you build up options for
  // the resulting bindings.
  let bindings = bindgen::Builder::default()
      // The input header we would like to generate
      // bindings for.
      .header("./wrapper.hpp")
      // .allowlist_type("double_conversion::DoubleToStringConverter")
      // .allowlist_function("double_conversion::DoubleToStringConverter::ToShortest")
      // .clang_arg("-x").clang_arg("c++")
      // .generate_inline_functions(true)
      // .clang_arg("-std=c++14")
      // Tell cargo to invalidate the built crate whenever any of the
      // included header files changed.
      .parse_callbacks(Box::new(bindgen::CargoCallbacks))
      // Finish the builder and generate the bindings.
      .generate()
      // Unwrap the Result and panic on failure.
      .expect("Unable to generate bindings");

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
  bindings
      .write_to_file(out_path.join("bindings.rs"))
      .expect("Couldn't write bindings!");


  // cxx_build::bridge("src/main.rs")  // returns a cc::Build
  //   .file("src/demo.cc")
  //   .flag_if_supported("-std=c++11")
  //   .compile("cxxbridge-demo");
  //
  // println!("cargo:rerun-if-changed=src/main.rs");
  // println!("cargo:rerun-if-changed=src/demo.cc");
  // println!("cargo:rerun-if-changed=include/demo.h");
}
