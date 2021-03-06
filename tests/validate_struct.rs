use binver::Serializable;

#[derive(Serializable)]
pub struct Test<'a> {
    #[since(0.0.1)]
    pub id: u32,
    #[since(0.0.2)]
    pub name: &'a str,
}

#[test]
fn test_serialize_simple() {
    // serialize a v2.0.0 struct
    let test = Test {
        id: 5,
        name: "Trangar",
    };
    let mut serialized = [0u8; 1024];
    let length = binver::write_to_slice(&mut serialized, &test).unwrap();
    assert_eq!(
        vec![
            0, 0, 0, 5, // id
            0, 0, 0, 7, b'T', b'r', b'a', b'n', b'g', b'a', b'r', // name
        ],
        &serialized[..length][6..] // Ignore version bytes
    );
}

#[test]
fn test_simple_deserialize() {
    // Deserialize a v2.0.0 struct
    let mut slice = [0u8; 21];
    slice[0..2].copy_from_slice(&(0u16.to_be_bytes())); // semver major
    slice[2..4].copy_from_slice(&(0u16.to_be_bytes())); // semver minor
    slice[4..6].copy_from_slice(&(2u16.to_be_bytes())); // semver patch

    slice[6..10].copy_from_slice(&(1u32.to_be_bytes())); // ID

    slice[10..14].copy_from_slice(&(7u32.to_be_bytes())); // name length
    slice[14..].copy_from_slice(b"Trangar"); // name

    let deserialized: Test = binver::deserialize_slice(&slice).unwrap();

    assert_eq!(deserialized.id, 1);
    assert_eq!(deserialized.name, "Trangar");
}

#[test]
fn test_deserialize_upgrade_version() {
    // Deserialize a v1.0.0 struct into v2.0.0
    let mut slice = [0u8; 10];
    slice[0..2].copy_from_slice(&(0u16.to_be_bytes())); // semver major
    slice[2..4].copy_from_slice(&(0u16.to_be_bytes())); // semver minor
    slice[4..6].copy_from_slice(&(1u16.to_be_bytes())); // semver patch

    slice[6..].copy_from_slice(&(1u32.to_be_bytes())); // ID

    let deserialized: Test = binver::deserialize_slice(&slice).unwrap();

    assert_eq!(deserialized.id, 1);
    assert_eq!(deserialized.name, "");
}

#[cfg(feature = "std")]
#[derive(Serializable)]
pub struct TestWithString {
    #[since(0.0.1)]
    pub id: u32,
    #[since(0.0.2)]
    pub name: String,
}
#[cfg(feature = "std")]
#[test]
fn test_std_serialize_simple() {
    // serialize a v2.0.0 struct
    let test = TestWithString {
        id: 5,
        name: "Trangar".to_owned(),
    };
    let serialized = binver::to_vec(&test);
    assert_eq!(
        vec![
            0, 0, 0, 5, // id
            0, 0, 0, 7, b'T', b'r', b'a', b'n', b'g', b'a', b'r', // name
        ],
        &serialized[6..] // Ignore version bytes
    );
}

#[cfg(feature = "std")]
#[test]
fn test_std_simple_deserialize() {
    // Deserialize a v2.0.0 struct
    let mut vec = Vec::new();
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver major
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver minor
    vec.extend_from_slice(&(2u16.to_be_bytes())); // semver patch

    vec.extend_from_slice(&(1u32.to_be_bytes())); // ID

    vec.extend_from_slice(&(7u32.to_be_bytes())); // name length
    vec.extend_from_slice(b"Trangar"); // name

    let deserialized: TestWithString = binver::deserialize_slice(&vec).unwrap();

    assert_eq!(deserialized.id, 1);
    assert_eq!(deserialized.name, "Trangar".to_owned());
}

#[test]
#[cfg(feature = "std")]
fn test_std_deserialize_upgrade_version() {
    // Deserialize a v1.0.0 struct into v2.0.0
    let mut vec = Vec::new();
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver major
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver minor
    vec.extend_from_slice(&(1u16.to_be_bytes())); // semver patch

    vec.extend_from_slice(&(1u32.to_be_bytes())); // ID

    let deserialized: TestWithString = binver::deserialize_slice(&vec).unwrap();

    assert_eq!(deserialized.id, 1);
    assert!(deserialized.name.is_empty());
}
