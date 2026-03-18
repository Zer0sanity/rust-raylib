#[allow(warnings, unused, clippy::approx_constant)]
pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/raylib_bindings.rs"));
}
