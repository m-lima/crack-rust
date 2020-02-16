use clap::value_t;

use crate::options;
use crate::print;

macro_rules! algorithm {
    (options::Algorithm::MD5) => {
        "MD5"
    };
    (options::Algorithm::SHA256) => {
        "SHA256"
    };
}

macro_rules! device {
    (options::Device::CPU) => {
        "CPU"
    };
    (options::Device::GPU) => {
        "GPU"
    };
}

enum _Command {
    Encrypt,
    Decrypt,
}

macro_rules! cmd {
    (_Command::Encrypt) => {
        "encrypt"
    };
    (_Command::Decrypt) => {
        "decrypt"
    };
}

enum _Arg {
    Algorithm,
    // File,
    Input,
    Length,
    Prefix,
    Salt,
    ThreadCount,
    Device,
    Verbose,
}

enum ArgField {
    Name,
    Short,
    Parameter,
}

macro_rules! arg {
    (_Arg::Algorithm, $f:path) => {
        match $f {
            ArgField::Name => "algorithm",
            ArgField::Short => "a",
            ArgField::Parameter => "ALGORITHM",
        }
    };
    // (_Arg::File, $f:path) => {
    //     match $f {
    //         ArgField::Name => "file",
    //         ArgField::Short => "f",
    //         ArgField::Parameter => "FILE",
    //     }
    // };
    (_Arg::Input, $f:path) => {
        match $f {
            ArgField::Name => "input",
            ArgField::Short => "i",
            ArgField::Parameter => "INPUT",
        }
    };
    (_Arg::Length, $f:path) => {
        match $f {
            ArgField::Name => "length",
            ArgField::Short => "l",
            ArgField::Parameter => "LENGTH",
        }
    };
    (_Arg::Prefix, $f:path) => {
        match $f {
            ArgField::Name => "prefix",
            ArgField::Short => "p",
            ArgField::Parameter => "PREFIX",
        }
    };
    (_Arg::Salt, $f:path) => {
        match $f {
            ArgField::Name => "salt",
            ArgField::Short => "s",
            ArgField::Parameter => "SALT",
        }
    };
    (_Arg::ThreadCount, $f:path) => {
        match $f {
            ArgField::Name => "thread-count",
            ArgField::Short => "n",
            ArgField::Parameter => "COUNT",
        }
    };
    (_Arg::Device, $f:path) => {
        match $f {
            ArgField::Name => "device",
            ArgField::Short => "c",
            ArgField::Parameter => "DEVICE",
        }
    };
    (_Arg::Verbose) => {
        "v"
    };
}

fn create_app<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new("Cracker")
        .version("0.1")
        .author("Marcelo Lima")
        .about("MD5 and SHA256 cracker")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
}

fn setup_decrypt<'a, 'b>() -> clap::App<'a, 'b> {
    clap::SubCommand::with_name(cmd!(_Command::Decrypt))
        .about("Attempts to crack the given input")
        .arg(
            clap::Arg::with_name(arg!(_Arg::Input, ArgField::Name))
                .long(arg!(_Arg::Input, ArgField::Name))
                .short(arg!(_Arg::Input, ArgField::Short))
                .value_name(arg!(_Arg::Input, ArgField::Parameter))
                .help("Input values to crack")
                .takes_value(true)
                .multiple(true)
                // .required_unless(arg!(_Arg::File, ArgField::Name)),
                .required(true),
        )
        // .arg(
        //     clap::Arg::with_name(arg!(_Arg::File, ArgField::Name))
        //         .long(arg!(_Arg::File, ArgField::Name))
        //         .short(arg!(_Arg::File, ArgField::Short))
        //         .value_name(arg!(_Arg::File, ArgField::Parameter))
        //         .help("Path to a file containing hashes to crack")
        //         .takes_value(true)
        //         .multiple(true)
        //         .validator(|v| {
        //             let path = std::path::Path::new(&v);
        //             if path.exists() && path.is_file() {
        //                 Ok(())
        //             } else {
        //                 Err(String::from(format!("\"{}\" is not a file", v)))
        //             }
        //         })
        //         .required_unless(arg!(_Arg::Input, ArgField::Name)),
        // )
        .arg(
            clap::Arg::with_name(arg!(_Arg::Prefix, ArgField::Name))
                .long(arg!(_Arg::Prefix, ArgField::Name))
                .short(arg!(_Arg::Prefix, ArgField::Short))
                .value_name(arg!(_Arg::Prefix, ArgField::Parameter))
                .help("Known prefix of hashed value")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name(arg!(_Arg::ThreadCount, ArgField::Name))
                .long(arg!(_Arg::ThreadCount, ArgField::Name))
                .short(arg!(_Arg::ThreadCount, ArgField::Short))
                .value_name(arg!(_Arg::ThreadCount, ArgField::Parameter))
                .help("Number of threads to spawn (0 for auto)")
                .takes_value(true)
                .default_value("0")
                .validator(|v| v.parse::<u8>().map(|_| ()).map_err(|e| e.to_string())),
        )
        .arg(
            clap::Arg::with_name(arg!(_Arg::Device, ArgField::Name))
                .long(arg!(_Arg::Device, ArgField::Name))
                .short(arg!(_Arg::Device, ArgField::Short))
                .value_name(arg!(_Arg::Device, ArgField::Parameter))
                .help("Device to run in [GPU, CPU]")
                .takes_value(true)
                .default_value(device!(options::Device::GPU))
                .possible_values(&options::Device::variants())
                .case_insensitive(true),
        )
        .arg(
            clap::Arg::with_name(arg!(_Arg::Length, ArgField::Name))
                .long(arg!(_Arg::Length, ArgField::Name))
                .short(arg!(_Arg::Length, ArgField::Short))
                .value_name(arg!(_Arg::Length, ArgField::Parameter))
                .help("Length of hashed value")
                .takes_value(true)
                .default_value("12")
                .validator(|v| {
                    v.parse::<u8>().map_err(|e| e.to_string()).and_then(|v| {
                        if v > 0 {
                            Ok(())
                        } else {
                            Err(format!(
                                "{} must be a positive integer",
                                arg!(_Arg::Length, ArgField::Name)
                            ))
                        }
                    })
                }),
        )
}

