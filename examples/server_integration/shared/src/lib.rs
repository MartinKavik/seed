use serde::{Serialize, Deserialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RequestExampleA {
    pub text: String,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ResponseExampleA {
    pub ordinal_number: u32,
    pub text: String,
}