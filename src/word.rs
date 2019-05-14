use crate::HasRequestType;
use crate::RequestType;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Word {
    pub word: String,
    pub frequency: Option<f32>,
    pub pronunciation: Option<HashMap<String, String>>,
    #[serde(rename = "results")]
    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub definition: String,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: Option<String>,
    pub derivation: Option<Vec<String>>,
    #[serde(rename = "hasSubstances")]
    pub has_substances: Option<Vec<String>>,
    #[serde(rename = "typeOf")]
    pub type_of: Option<Vec<String>>,
    #[serde(rename = "verbGroup")]
    pub verb_group: Option<Vec<String>>,
    #[serde(rename = "hasTypes")]
    pub has_types: Option<Vec<String>>,
    #[serde(rename = "hasParts")]
    pub has_parts: Option<Vec<String>>,
    #[serde(rename = "memberOf")]
    pub member_of: Option<Vec<String>>,
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<String>>,
    pub synonyms: Option<Vec<String>>,
    pub antonyms: Option<Vec<String>>,
    pub examples: Option<Vec<String>>,
    #[serde(rename = "similarTo")]
    pub similar_to: Option<Vec<String>>,
    #[serde(rename = "pertainsTo")]
    pub pertains_to: Option<Vec<String>>,
}

impl HasRequestType for Word {
    fn request_type() -> RequestType {
        RequestType::Word
    }
}
