#[macro_use]
extern crate hyper;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;

use hyper::header::Headers;
use reqwest::{Client, Error};
use std::collections::HashMap;

header! { (XMashapeKey, "X-Mashape-Key") => [String] }
header! { (XMashapeHost, "X-Mashape-Host") => [String] }

#[derive(Serialize, Deserialize, Debug)]
pub struct WordData {
    pub word: String,
    pub frequency: f32,
    pub pronunciation: HashMap<String, String>,
    pub results: Vec<WordEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WordEntry {
    pub definition: String,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
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
    pub synonyms: Option<Vec<String>>,
    pub antonyms: Option<Vec<String>>,
    pub examples: Option<Vec<String>>,
    #[serde(rename = "similarTo")]
    pub similar_to: Option<Vec<String>>,
}

pub fn look_up_word(word: &str, token: &str) -> Result<WordData, Error> {
    let api_base = "https://wordsapiv1.p.mashape.com/words/".to_owned();
    let mashape_host = "wordsapiv1.p.mashape.com".to_owned();
    let uri = format!("{}{}", &api_base, &word);
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set(XMashapeKey(token.to_owned()));
    headers.set(XMashapeHost(mashape_host.to_owned()));

    let resp = client.get(&uri).headers(headers).send();
    return match resp {
        Ok(mut v) => {
            let data: WordData = v.json()?;
            Ok(data)
        }
        Err(e) => Err(e),
    };
}

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