fn setup_encrypt<'a, 'b>() -> clap::App<'a, 'b> {
    clap::SubCommand::with_name(cmd!(_Command::Encrypt))
        .about("Hashes the given input")
        .arg(
            clap::Arg::with_name(arg!(_Arg::Input, ArgField::Name))
                .long(arg!(_Arg::Input, ArgField::Name))
                .short(arg!(_Arg::Input, ArgField::Short))
                .value_name(arg!(_Arg::Input, ArgField::Parameter))
                .help("Input values to hash")
                .takes_value(true)
                .multiple(true)
                .required(true),
        )
}

fn setup<'a>() -> clap::ArgMatches<'a> {
    create_app()
        .arg(
            clap::Arg::with_name(arg!(_Arg::Algorithm, ArgField::Name))
                .long(arg!(_Arg::Algorithm, ArgField::Name))
                .short(arg!(_Arg::Algorithm, ArgField::Short))
                .value_name(arg!(_Arg::Algorithm, ArgField::Parameter))
                .help("Hashing algorithm")
                .takes_value(true)
                .default_value(algorithm!(options::Algorithm::SHA256))
                .possible_values(&options::Algorithm::variants())
                .case_insensitive(true)
                .global(true),
        )
        .arg(
            clap::Arg::with_name(arg!(_Arg::Salt, ArgField::Name))
                .long(arg!(_Arg::Salt, ArgField::Name))
                .short(arg!(_Arg::Salt, ArgField::Short))
                .value_name(arg!(_Arg::Salt, ArgField::Parameter))
                .help("Salt to use")
                .takes_value(true)
                .default_value(crate::secrets::SALT)
                .global(true),
        )
        .arg(
            clap::Arg::with_name(arg!(_Arg::Verbose))
                .short(arg!(_Arg::Verbose))
                .help("Verbose")
                .multiple(true)
                .global(true),
        )
        .subcommand(setup_decrypt())
        .subcommand(setup_encrypt())
        .get_matches()
}

fn get_input(matches: &clap::ArgMatches<'_>) -> Vec<String> {
    matches
        .values_of(arg!(_Arg::Input, ArgField::Name))
        .unwrap()
        .map(String::from)
        .collect()
}

fn parse_verboseness(matches: &clap::ArgMatches<'_>) -> print::Verboseness {
    match matches.occurrences_of(arg!(_Arg::Verbose)) {
        2 => print::Verboseness::High,
        1 => print::Verboseness::Low,
        0 | _ => print::Verboseness::None,
    }
}

fn parse_shared_args(matches: &clap::ArgMatches<'_>) -> options::Shared {
    options::Shared {
        algorithm: value_t!(
            matches,
            arg!(_Arg::Algorithm, ArgField::Name),
            options::Algorithm
        )
        .unwrap(),
        salt: String::from(matches.value_of(arg!(_Arg::Salt, ArgField::Name)).unwrap()),
        input: get_input(matches),
    }
}

fn parse_encrypt(matches: &clap::ArgMatches<'_>) -> (options::Mode, print::Verboseness) {
    (
        options::Mode::Encrypt(options::Encrypt {
            shared: parse_shared_args(&matches),
        }),
        parse_verboseness(&matches),
    )
}

fn parse_decrypt(matches: &clap::ArgMatches<'_>) -> (options::Mode, print::Verboseness) {
    let shared = parse_shared_args(&matches);

    let prefix = String::from(
        matches
            .value_of(arg!(_Arg::Prefix, ArgField::Name))
            .unwrap_or(""),
    );
    let total_length = matches
        .value_of(arg!(_Arg::Length, ArgField::Name))
        .unwrap()
        .parse::<u8>()
        .unwrap();
    if prefix.len() > usize::from(total_length) {
        panic!("Prefix is too long");
    }

    // Allowed because the length was checked for overflow
    #[allow(clippy::cast_possible_truncation)]
    let length = total_length - prefix.len() as u8;
    let number_space = 10_u64.pow(u32::from(length));
    let thread_count = matches
        .value_of(arg!(_Arg::ThreadCount, ArgField::Name))
        .unwrap()
        .parse::<u8>()
        .unwrap();
    let device = value_t!(matches, arg!(_Arg::Device, ArgField::Name), options::Device).unwrap();

    // let files = Vec::<String>::new();

    (
        options::Mode::Decrypt(options::Decrypt {
            shared,
            thread_count,
            length,
            number_space,
            prefix,
            device,
        }),
        parse_verboseness(&matches),
    )
}

pub fn parse() -> (options::Mode, print::Verboseness) {
    let matches = setup();
    match matches.subcommand() {
        (cmd!(_Command::Encrypt), Some(sub_matches)) => parse_encrypt(&sub_matches),
        (cmd!(_Command::Decrypt), Some(sub_matches)) => parse_decrypt(&sub_matches),
        _ => unreachable!(),
    }
}
