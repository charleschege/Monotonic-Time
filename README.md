## Monotonic-Time

A simple lightweight crate to convert seconds provided to `Coordinated Universal Time` (UTC) or ` Temps Atomique International` (TAI). 

[![Rust](https://github.com/charleschege/Monotonic-Time/actions/workflows/rust.yml/badge.svg)](https://github.com/charleschege/Monotonic-Time/actions/workflows/rust.yml)
![crates.io](https://img.shields.io/crates/v/monotonic-time.svg)


#### Instantiate a DateTime

```rust
use monotonic_time::DateTime;

let mut datetime = DateTime::new();
```

#### Convert Seconds to UTC

```rust
use monotonic_time::DateTime;

let now = 1656603896;
let mut datetime = DateTime::new();
datetime.to_datetime(now);
println!("{}", datetime)
```

#### Convert Seconds to TAI

```rust
use monotonic_time::DateTime;

let now = 1656603896;
let mut datetime = DateTime::new();
// Pass in the seconds and the offset of UTC from TAI
datetime.to_taitime(now, 37);
println!("{}", datetime)
```



#### LICENSING

This crate is Licensed under `Apache-2.0` or `MIT`

CONTRIBUTING

All contributions must adhere to the crate's code and the Rust Code of Conduct - https://www.rust-lang.org/policies/code-of-conduct