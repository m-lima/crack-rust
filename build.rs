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
        std::fs::write(template_path, b"[Template::new(\"Custom\", \"\", 11)]")
            .expect("Failed to create dummy templates");
    }
}

fn main() {
    generate_dummys();
}
