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

    let template_path = hidden_path.join("template.in");
    if !template_path.exists() {
        println!("Creating dummy template file");
        std::fs::write(template_path, b"&[]").expect("Failed to create dummy templates");
    }
}

#[cfg(feature = "qt")]
fn add_resources() {
    qt_ritual_build::add_resources(concat!(env!("CARGO_MANIFEST_DIR"), "/res/resources.qrc"));
}

fn main() {
    generate_dummys();
    #[cfg(feature = "qt")]
    add_resources();
}
