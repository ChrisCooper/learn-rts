# learn-rts
An RTS where you actually train your units' neural networks


## Running the program
```
cargo run -p rts
# or
$env:RUST_BACKTRACE=1; cargo run; $env:RUST_BACKTRACE=''
```

## Running tests
```
cargo test
```


## Notes

For Rust--Python interop, candidates include Apache Arrow, the PyO3 crate, and the rust-numpy crate to fill numpy arrays from Rust