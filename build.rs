use std::env;
use std::path::PathBuf;

fn main() {
    // Compile raylib using CMake
    let dst = cmake::Config::new("raylib")
        .define("BUILD_EXAMPLES", "OFF")
        .build();

    let bindings = bindgen::Builder::default()
        .header("raylib/src/raylib.h")
        // window
        .allowlist_function("InitWindow")
        .allowlist_function("IsWindowReady")
        .allowlist_function("WindowShouldClose")
        .allowlist_function("CloseWindow")
        // drawing 2d
        .allowlist_function("BeginDrawing")
        .allowlist_function("EndDrawing")
        .allowlist_function("ClearBackground")
        .allowlist_function("DrawText")
        .allowlist_function("MeasureText")
        .allowlist_function("DrawRectangle")
        .allowlist_function("DrawRectangleLines")
        // drawing 3d
        .allowlist_function("BeginMode3D")
        .allowlist_function("EndMode3D")
        .allowlist_function("DrawGrid")
        .allowlist_function("DrawCube")
        .allowlist_function("DrawCubeWires")
        .allowlist_function("DrawSphere")
        .allowlist_function("DrawSphereWires")
        // camera
        .allowlist_type("CameraProjection")
        // input
        .allowlist_function("IsKeyDown")
        .allowlist_type("KeyboardKey")
        .allowlist_function("GetMouseDelta")
        .allowlist_function("GetMousePosition")
        // misc
        .allowlist_function("GetFrameTime")
        .allowlist_function("GetScreenToWorldRay")
        .allowlist_function("GetTime")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("raylib_bindings.rs"))
        .expect("Couldn't write bindings!");

    // Tell cargo where to find the library
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    println!("cargo:rustc-link-lib=static=raylib");

    // Link system dependencies (vary by OS)
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=GL");
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=Xcursor");
        println!("cargo:rustc-link-lib=Xrandr");
        println!("cargo:rustc-link-lib=Xinerama");
        println!("cargo:rustc-link-lib=Xi");
        println!("cargo:rustc-link-lib=m");
        println!("cargo:rustc-link-lib=pthread");
        println!("cargo:rustc-link-lib=dl");
    }
}
// Local Variables:
// jinx-local-words: "CMake Init Xcursor Xinerama Xrandr dl pthread raylib rustc src"
// End:
