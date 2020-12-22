use crate::error;
use crate::hash;
use crate::results;

pub fn read<H: hash::Hash>(
    input: &mut std::collections::HashSet<H>,
    path: &std::path::PathBuf,
) -> Result<(), error::Error> {
    std::fs::File::open(path)
        .map(std::io::BufReader::new)
        .map_err(|e| error!(e; "Could not open file: {}", path.display()))
        .and_then(|stream| read_from_stream(input, stream))
}

pub fn read_from_stream<H: hash::Hash>(
    input: &mut std::collections::HashSet<H>,
    mut stream: impl std::io::BufRead,
) -> Result<(), error::Error> {
    let mut buffer = String::new();
    let regex = H::regex();

    loop {
        buffer.clear();
        match stream.read_line(&mut buffer) {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                }

                input.extend(
                    regex.find_iter(&buffer).map(|h| {
                        H::from_str(h.as_str()).expect("Regex failed to capture valid hash")
                    }),
                );
            }
            Err(e) => {
                bail!(e; "Error while reading");
            }
        }
    }
    Ok(())
}

pub fn write(
    regex: &regex::Regex,
    path: &std::path::PathBuf,
    summary: &results::Summary,
) -> Result<(), error::Error> {
    create_file(path).and_then(|(i, o, p)| write_output_file(regex, &summary, &i, &o, p))
}

fn create_file(
    input: &std::path::PathBuf,
) -> Result<(std::fs::File, std::fs::File, std::path::PathBuf), error::Error> {
    let input_file = std::fs::File::open(input)
        .map_err(|e| error!(e; "Could not open '{}' for translating", input.display()))?;

    let file_name = input
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from)
        .ok_or_else(|| error!("Could not generate output file name"))?;

    let mut output = input.with_file_name(format!("{}.cracked", file_name));

    let mut index = 0;
    while output.exists() && index < 100 {
        output = input.with_file_name(format!("{}.cracked.{}", file_name, index));
        index += 1;
    }

    if output.exists() {
        bail!(
            "Could not create output file name for '{}': too many name collisions",
            file_name
        )
    } else {
        std::fs::File::create(&output)
            .map(|file| (input_file, file, output))
            .map_err(|e| {
                error!(
                    e;
                    "Could not open output file for '{}'",
                    file_name,
                )
            })
    }
}

fn write_output_file(
    regex: &regex::Regex,
    summary: &results::Summary,
    input: &std::fs::File,
    output: &std::fs::File,
    output_path: std::path::PathBuf,
) -> Result<(), error::Error> {
    fn inner(
        regex: &regex::Regex,
        summary: &results::Summary,
        input: &std::fs::File,
        output: &std::fs::File,
    ) -> Result<(), error::Error> {
        use std::io::{BufRead, Write};

        let mut buffer = String::new();
        let mut reader = std::io::BufReader::new(input);
        let mut writer = std::io::BufWriter::new(output);

        loop {
            buffer.clear();
            match reader.read_line(&mut buffer) {
                Ok(bytes) => {
                    if bytes == 0 {
                        return Ok(());
                    }

                    if regex.is_match(&buffer) {
                        for decrypted in &summary.results {
                            buffer = buffer.replace(&decrypted.hash, &decrypted.plain);
                        }
                    }

                    if let Err(e) = writer.write_all(buffer.as_bytes()) {
                        bail!(e;  "Failed to write to file");
                    }
                }
                Err(e) => {
                    bail!(e;  "Failed to read input file");
                }
            }
        }
    }

    inner(regex, summary, input, output).map_err(|e| {
        let _ = std::fs::remove_file(output_path);
        e
    })
}
