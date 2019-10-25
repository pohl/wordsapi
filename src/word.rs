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

/*
{"word":"large","results":[{"definition":"in a boastful manner","partOfSpeech":"adverb","synonyms":["big","boastfully","vauntingly"]},{"definition":"in an advanced stage of pregnancy","partOfSpeech":"adjective","synonyms":["big","enceinte","expectant","gravid","great","heavy","with child"],"similarTo":["pregnant"],"derivation":["largeness"]},{"definition":"conspicuous in position or importance","partOfSpeech":"adjective","synonyms":["big","prominent"],"similarTo":["conspicuous"],"examples":["he's very large in financial circles"]},{"definition":"generous and understanding and tolerant","partOfSpeech":"adjective","synonyms":["big","magnanimous"],"similarTo":["generous"],"examples":["a large and generous spirit","a large heart"]},{"definition":"above average in size or number or quantity or magnitude or extent","partOfSpeech":"adjective","synonyms":["big"],"attribute":["size"],"similarTo":["ample","astronomic","astronomical","banging","bear-sized","bigger","biggish","blown-up","bouffant","broad","brobdingnagian","bulky","capacious","colossal","wide-ranging","tremendous","vast","volumed","voluminous","walloping","whacking","whopping","wide","cosmic","deep","double","elephantine","enlarged","enormous","epic","extended","extensive","full-size","galactic","gargantuan","giant","gigantic","grand","great","heroic","huge","hulking","hulky","humongous","immense","jumbo","king-size","king-sized","large-mouthed","large-scale","larger","larger-than-life","largish","life-size","life-sized","lifesize","macro","macroscopic","macroscopical","mammoth","man-sized","massive","medium-large","monolithic","monstrous","monumental","mountainous","outsize","outsized","overlarge","oversize","oversized","plumping","prodigious","puffy","queen-size","queen-sized","rangy","sizable","sizeable","spacious","stupendous","super","thumping","titanic","too large"],"antonyms":["small"],"derivation":["largeness"],"examples":["a large city","a large sum","a big (or large) barn","a large family","a large number of newspapers","large areas of the world"]},{"definition":"ostentatiously lofty in style","partOfSpeech":"adjective","synonyms":["bombastic","declamatory","orotund","tumid","turgid"],"similarTo":["rhetorical"],"derivation":["largeness"],"examples":["a man given to large talk"]},{"definition":"a garment size for a large person","partOfSpeech":"noun","typeOf":["size"]},{"definition":"fairly large or important in effect; influential","partOfSpeech":"adjective","similarTo":["significant","important"],"derivation":["largeness"],"examples":["played a large role in the negotiations"]},{"definition":"having broad power and range and scope","partOfSpeech":"adjective","similarTo":["comprehensive"],"derivation":["largeness"],"examples":["taking the large view","a large effect","a large sympathy"]}],"syllables":{"count":1,"list":["large"]},"pronunciation":{"all":"lɑrdʒ"},"frequency":4.69}
*/
