use crate::HasRequestType;
use crate::RequestType;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Antonyms {
    pub word: String,
    pub antonyms: Option<Vec<String>>,
}

impl HasRequestType for Antonyms {
    fn request_type() -> RequestType {
        RequestType::Antonyms
    }
}

// {"word":"silence","antonyms":["sound"]}

