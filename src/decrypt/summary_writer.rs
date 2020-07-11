use crate::{error::Error, options, summary};

pub(super) fn export(options: &options::Decrypt, summary: &summary::Decrypt) {
    options
        .files()
        .iter()
        .map(create_file)
        .filter_map(filter_error)
        .filter_map(|(i, o, p)| write_output_file(&options, &summary, &i, &o, p))
        .for_each(clean_unwritten_file);
}

fn create_file(
    input: &std::path::PathBuf,
) -> Result<(std::fs::File, std::fs::File, std::path::PathBuf), Error> {
    let input_file = match std::fs::File::open(input) {
        Ok(file) => file,
        Err(e) => {
            return error!(e; "Could not open '{}' for reading", input.display());
        }
    };

    let file_name = if let Some(file_name) = input
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from)
    {
        file_name
    } else {
        return error!("Could not generate output file name");
    };

    let mut output = input.with_file_name(format!("{}.cracked", file_name));

    let mut index = 0;
    while output.exists() && index < 100 {
        output = input.with_file_name(format!("{}.cracked.{}", file_name, index));
        index += 1;
    }

    if output.exists() {
        error!("Could not create output file for '{}'", file_name)
    } else {
        match std::fs::File::create(&output) {
            Ok(file) => Ok((input_file, file, output)),
            Err(e) => error!(
                e;
                "Could not open output file for '{}'",
                file_name,
            ),
        }
    }
}

fn write_output_file(
    options: &options::Decrypt,
    summary: &summary::Decrypt,
    input: &std::fs::File,
    output: &std::fs::File,
    output_path: std::path::PathBuf,
) -> Option<std::path::PathBuf> {
    use options::SharedAccessor;
    use std::io::{BufRead, Write};

    let mut buffer = String::new();
    let mut reader = std::io::BufReader::new(input);
    let mut writer = std::io::BufWriter::new(output);
    let mut replaced = false;
    let regex = options.algorithm().regex();

    loop {
        buffer.clear();
        if let Ok(bytes) = reader.read_line(&mut buffer) {
            if bytes == 0 {
                return if replaced { None } else { Some(output_path) };
            }

            if regex.is_match(&buffer) {
                for decrypted in &summary.results {
                    if !replaced {
                        replaced = buffer.contains(&decrypted.hash);
                    }
                    buffer = buffer.replace(&decrypted.hash, &decrypted.plain);
                }
            }

            if writer.write_all(buffer.as_bytes()).is_err() {
                return Some(output_path);
            }
        } else {
            return Some(output_path);
        }
    }
}

fn clean_unwritten_file(file: std::path::PathBuf) {
    let _ = std::fs::remove_file(file);
}

fn filter_error<T>(result: Result<T, Error>) -> Option<T> {
    match result {
        Ok(f) => Some(f),
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}
