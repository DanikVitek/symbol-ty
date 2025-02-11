# `symbol-ty`

[![Crates.io](https://img.shields.io/crates/v/symbol-ty)](https://crates.io/crates/symbol-ty)
[![Docs.rs](https://docs.rs/symbol-ty/badge.svg)](https://docs.rs/symbol-ty)
[![Crates.io MSRV](https://img.shields.io/crates/msrv/symbol-ty)](https://crates.io/crates/symbol-ty)
[![Crates.io Downloads](https://img.shields.io/crates/d/symbol-ty)](https://crates.io/crates/symbol-ty)
[![Crates.io License](https://img.shields.io/crates/l/symbol-ty)](https://crates.io/crates/symbol-ty)

A simple crate that allows to create a type that represents a unique symbol.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
symbol-ty = "0.2"
```

## Example

```rust
use symbol_ty::Symbol;

fn main() {
    let symbol1 = <Symbol!("foo")>::new();
    let symbol2 = <Symbol!("foo")>::new();

    assert_eq!(symbol1, symbol2);
    
    println!("{}", symbol1);
    println!("{:?}", symbol1);
}

struct Foo {
    foo: Symbol!("foo"),
}
```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
