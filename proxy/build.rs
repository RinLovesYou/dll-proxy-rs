use std::path::Path;

/// This may no longer be supported in the future...
/// Need to find an alternative to forwarding exports to dependent crates.
///
/// ```
/// #[used]
/// #[link_section = ".drectve"]
/// #[cfg(all(windows, target_env = "msvc"))]
/// static DEF_ARG: [u8; 27] = *b"/DEF:proxy/deps/Exports.def";
/// ```
///
/// was suggested, however this does not seem to be a viable option, the linker seems to swallow this.
fn main() {
    #[cfg(target_os = "windows")]
    {
        let lib_path = Path::new("deps").join("Exports.def");
        let absolute_path = std::fs::canonicalize(&lib_path).unwrap();
        println!(
            "cargo:rustc-cdylib-link-arg=/DEF:{}",
            absolute_path.display()
        );
    }
}
