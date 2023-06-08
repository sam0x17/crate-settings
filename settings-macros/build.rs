fn main() {
    println!(
        "cargo:warning=MACROS:OUT_DIR={}",
        std::env::var("OUT_DIR").unwrap()
    );
    println!(
        "cargo:warning=MACROS:CARGO_PKG_NAME={}",
        std::env::var("CARGO_PKG_NAME").unwrap()
    );
    println!(
        "cargo:warning=MACROS:CARGO_MANIFEST_DIR={}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    println!(
        "cargo:rustc-env=OUT_DIR={}",
        std::env::var("OUT_DIR").unwrap()
    )
}
