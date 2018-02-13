#[macro_use]
extern crate hyper;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use hyper::header::Headers;
use reqwest::{Client, Error, Response};
use std::collections::HashMap;

header! { (XMashapeKey, "X-Mashape-Key") => [String] }
header! { (XMashapeHost, "X-Mashape-Host") => [String] }

static API_BASE: &'static str = "https://wordsapiv1.p.mashape.com/words/";
static MASHAPE_HOST: &'static str = "wordsapiv1.p.mashape.com";

pub struct WordClient {
    http_client: Client,
    api_base: String,
    api_token: String,
    mashape_host: String,
}

pub struct WordResponse {
    request_response: Response,
    response_json: String
}

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

impl WordClient {
    pub fn new(token: &str) -> WordClient {
        WordClient {
            http_client: Client::new(),
            api_base: API_BASE.to_owned(),
            api_token: token.to_owned(),
            mashape_host: MASHAPE_HOST.to_owned(),
        }
    }

    pub fn look_up(&self, word: &str) -> Result<WordResponse, Error> {
        let uri = format!("{}{}", self.api_base, &word);
        let mut headers = Headers::new();
        headers.set(XMashapeKey(self.api_token.clone()));
        headers.set(XMashapeHost(self.mashape_host.clone()));

        let resp = self.http_client.get(&uri).headers(headers).send();
        match resp {
            Ok(v) => Ok(WordResponse::new(v)),
            Err(e) => Err(e),
        }
    }
}

impl WordResponse {
    pub fn new(mut request_response: Response) -> WordResponse {
        let raw_json = match request_response.text() {
            Err(_e) => "".to_owned(),
            Ok(s) => s,
        };
        WordResponse {
            request_response: request_response,
            response_json: raw_json,            
        }
    }

    pub fn try_parse(&self) -> Result<WordData, serde_json::Error> {
        let data: WordData = serde_json::from_str(&self.response_json)?;
        Ok(data)
    }

    pub fn raw_json(&self) -> &String {
        &self.response_json
    }
}

#[cfg(test)]
mod tests {
    use WordClient;
    use API_BASE;
    use MASHAPE_HOST;

    #[test]
    fn it_has_api_token() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        assert_eq!(word_client.api_token, token);
    }

    #[test]
    fn it_has_api_base() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        assert_eq!(word_client.api_base, API_BASE);
    }

    #[test]
    fn it_has_mashape_host() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        assert_eq!(word_client.mashape_host, MASHAPE_HOST);
    }
}
