mod args;
mod decrypt;
mod encrypt;
mod hash;
mod options;
mod print;
mod secrets;
mod summary;

fn main() {
    let (options, verboseness) = args::parse();
    let print = print::new(verboseness);

    print.options(&options);

    let summary = match options {
        options::Variant::Encrypt(options) => encrypt::execute(options),
        options::Variant::Decrypt(options) => decrypt::execute(options),
    };

    print.summary(&summary);
}
