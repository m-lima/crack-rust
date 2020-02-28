Hasher [![Github](https://github.com/m-lima/hasher/workflows/build/badge.svg)](https://github.com/m-lima/hasher/actions?workflow=build)
========
#### A MD5 and Sha256 hasher / dehasher

## Building

### Installing rust

#### Linux / OSX
```bash
$ curl https://sh.rustup.rs -sSf | sh
```

#### Windows
```bash
$ scoop install rustup
```

### Compiling
```bash
$ cd <this repository>
$ cargo build --release
```

## Running
```bash
USAGE:
    hasher [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -v               Verbose
    -V, --version    Prints version information

OPTIONS:
    -a, --algorithm <ALGORITHM>    Hashing algorithm [default: SHA256]  [possible values: MD5, SHA256]
    -s, --salt <SALT>              Salt to use

SUBCOMMANDS:
    decrypt    Attempts to crack the given input
    encrypt    Hashes the given input
    help       Prints this message or the help of the given subcommand(s)
```

### Encrypt
```bash
USAGE:
    hasher encrypt [FLAGS] [OPTIONS] --input <INPUT>...

FLAGS:
    -h, --help       Prints help information
    -v               Verbose
    -V, --version    Prints version information

OPTIONS:
    -a, --algorithm <ALGORITHM>    Hashing algorithm [default: SHA256]  [possible values: MD5, SHA256]
    -i, --input <INPUT>...         Input values to hash
    -s, --salt <SALT>              Salt to use
```

### Decrypt
```bash
USAGE:
    hasher decrypt [FLAGS] [OPTIONS] --input <INPUT>...

FLAGS:
    -h, --help       Prints help information
    -v               Verbose
    -V, --version    Prints version information

OPTIONS:
    -a, --algorithm <ALGORITHM>    Hashing algorithm [default: SHA256]  [possible values: MD5, SHA256]
    -d, --device <DEVICE>          Device to run in [GPU, CPU] [possible values: CPU, GPU]
    -i, --input <INPUT>...         Input values to crack
    -l, --length <LENGTH>          Length of hashed value [default: 12]
    -p, --prefix <PREFIX>          Known prefix of hashed value
    -s, --salt <SALT>              Salt to use
    -n, --thread-count <COUNT>     Number of threads to spawn (0 for auto) [default: 0]
```
