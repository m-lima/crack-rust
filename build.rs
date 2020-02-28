fn generate_secrets() {
    let hidden_path = std::path::Path::new("hidden");

    if !hidden_path.exists() {
        std::fs::create_dir(hidden_path).expect("Failed to create hidden directory");
    }

    let salt_path = hidden_path.join("salt");
    if !salt_path.exists() {
        println!("Creating dummy salt file");
        std::fs::File::create(salt_path).expect("Failed to create dummy salt");
    }
}

fn main() {
    generate_secrets();
}
