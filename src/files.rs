use crate::cli::print;
use crate::error;
use crate::hash;
use crate::options;
use crate::summary;

pub fn read<H: hash::Hash>(
    input: std::collections::HashSet<H>,
    paths: &std::collections::HashSet<std::path::PathBuf>,
    printer: print::Printer,
) -> std::collections::HashSet<H> {
    paths
        .iter()
        .inspect(|path| printer.read_start(path.display().to_string()))
        .filter_map(|file| {
            std::fs::File::open(file)
                .map_err(|e| {
                    printer.read_done(Err(error!(e; "Could not open file")));
                })
                .ok()
                .map(std::io::BufReader::new)
        })
        .fold(input, |acc, curr| insert_from_stream(acc, curr, printer))
}

// Allowed because it look better, dang it!
#[allow(clippy::filter_map)]
pub fn write<H: hash::Hash>(
    options: &options::Decrypt<H>,
    summary: &summary::Summary,
    printer: print::Printer,
) {
    options
        .files()
        .iter()
        .map(create_file)
        .filter_map(filter_error)
        .map(|(i, o, p)| write_output_file(H::regex(), &summary, &i, &o, p, printer))
        .for_each(finalize);
}

fn create_file(
    input: &std::path::PathBuf,
) -> Result<(std::fs::File, std::fs::File, std::path::PathBuf), error::Error> {
    let input_file = match std::fs::File::open(input) {
        Ok(file) => file,
        Err(e) => {
            bail!(e; "Could not open '{}' for translating", input.display());
        }
    };

    let file_name = if let Some(file_name) = input
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from)
    {
        file_name
    } else {
        bail!("Could not generate output file name");
    };

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
        match std::fs::File::create(&output) {
            Ok(file) => Ok((input_file, file, output)),
            Err(e) => bail!(
                e;
                "Could not open output file for '{}'",
                file_name,
            ),
        }
    }
}

fn write_output_file(
    regex: &regex::Regex,
    summary: &summary::Summary,
    input: &std::fs::File,
    output: &std::fs::File,
    output_path: std::path::PathBuf,
    printer: print::Printer,
) -> (
    print::Printer,
    Result<(), (std::path::PathBuf, error::Error)>,
) {
    use std::io::{BufRead, Write};

    printer.write_start(output_path.display().to_string());

    let mut buffer = String::new();
    let mut reader = std::io::BufReader::new(input);
    let mut writer = std::io::BufWriter::new(output);
    let mut replaced = false;

    loop {
        buffer.clear();
        match reader.read_line(&mut buffer) {
            Ok(bytes) => {
                if bytes == 0 {
                    return if replaced {
                        (printer, Ok(()))
                    } else {
                        (
                            printer,
                            Err(error!(output_path had "No replacements found")),
                        )
                    };
                }

                if regex.is_match(&buffer) {
                    for decrypted in &summary.results {
                        if !replaced {
                            replaced = buffer.contains(&decrypted.hash);
                        }
                        buffer = buffer.replace(&decrypted.hash, &decrypted.plain);
                    }
                }

                if let Err(e) = writer.write_all(buffer.as_bytes()) {
                    return (
                        printer,
                        Err(error!(output_path had e; "Failed to write to file")),
                    );
                }
            }
            Err(e) => {
                return (
                    printer,
                    Err(error!(output_path had e; "Failed to read input file")),
                );
            }
        }
    }
}

fn filter_error<T>(result: Result<T, error::Error>) -> Option<T> {
    match result {
        Ok(f) => Some(f),
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}

fn finalize(
    result: (
        print::Printer,
        Result<(), (std::path::PathBuf, error::Error)>,
    ),
) {
    result.0.write_done(result.1.map_err(|e| {
        let _ = std::fs::remove_file(e.0);
        e.1
    }));
}

pub fn insert_from_stream<H: hash::Hash>(
    mut input: std::collections::HashSet<H>,
    mut reader: impl std::io::BufRead,
    printer: print::Printer,
) -> std::collections::HashSet<H> {
    let mut buffer = String::new();
    let regex = H::regex();

    loop {
        buffer.clear();
        match reader.read_line(&mut buffer) {
            Ok(bytes) => {
                if bytes == 0 {
                    printer.read_done(Ok(()));
                    break;
                }

                input.extend(
                    regex.find_iter(&buffer).map(|h| {
                        H::from_str(h.as_str()).expect("Regex failed to capture valid hash")
                    }),
                );
            }
            Err(e) => {
                printer.read_done(Err(error!(e; "Error reading")));
                break;
            }
        }
    }
    input
}
