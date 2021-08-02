fn generate_dummys() {
    let hidden_path = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/hidden"));

    if !hidden_path.exists() {
        std::fs::create_dir(hidden_path).expect("Failed to create hidden directory");
    }

    let salt_path = hidden_path.join("salt");
    if !salt_path.exists() {
        println!("Creating dummy salt file");
        std::fs::File::create(salt_path).expect("Failed to create dummy salt");
    }

    let xor_path = hidden_path.join("xor");
    if !xor_path.exists() {
        println!("Creating dummy xor file");
        std::fs::File::create(xor_path).expect("Failed to create dummy xor");
    }

    let template_path = hidden_path.join("template.in");
    if !template_path.exists() {
        println!("Creating dummy template file");
        std::fs::write(template_path, b"[Template::new(\"Custom\", \"\", 11)]")
            .expect("Failed to create dummy templates");
    }
}

#[cfg(feature = "qml")]
fn build_cpp() {
    let cpp_include_path = concat!(env!("CARGO_MANIFEST_DIR"), "/cpp");
    let qt_include_path = std::env::var("DEP_QT_INCLUDE_PATH").unwrap();
    let qt_library_path = std::env::var("DEP_QT_LIBRARY_PATH").unwrap();

    let mut config = cpp_build::Config::new();

    if cfg!(target_os = "macos") {
        config.flag("-F");
        config.flag(qt_library_path.trim());
    }

    config
        .include(cpp_include_path)
        .include(qt_include_path.trim())
        .flag_if_supported("-std=c++17")
        .flag_if_supported("/std:c++17")
        .build("src/gui/mod.rs");
}

fn main() {
    generate_dummys();

    #[cfg(feature = "qml")]
    build_cpp();
}
