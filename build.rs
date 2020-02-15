fn generate_secrets() {
    let mut secrets = std::collections::HashMap::<String, String>::new();

    {
        use std::io::BufRead;
        let path = std::path::Path::new("hidden/secrets");
        let file = std::fs::File::open(&path).expect("Failed to find 'secrets' file");
        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            let line = line.expect("Failed to read line from 'secrets'");
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
        let path = std::path::Path::new(
            &std::env::var("OUT_DIR").expect("Output directory not found in the environment"),
        )
        .join("secrets.rs");
        let mut file = std::fs::File::create(path).expect("Failed to create secret output file");

        for (k, v) in secrets {
            file.write_all(format!("pub static {}: &str = \"{}\";\n", k, v).as_bytes())
                .expect("Failed to write to secret output file");
        }
    }
}

fn generate_opencl(algorithm: &str) {
    use std::io::BufRead;
    use std::io::Write;

    let input = std::fs::File::open(std::path::Path::new(&format!("cl/{}.cl", algorithm)))
        .expect("Failed to open input file");
    let reader = std::io::BufReader::new(input);

    let mut output = {
        let path = std::path::Path::new(
            &std::env::var("OUT_DIR").expect("Output directory not found in the environment"),
        )
        .join("cl");

        if !path.exists() {
            std::fs::create_dir(&path).expect("Failed to create ocl output directory");
        }

        std::fs::File::create(path.join(&format!("{}.rs", algorithm)))
            .expect(&format!("Failed to open {}.cl output file", algorithm))
    };

    output
        .write_all(b"pub static SRC: &str = r#\"\n")
        .expect(&format!("Failed to write start to {}.cl", algorithm));
    for line in reader.lines() {
        let line = line.expect(&format!("Failed to read line from {}.cl", algorithm));
        output
            .write_all(format!("{}\n", line).as_bytes())
            .expect(&format!("Failed to write body to {}.cl", algorithm));
    }
    output
        .write_all(b"\"#;\n")
        .expect(&format!("Failed to write end to {}.cl", algorithm));
}

fn main() {
    generate_secrets();
    generate_opencl("md5");
    generate_opencl("sha256");
}
