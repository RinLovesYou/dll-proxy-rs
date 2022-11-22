use std::path::Path;

fn main() {
    #[cfg(target_os = "windows")]
    {
        let lib_path = Path::new("deps").join("Exports.def");
        let absolute_path = std::fs::canonicalize(&lib_path).unwrap();
        println!("cargo:rustc-cdylib-link-arg=/DEF:{}", absolute_path.display());
    }
}