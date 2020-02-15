fn generate_secrets() {
    let mut secrets = std::collections::HashMap::<String, String>::new();

    {
        use std::io::BufRead;
        let path = std::path::Path::new("hidden/secrets");
        let file = std::fs::File::open(&path).unwrap();
        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            match line.find('=') {
                Some(pivot) => {
                    secrets.insert(
                        String::from(&line[0..pivot]),
                        String::from(&line[(pivot + 1)..]),
                    );
                }
                None => {}
            }
        }
    }

    {
        use std::io::Write;
        let path = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("secrets.rs");
        let mut file = std::fs::File::create(path).unwrap();

        for (k, v) in secrets {
            file.write_all(format!("pub static {}: &str = \"{}\";\n", k, v).as_bytes())
                .unwrap();
        }
    }
}

fn main() {
    generate_secrets();
}
