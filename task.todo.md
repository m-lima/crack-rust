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
- [ ] Remove result from summary
- [ ] Remove hash/encrypt summary
- [ ] Move HashPair to hash module
- [ ] Move OPTIMAL_CPU to cpu module
- [ ] Move printer to cli module
- [ ] Create trait for reporting back results
- [X] Move encrypt to a directory
