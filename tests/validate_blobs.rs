use binver::{ReadConfig, Serializable};

#[derive(Serializable, Debug, PartialEq)]
pub struct Test<'a> {
    #[since(0.0.1)]
    pub slice: &'a [u8],
}

#[test]
fn test_slice_serialize_deserialize() {
    let config = ReadConfig {
        error_on_trailing_bytes: true,
    };

    let test = Test {
        slice: b"Hello there",
    };

    let mut slice = [0u8; 1024];
    let length = binver::write_to_slice(&mut slice, &test).unwrap();

    assert_eq!(slice[6..10], (test.slice.len() as u32).to_be_bytes());
    assert_eq!(&slice[..length][10..], test.slice);

    let result: Test = binver::deserialize_slice_with_config(&slice[..length], config).unwrap();
    assert_eq!(result.slice, b"Hello there");
}

#[cfg(feature = "std")]
#[derive(Serializable, Debug, PartialEq)]
pub struct TestVec {
    #[since(0.0.1)]
    pub blob: Vec<u8>,
    #[since(0.0.1)]
    pub string_list: Vec<String>,
}

#[test]
fn test_vec_serialize_deserialize() {
    let config = ReadConfig {
        error_on_trailing_bytes: true,
    };
    let test = TestVec {
        blob: vec![1, 2, 3, 4, 5, 6],
        string_list: vec![String::from("Hello"), String::from("there")],
    };

    let serialized = binver::to_vec(&test);

    let deserialized: TestVec = binver::deserialize_slice_with_config(&serialized, config).unwrap();

    assert_eq!(test, deserialized);
}
