[![Crates.io](https://img.shields.io/crates/d/binver)](https://crates.io/crates/binver)
[![Build status](https://github.com/trangar/binver/actions/workflows/rust.yml/badge.svg)](https://github.com/Trangar/binver/actions)
[![docs.io](https://docs.rs/binver/badge.svg)](https://docs.rs/binver)
[![codecov](https://codecov.io/gh/Trangar/binver/branch/main/graph/badge.svg?token=tYaAvN3Oja)](https://codecov.io/gh/Trangar/binver)

# binver

Binary (de)serialization framework that is backwards compatible with versioned fields.

```rust

#[derive(Serializable, PartialEq, Debug)]
pub struct Player {
    // This field has existed since binary version 0.0.1
    #[since(0.0.1)]
    pub id: u32,

    // In 0.0.2 we introduced a new field
    // When loading a serialized 0.0.1 object, this field will have it's `Default` value
    #[since(0.0.2)]
    pub name: String,
}

let player = Player {
    id: 5,
    name: String::from("foo")
};

let serialized = binver::to_vec(&player);
let deserialized_player = binver::deserialize_slice(&serialized).unwrap();

assert_eq!(player, deserialized_player);
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
