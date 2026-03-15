use std::env;
use std::path::PathBuf;

fn main() {
    // Compile raylib using CMake
    let dst = cmake::Config::new("raylib")
        .define("BUILD_EXAMPLES", "OFF")
        .build();

    let bindings = bindgen::Builder::default()
    .header("raylib/src/raylib.h")
    .generate()
    .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
    
    // Tell cargo where to find the library
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=raylib");

    // Link system dependencies (vary by OS)
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=GL");
        println!("cargo:rustc-link-lib=m");
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=dl");
    }
}
// Local Variables:
// jinx-local-words: "CMake dl pthread raylib rustc"
// End:
