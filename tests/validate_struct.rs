use binver::Serializable;

#[derive(Serializable)]
pub struct Test {
    #[since(1.0.0)]
    pub id: u32,
    #[since(2.0.0)]
    pub name: String,
}

#[test]
fn test_serialize_simple() {
    // serialize a v2.0.0 struct
    let test = Test {
        id: 5,
        name: String::from("Trangar"),
    };
    let serialized = binver::to_vec(&test);
    assert_eq!(
        vec![
            0, 2, // semver major
            0, 0, // semver minor
            0, 0, // semver patch
            0, 0, 0, 5, // id
            0, 0, 0, 7, b'T', b'r', b'a', b'n', b'g', b'a', b'r', // name
        ],
        serialized
    );
}

#[test]
fn test_simple_deserialize() {
    // Deserialize a v2.0.0 struct
    let mut vec = Vec::<u8>::new();
    vec.extend_from_slice(&(2u16.to_be_bytes())); // semver major
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver minor
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver patch

    vec.extend_from_slice(&(1u32.to_be_bytes())); // ID

    vec.extend_from_slice(&(7u32.to_be_bytes())); // name length
    vec.extend_from_slice(b"Trangar"); // name

    let deserialized: Test = binver::deserialize_slice(&vec).unwrap();

    assert_eq!(deserialized.id, 1);
    assert_eq!(deserialized.name, String::from("Trangar"));
}

#[test]
fn test_deserialize_upgrade_version() {
    // Deserialize a v1.0.0 struct into v2.0.0
    let mut vec = Vec::<u8>::new();
    vec.extend_from_slice(&(1u16.to_be_bytes())); // semver major
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver minor
    vec.extend_from_slice(&(0u16.to_be_bytes())); // semver patch

    vec.extend_from_slice(&(1u32.to_be_bytes())); // ID

    let deserialized: Test = binver::deserialize_slice(&vec).unwrap();

    assert_eq!(deserialized.id, 1);
    assert_eq!(deserialized.name, String::new());
}
