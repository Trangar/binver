#![cfg(feature = "std")]

use binver::Serializable;

#[derive(Serializable)]
pub struct Root {
    #[since(1.0.0)]
    pub branches: Vec<Branch>,
}

#[derive(Serializable)]
pub struct Branch {
    #[since(1.0.0)]
    pub id: u32,
    #[since(1.0.0)]
    pub name: String,

    #[since(2.0.0)]
    pub branches: Vec<Branch>,
}

#[test]
fn validate_version() {}
