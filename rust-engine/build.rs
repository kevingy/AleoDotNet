fn main() {
    // Generate C header using cbindgen
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("ALEO_DOTNET_ENGINE_H")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/aleo_dotnet_engine.h");
}
