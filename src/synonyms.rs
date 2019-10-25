use crate::HasRequestType;
use crate::RequestType;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Synonyms {
    pub word: String,
    pub synonyms: Option<Vec<String>>,
}

impl HasRequestType for Synonyms {
    fn request_type() -> RequestType {
        RequestType::Synonyms
    }
}

// {"word":"practice","synonyms":["praxis","pattern","apply","use","drill","exercise","practice session","recitation","practise","rehearse","commit","do"]}
