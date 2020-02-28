use clap::value_t;

macro_rules! algorithm {
    (options::Algorithm::MD5) => {
        "MD5"
    };
    (options::Algorithm::SHA256) => {
        "SHA256"
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

#[derive(Copy, Clone)]
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
            ArgField::Short => "d",
            ArgField::Parameter => "DEVICE",
        }
    };
    (_Arg::Verbose) => {
        "v"
    };
}

// Allowed because the count was checked for overflow
#[allow(clippy::cast_possible_truncation)]
fn get_optimal_thread_count(requested_count: u8, number_space: u64) -> u8 {
    let thread_count = std::cmp::min(
        number_space / super::OPTIMAL_HASHES_PER_THREAD + 1,
        if requested_count == 0 {
            let cores = num_cpus::get();
            if cores > usize::from(u8::max_value()) {
                eprintln!("Too many cores.. You have one powerful computer!");
                std::process::exit(-1);
            }
            cores as u64
        } else {
            u64::from(requested_count)
        },
    );

    // Due to `min`, it will always be less than u8::MAX (255)
    thread_count as u8
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
                .possible_values(&super::Device::variants())
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
                .possible_values(&super::Algorithm::variants())
                .case_insensitive(true)
                .global(true),
        )
        .arg({
            let salt = clap::Arg::with_name(arg!(_Arg::Salt, ArgField::Name))
                .long(arg!(_Arg::Salt, ArgField::Name))
                .short(arg!(_Arg::Salt, ArgField::Short))
                .value_name(arg!(_Arg::Salt, ArgField::Parameter))
                .help("Salt to use")
                .takes_value(true)
                .global(true);
            if crate::secrets::SALT.is_empty() {
                salt
            } else {
                salt.default_value(crate::secrets::SALT)
            }
        })
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

fn parse_verboseness(matches: &clap::ArgMatches<'_>) -> super::Verboseness {
    match matches.occurrences_of(arg!(_Arg::Verbose)) {
        2 => super::Verboseness::High,
        1 => super::Verboseness::Low,
        0 | _ => super::Verboseness::None,
    }
}

fn parse_shared_args(matches: &clap::ArgMatches<'_>) -> super::Shared {
    super::Shared {
        algorithm: value_t!(
            matches,
            arg!(_Arg::Algorithm, ArgField::Name),
            super::Algorithm
        )
        .unwrap(),
        salt: String::from(matches.value_of(arg!(_Arg::Salt, ArgField::Name)).unwrap()),
        input: get_input(matches),
        verboseness: parse_verboseness(&matches),
    }
}

fn parse_encrypt(matches: &clap::ArgMatches<'_>) -> super::Mode {
    super::Mode::Encrypt(super::Encrypt {
        shared: parse_shared_args(&matches),
    })
}

fn parse_decrypt(matches: &clap::ArgMatches<'_>) -> super::Mode {
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
        eprintln!("Prefix is too long");
        std::process::exit(-1);
    }

    // Allowed because the length was checked for overflow
    #[allow(clippy::cast_possible_truncation)]
    let length = total_length - prefix.len() as u8;
    let number_space = 10_u64.pow(u32::from(length));
    let thread_count = get_optimal_thread_count(
        matches
            .value_of(arg!(_Arg::ThreadCount, ArgField::Name))
            .unwrap()
            .parse::<u8>()
            .unwrap(),
        number_space,
    );
    let device = value_t!(matches, arg!(_Arg::Device, ArgField::Name), super::Device)
        .unwrap_or_else(|_| {
            if number_space > u64::from(thread_count) * super::OPTIMAL_HASHES_PER_THREAD {
                super::Device::GPU
            } else {
                super::Device::CPU
            }
        });

    // let files = Vec::<String>::new();

    super::Mode::Decrypt(super::Decrypt {
        shared,
        thread_count,
        length,
        number_space,
        prefix,
        device,
    })
}

pub fn parse() -> super::Mode {
    let matches = setup();
    match matches.subcommand() {
        (cmd!(_Command::Encrypt), Some(sub_matches)) => parse_encrypt(&sub_matches),
        (cmd!(_Command::Decrypt), Some(sub_matches)) => parse_decrypt(&sub_matches),
        _ => unreachable!(),
    }
}
