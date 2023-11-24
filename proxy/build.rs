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
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

#[cfg(not(any(feature = "version", feature = "winhttp", feature = "winmm")))]
compile_error!("At least one feature must enabled: \"version\", \"winhttp\", or \"winmm\".");

fn main() {
    println!("cargo:warning=Linking Exports File..");

    let deps = Path::new("deps");
    let mut exports = Vec::new();

    #[cfg(feature = "version")]
    exports.extend_from_slice(&get_exports(&deps.join("version-exports.txt")));

    #[cfg(feature = "winhttp")]
    exports.extend_from_slice(&get_exports(&deps.join("winhttp-exports.txt")));

    #[cfg(feature = "winmm")]
    exports.extend_from_slice(&get_exports(&deps.join("winmm-exports.txt")));

    let lib_str = exports
        .into_iter()
        .fold("EXPORTS".to_owned(), |a, b| a + "\n" + &b);

    let out_dir = std::env::var("OUT_DIR").expect("failed to get OUT_DIR");
    let lib_path = Path::new(&out_dir).join("Exports.def");

    {
        File::create(&lib_path)
            .expect("failed to open lib path")
            .write_all(lib_str.as_bytes())
            .expect("failed to write lib file");
    }

    let absolute_path = std::fs::canonicalize(&lib_path).unwrap();
    println!(
        "cargo:rustc-cdylib-link-arg=/DEF:{}",
        absolute_path.display()
    );
}

fn get_exports(path: &Path) -> Vec<String> {
    let file = File::open(path).expect("failed to open exports file");
    BufReader::new(file)
        .lines()
        .collect::<Result<_, _>>()
        .expect("failed to read exports file")
}
