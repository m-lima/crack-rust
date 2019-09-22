mod args;
mod decrypt;
mod encrypt;
mod options;
mod secrets;

fn main() {
    let options = args::parse();
    println!("{}", options);
    match options {
        options::Variant::Encrypt(options) => {
            encrypt::execute(options);
        }
        options::Variant::Decrypt(options) => {
            decrypt::execute(options);
        }
    }
}
