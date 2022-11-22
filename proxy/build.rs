fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-cdylib-link-arg=/DEF:deps/Exports.def");
    }
}