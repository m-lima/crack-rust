use crate::error;
use crate::hash;
use crate::Input;

pub trait Reader<T: Input> {
    fn extract_from<R: std::io::BufRead>(
        set: &mut std::collections::HashSet<T>,
        reader: R,
    ) -> Result<(), error::Error> {
        unimplemented!();
    }
}

impl<H: hash::Hash> Reader<H> for H {
    fn extract_from<R: std::io::BufRead>(
        set: &mut std::collections::HashSet<H>,
        mut reader: R,
    ) -> Result<(), error::Error> {
        // fn insert_from<R: std::io::BufRead>(&mut self, mut reader: R) -> Result<(), error::Error> {
        let mut buffer = String::new();
        let regex = H::regex();

        loop {
            buffer.clear();
            match reader.read_line(&mut buffer) {
                Ok(bytes) => {
                    if bytes == 0 {
                        break;
                    }

                    set.extend(
                        regex
                            .find_iter(&buffer)
                            .map(|h| H::from_str(h.as_str()).unwrap()),
                    );
                }
                Err(e) => {
                    return error!(e; "Error reading");
                }
            }
        }
        Ok(())
    }
}

impl Reader<String> for String {
    fn extract_from<R: std::io::BufRead>(
        set: &mut std::collections::HashSet<String>,
        mut reader: R,
    ) -> Result<(), error::Error> {
        let mut buffer = String::new();
        if let Ok(bytes) = reader.read_to_string(&mut buffer) {
            if bytes > 0 {
                set.insert(buffer);
            }
        }
        Ok(())
    }
}

pub fn extract_from_files<H: hash::Hash>(input: std::collections::HashSet<H>, paths: &std::collections::HashSet<std::path::PathBuf>) -> std::collections::HashSet<H> {
    for path in paths {
        use stream::Reader;

        print::loading_start(shared.shared.verbose, &path.display().to_string());
        let file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(e) => {
                print::loading_done(
                    shared.shared.verbose,
                    error!(e; "Could not open '{}'", path.display()),
                );
                continue;
            }
        };
        print::loading_done(
            shared.shared.verbose,
            H::extract_from(&mut input, std::io::BufReader::new(file)),
        );
    }
}