# Learning Rust


Picking up Rust studies for the second time.

## Getting a dev environment:
```
docker pull rust
docker run --name='rust_container' --hostname='rust_container' -di rust:latest
docker exec -it rust_container bash
```

## Building and executing code:

```
cargo run

# compile packages without the final step of code generation:
cargo check 


cargo doc --open

# with an env var:
RUST_LOG=info cargo run

cargo run -- --other --args
```

```
cargo build --release

```


Add package using cargo:
```
cargo add clap --features derive
```

## Useful links:
https://doc.rust-lang.org/stable/book/
https://nnethercote.github.io/perf-book/
https://rust-unofficial.github.io/patterns/
https://rust-unofficial.github.io/too-many-lists/
https://doc.rust-lang.org/stable/rust-by-example/
https://doc.rust-lang.org/nightly/nomicon/
https://marabos.nl/atomics/foreword.html

build an interpreter in Rust, inspired by Nystrom (not finished yet):
https://rust-hosted-langs.github.io/book/


https://adventofcode.com/2015
https://adventofcode.com/2015/day/1


## Interesting repos:

From command line Rust:
https://github.com/kyclark/command-line-rust

From Atomics and locks:
https://github.com/m-ou-se/rust-atomics-and-locks/tree/main/src

## Tooling (mostly VSCode) comments and links:


Rust analyzer not working? Add the proper path to the config in `settings.json`:
```
    "rust-analyzer.linkedProjects": [
        "/home/klundert/rust/basics/Cargo.toml"
    ]
```

https://rust-analyzer.github.io/manual.html