# AOC 2022

This year I'll be trying out Rust!!

### Get version
`rustc --version`

### Update
`rustup update`

### Running
#### Compile first
`rustc main.rs`
#### Run it
`./main`

```
➜  aoc2022 cd day01
➜  day01 rustc main.rs
➜  day01 ./main
Hello, world!
➜  day01
```

#### Cargo
Let’s recap what we’ve learned so far about Cargo:

We can create a project using `cargo new`.
We can build a project using `cargo build`.
We can build and run a project in one step using `cargo run`.
We can build a project without producing a binary to check for errors using `cargo check`.
Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.
