use crate::error;
use crate::hash;
use crate::results;

pub fn read<H: hash::Hash>(
    input: &mut std::collections::HashSet<H>,
    path: &std::path::Path,
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
                    regex
                        .find_iter(&buffer)
                        .map(|h| H::from_str(h.as_str()))
                        .collect::<Result<Vec<_>, _>>()?,
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
    path: &std::path::Path,
    output: Option<std::path::PathBuf>,
    summary: &results::Summary,
) -> Result<(), error::Error> {
    let input = std::fs::File::open(path)
        .map_err(|e| error!(e; "Could not open '{}' for translating", path.display()))?;

    let output_path = match output {
        Some(output) => output,
        None => derive_output_file(path)?,
    };

    let output = std::fs::File::create(&output_path).map_err(|e| {
        error!(
            e;
            "Could not open output file for '{}'",
            output_path.display(),
        )
    })?;

    write_output_file(regex, summary, &input, &output, output_path)
}

fn derive_output_file(input: &std::path::Path) -> Result<std::path::PathBuf, error::Error> {
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
    }

    Ok(output)
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

                    let matches = regex
                        .captures_iter(&buffer)
                        .filter_map(|capture| capture.get(1).map(|group| group.as_str().to_owned()))
                        .collect::<Vec<_>>();

                    for matched in &matches {
                        if let Some(decrypted) = summary
                            .results
                            .iter()
                            .find(|decrypted| decrypted.hash == matched.as_str())
                        {
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
        let _ignored = std::fs::remove_file(output_path);
        e
    })
}
