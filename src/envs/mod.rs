pub fn env_toml_path() {
    let cargo_toml_path = env!("CARGO_MANIFEST_DIR");
    println!("Cargo.toml path: {}", cargo_toml_path);
}
