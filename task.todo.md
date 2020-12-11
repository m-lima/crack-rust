## UI
- [X] Show progress (based on atomic for CPU and on queue for GPU) (use stderr)
- [X] Time taken

## Input
- [X] Get input from stdin
- [X] Get input from file(s)
- [ ] *Accept multiple lengths and prefixes*

## OpenCL
- [X] **GPU**
- [X] Optimize SHA256 (make assumptions)
- [X] ~~Share GLSL code between algorithms (structs, prepare, search)~~

## Design
- [X] Make options/args self-contained
- [ ] **Now that the algorithm is typed, go over the code and reduce the runtime dispatching**
- [ ] print::io_* is printing colored and out of place (no section and for writes, it comes before Summary on -vv)

## Refactor
- [ ] Rename encrypt/decrypt to hash/crack (the idea is to open up for rc4 encrypt/decrypt)
- [ ] Create trait for reporting back results
  - [ ] Merge with the summary module (make it a single reporting module)
  - [ ] Bring the hash::Pair into the reporting module
- [ ] Remove the extra error creation macro
- [ ] Group modes by only encrypt/decrypt and nest the algorithm
  - [ ] Have only two subcommands (hash and crack)
  - [ ] Infer algorithm from input? (tricky if loading from files)
  - [X] options::Mode should have only two variants
- [ ] Remove result from summary
  - [ ] Detect if not all hashes were cracked
- [ ] Consider not returning a Result from the error macro
- [X] Move encrypt to a directory
- [X] Move printer to cli module
- [X] Move SALT_ENV from options::mod to cli::args
- [X] Move options::parse to cli::args::parse
- [X] Move args to cli module
- [X] Move OPTIMAL_CPU to cpu module
- [X] Validate that clap is reading SALT_ENV
- [X] Move HashPair to hash module
- [X] Remove hash/encrypt summary
- [X] Move file saving out of decrypt module
- [X] Avoid repeated code in cli::args::{compose_crack, compose_hash} for the `new` function calls
- [X] Move cli methods from crate::files
