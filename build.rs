fn main() {
    println!(
        "cargo:warning=OUT_DIR={}",
        std::env::var("OUT_DIR").unwrap()
    );
    println!(
        "cargo:warning=CARGO_PKG_NAME={}",
        std::env::var("CARGO_PKG_NAME").unwrap()
    );
    println!(
        "cargo:warning=CARGO_MANIFEST_DIR={}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );
}
