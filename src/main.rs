mod args;
mod decrypt;
mod encrypt;
mod options;
mod secrets;
// mod summary;

fn main() {
    let options = args::parse();
    println!("{}", options);
    println!("----------");
    match options {
        options::Variant::Encrypt(options) => {
            encrypt::execute(options);
        }
        options::Variant::Decrypt(options) => {
            decrypt::execute(options);
        }
    }
    // println!("----------");
    // let summary = match options {
    //     options::Variant::Encrypt(options) => encrypt::execute(options),

    //     options::Variant::Decrypt(options) => decrypt::execute(options),
    // };
}
