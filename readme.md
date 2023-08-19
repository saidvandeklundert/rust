# Learning Rust


Picking up Rust studies for the second time.

## Getting a dev environment:
```
docker pull rust
docker run --name='rust_container' --hostname='rust_container' -di rust:latest
docker exec -it rust_container bash
```


## Useful links:
https://doc.rust-lang.org/stable/book/
https://nnethercote.github.io/perf-book/
https://rust-unofficial.github.io/patterns/
https://rust-unofficial.github.io/too-many-lists/
https://doc.rust-lang.org/stable/rust-by-example/
https://doc.rust-lang.org/nightly/nomicon/
https://marabos.nl/atomics/foreword.html

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