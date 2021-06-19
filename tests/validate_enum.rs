use binver::{ReadConfig, Serializable};

#[derive(Serializable, Debug, PartialEq)]
pub enum Test<'a> {
    #[since(0.0.1)]
    Variant1,

    #[since(0.0.2)]
    Variant2 {
        #[since(0.0.3)]
        name: &'a str,
    },
}

#[test]
fn test_serialize_simple() {
    let mut serialized = [0u8; 1024];

    // serialize a v0.0.3 struct
    let length = binver::write_to_slice(&mut serialized, &Test::Variant1);

    assert_eq!(
        &[0, 0],          // Variant 1
        &serialized[..length][6..]  // ignore the version bytes
    );
    let length = binver::write_to_slice(&mut serialized, &Test::Variant2 { name: "Trangar" });
    assert_eq!(
        vec![
            0, 1, // Variant2
            0, 0, 0, 7, b'T', b'r', b'a', b'n', b'g', b'a', b'r', // name
        ],
        &serialized[..length][6..] // ignore the version bytes
    );
    let config = ReadConfig {
        error_on_trailing_bytes: true,
    };

    let deserialized: Test =
        binver::deserialize_slice_with_config(&serialized[..length], config).unwrap();
    assert_eq!(deserialized, Test::Variant2 { name: "Trangar" });
}

#[test]
fn test_simple_deserialize() {
    let config = ReadConfig {
        error_on_trailing_bytes: true,
    };
    // Deserialize a v2.0.0 struct

    // variant 1
    let mut vec = Vec::<u8>::new();
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver major
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver minor
    vec.extend_from_slice(&(2u16.to_be_bytes())); // semver patch

    vec.extend_from_slice(&(0u16.to_be_bytes())); // Variant1

    let deserialized: Test = binver::deserialize_slice_with_config(&vec, config.clone()).unwrap();
    assert_eq!(deserialized, Test::Variant1);

    // variant 2
    // note that field is version 3, so it's always empty
    let mut vec = Vec::<u8>::new();
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver major
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver minor
    vec.extend_from_slice(&(2u16.to_be_bytes())); // semver patch

    vec.extend_from_slice(&(1u16.to_be_bytes())); // Variant2

    let deserialized: Test = binver::deserialize_slice_with_config(&vec, config.clone()).unwrap();
    assert_eq!(deserialized, Test::Variant2 { name: "" });

    // variant 2, version 3
    // now name has a value
    let mut vec = Vec::<u8>::new();
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver major
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver minor
    vec.extend_from_slice(&(3u16.to_be_bytes())); // semver patch

    vec.extend_from_slice(&(1u16.to_be_bytes())); // Variant2
    vec.extend_from_slice(&(7u32.to_be_bytes())); // name length
    vec.extend_from_slice(b"Trangar"); // name

    let deserialized: Test = binver::deserialize_slice_with_config(&vec, config.clone()).unwrap();
    assert_eq!(deserialized, Test::Variant2 { name: "Trangar" });
}

#[test]
fn test_deserialize_upgrade_version() {
    let config = ReadConfig {
        error_on_trailing_bytes: true,
    };
    // Deserialize a v1.0.0 struct into v2.0.0
    let mut vec = Vec::<u8>::new();
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver major
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver minor
    vec.extend_from_slice(&(1u16.to_be_bytes())); // semver patch

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
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver major
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver minor
    vec.extend_from_slice(&(1u16.to_be_bytes())); // semver patch

    vec.extend_from_slice(&(1u16.to_be_bytes())); // Variant2
    vec.extend_from_slice(&(7u32.to_be_bytes())); // name length
    vec.extend_from_slice(b"Trangar"); // name

    // This should fail
    binver::deserialize_slice_with_config::<Test>(&vec, config).unwrap();
}
