fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-arg=/DEF:src/deps/Exports.def");
    }
}