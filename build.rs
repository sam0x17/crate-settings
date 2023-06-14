fn main() {
    if let Ok(var) = std::env::var("CARGO_MANIFEST_DIR") {
        if var.contains(".cargo") {
            println!(
                "cargo:warning=`crate-settings` must be a direct dependency for it to work \
                properly."
            );
        }
    }
}
