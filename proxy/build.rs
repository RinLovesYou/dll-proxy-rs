use std::path::Path;

fn main() {
    #[cfg(target_os = "windows")]
    {
        let lib_path = Path::new("deps").join("Exports.def");
        println!("cargo:rustc-cdylib-link-arg=/DEF:{}", lib_path.display());
    }
}