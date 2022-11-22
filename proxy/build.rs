fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-cdylib-link-arg=/DEF:proxy/deps/Exports.def");
    }
}