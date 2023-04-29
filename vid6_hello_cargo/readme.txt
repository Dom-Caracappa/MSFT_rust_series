From video 6 of 35

https://learn.microsoft.com/en-us/shows/beginners-series-to-rust/what-is-cargo-6-of-35--beginners-series-to-rust


❯ cargo new cargo-hello-world
     Created binary (application) `cargo-hello-world` package

MSFT_rust_series/hello_cargo [?] 
❯ ls
cargo-hello-world       readme.txt

MSFT_rust_series/hello_cargo [?] 
❯ cd cargo-hello-world 

MSFT_rust_series/hello_cargo/cargo-hello-world [?] is 📦 v0.1.0 via 🦀 v1.69.0 
❯ ls
Cargo.toml      src

MSFT_rust_series/hello_cargo/cargo-hello-world [?] is 📦 v0.1.0 via 🦀 v1.69.0 
❯ cargo build
   Compiling cargo-hello-world v0.1.0 (/Users/pope/Dev/MSFT_rust_series/MSFT_rust_series/hello_cargo/cargo-hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.56s

MSFT_rust_series/hello_cargo/cargo-hello-world [?] is 📦 v0.1.0 via 🦀 v1.69.0 
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/cargo-hello-world`
Hello, world!

MSFT_rust_series/hello_cargo/cargo-hello-world [?] is 📦 v0.1.0 via 🦀 v1.69.0 
❯ cargo test
   Compiling cargo-hello-world v0.1.0 (/Users/pope/Dev/MSFT_rust_series/MSFT_rust_series/hello_cargo/cargo-hello-world)
    Finished test [unoptimized + debuginfo] target(s) in 0.22s
     Running unittests src/main.rs (target/debug/deps/cargo_hello_world-b97eb9402c521ee2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
