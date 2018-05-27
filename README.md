# is-travis
Check if your code is running on Travis CI

## Install
Specify the dependencty in Cargo.toml:

```yaml
[dependencies]
is-travis = "~1.0.0"
```

Fetch it with cargo:
```bash
$ cargo build
```

## Usage

```rust
extern crate is_travis;
use is_travis::is_travis;

println!("{}", is_travis()); // prints false on your PC, true on Travis CI
```

## About
### License
Copyright © 2018, [nukeop](https://github.com/nukeop).
Released under the [MIT License](LICENSE).
