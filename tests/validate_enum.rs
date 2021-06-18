use binver::{ReadConfig, Serializable};

#[derive(Serializable, Debug, PartialEq)]
pub enum Test {
    #[since(1.0.0)]
    Variant1,

    #[since(2.0.0)]
    Variant2 {
        #[since(3.0.0)]
        name: String,
    },
}

#[test]
fn test_serialize_simple() {
    // serialize a v3.0.0 struct
    let serialized = binver::to_vec(&Test::Variant1);
    assert_eq!(
        vec![
            0, 3, // semver major
            0, 0, // semver minor
            0, 0, // semver patch
            0, 0, // Variant1
        ],
        serialized
    );
    let serialized = binver::to_vec(&Test::Variant2 {
        name: String::from("Trangar"),
    });
    assert_eq!(
        vec![
            0, 3, // semver major
            0, 0, // semver minor
            0, 0, // semver patch
            0, 1, // Variant2
            0, 0, 0, 7, b'T', b'r', b'a', b'n', b'g', b'a', b'r', // name
        ],
        serialized
    );
    let config = ReadConfig {
        error_on_trailing_bytes: true,
    };

    let deserialized: Test = binver::deserialize_slice_with_config(&serialized, config).unwrap();
    assert_eq!(
        deserialized,
        Test::Variant2 {
            name: String::from("Trangar"),
        }
    );
}

#[test]
fn test_simple_deserialize() {
    let config = ReadConfig {
        error_on_trailing_bytes: true,
    };
    // Deserialize a v2.0.0 struct

    // variant 1
    let mut vec = Vec::<u8>::new();
    vec.extend_from_slice(&(2u16.to_be_bytes())); // semver major
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver minor
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver patch

    vec.extend_from_slice(&(0u16.to_be_bytes())); // Variant1

    let deserialized: Test = binver::deserialize_slice_with_config(&vec, config.clone()).unwrap();
    assert_eq!(deserialized, Test::Variant1);

    // variant 2
    // note that field is version 3, so it's always empty
    let mut vec = Vec::<u8>::new();
    vec.extend_from_slice(&(2u16.to_be_bytes())); // semver major
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver minor
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver patch

    vec.extend_from_slice(&(1u16.to_be_bytes())); // Variant2

    let deserialized: Test = binver::deserialize_slice_with_config(&vec, config.clone()).unwrap();
    assert_eq!(
        deserialized,
        Test::Variant2 {
            name: String::new()
        }
    );

    // variant 2, version 3
    // now name has a value
    let mut vec = Vec::<u8>::new();
    vec.extend_from_slice(&(3u16.to_be_bytes())); // semver major
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver minor
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver patch

    vec.extend_from_slice(&(1u16.to_be_bytes())); // Variant2
    vec.extend_from_slice(&(7u32.to_be_bytes())); // name length
    vec.extend_from_slice(b"Trangar"); // name

    let deserialized: Test = binver::deserialize_slice_with_config(&vec, config.clone()).unwrap();
    assert_eq!(
        deserialized,
        Test::Variant2 {
            name: String::from("Trangar")
        }
    );
}

#[test]
fn test_deserialize_upgrade_version() {
    let config = ReadConfig {
        error_on_trailing_bytes: true,
    };
    // Deserialize a v1.0.0 struct into v2.0.0
    let mut vec = Vec::<u8>::new();
    vec.extend_from_slice(&(1u16.to_be_bytes())); // semver major
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver minor
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver patch

    vec.extend_from_slice(&(0u16.to_be_bytes())); // Variant1

    let deserialized: Test = binver::deserialize_slice_with_config(&vec, config.clone()).unwrap();

    assert_eq!(deserialized, Test::Variant1);
}

#[test]
#[should_panic]
fn test_deserialize_upgrade_unknown_variant() {
    let config = ReadConfig {
        error_on_trailing_bytes: true,
    };

    // Deserialize a v1.0.0 struct into v2.0.0
    // but we try to deserialize a variant that's being introduced in 2.0.0
    // This should fail
    let mut vec = Vec::<u8>::new();
    vec.extend_from_slice(&(1u16.to_be_bytes())); // semver major
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver minor
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver patch

    vec.extend_from_slice(&(1u16.to_be_bytes())); // Variant2
    vec.extend_from_slice(&(7u32.to_be_bytes())); // name length
    vec.extend_from_slice(b"Trangar"); // name

    // This should fail
    binver::deserialize_slice_with_config::<Test>(&vec, config).unwrap();
}
