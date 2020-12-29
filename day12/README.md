For this day, I am implementing the solution as a python module, implemented in Rust, using PyO3.
This is a bit superfluous, but it's a good way to practice using PyO3.

## Build the python module `day12` from Rust using PyO3

```
cargo build --release
```

## Run rust tests

```
cargo test --no-default-features
```

See https://github.com/PyO3/pyo3/issues/340, https://github.com/rust-lang/rust/issues/25289 for discussion of why this is needed with PyO3.

## Solve the Advent of Code problem from python

```
python main.py
```
