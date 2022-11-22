fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-arg=/DEF:proxy-sys/src/deps/Exports.def");
    }
}