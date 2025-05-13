# vep-noodles
A set of WASM wrappers for a VEP tool. The purpose is to show how portable WASM can be and how good bioinformatic tools

## No noodle!

Could not figure out file reading by line. Need to come back to this

## WIT
No `&mut self` for wit-bindgen :(

## Features

- Provide progrmmatic access to CSQ value for a given variant 
- Provide start of variant list
- subset provided vcf

## Targets

- [x] wasm cli - part of core
- [ ] python import - need WIT!
- [ ] html
- [ ] web service?

## Links 

- [WIT](https://component-model.bytecodealliance.org/introduction.html)
- [WIT cheat sheet](https://cosmonic.com/blog/engineering/wit-cheat-sheet)
- [WASMTIME integration](https://docs.rs/wasmtime/latest/wasmtime/index.html)
- [wasm-tools](https://github.com/bytecodealliance/wasm-tools)
