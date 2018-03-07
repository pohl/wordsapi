#[macro_use]
extern crate hyper;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use hyper::header::Headers;
use std::collections::HashMap;
use std::fmt;
use std::error::Error as StdError;

header! { (XMashapeKey, "X-Mashape-Key") => [String] }
header! { (XMashapeHost, "X-Mashape-Host") => [String] }
header! { (XRateLimitRemaining, "X-RateLimit-requests-Remaining") => [usize]}
header! { (XRateLimitRequestsLimit, "X-RateLimit-requests-Limit") => [usize]}

static API_BASE: &'static str = "https://wordsapiv1.p.mashape.com/words/";
static MASHAPE_HOST: &'static str = "wordsapiv1.p.mashape.com";

#[derive(Debug)]
pub enum WordAPIError {
    RequestError,
    ResultParseError,
}

#[derive(Debug)]
pub enum WordRequestType {
    Everything,
    Definitions,
    Synonyms,
    Antonyms,
    Examples,
    Rhymes,
    Frequency,
    IsATypeOf,
    HasTypes,
    PartOf,
    HasParts,
    IsAnInstanceOf,
    HasInstances,
    InRegion,
    RegionOf,
    UsageOf,
    HasUsages,
    IsAMemberOf,
    HasMembers,
    IsASubstanceOf,
    HasSubstances,
    HasAttribute,
    InCategory,
    HasCategories,
    Also,
    PertainsTo,
    SimilarTo,
    Entails,
}

impl fmt::Display for WordAPIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WordAPIError::RequestError => f.write_str("RequestError"),
            WordAPIError::ResultParseError => f.write_str("ResultParseError"),
        }
    }
}
impl StdError for WordAPIError {
    fn description(&self) -> &str {
        match *self {
            WordAPIError::RequestError => "WordAPI request failed",
            WordAPIError::ResultParseError => "Could not parse result",
        }
    }
}

pub struct WordClient {
    http_client: reqwest::Client,
    api_base: String,
    api_token: String,
    mashape_host: String,
}

pub struct WordResponse {
    pub response_json: String,
    pub rate_limit_remaining: usize,
    pub rate_limit_requests_limit: usize,
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
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<String>>,
    pub synonyms: Option<Vec<String>>,
    pub antonyms: Option<Vec<String>>,
    pub examples: Option<Vec<String>>,
    #[serde(rename = "similarTo")]
    pub similar_to: Option<Vec<String>>,
}

impl WordClient {
    pub fn new(token: &str) -> WordClient {
        WordClient {
            http_client: reqwest::Client::new(),
            api_base: API_BASE.to_owned(),
            api_token: token.to_owned(),
            mashape_host: MASHAPE_HOST.to_owned(),
        }
    }

    pub fn look_up(&self, word: &str) -> Result<WordResponse, WordAPIError> {
        let uri = format!("{}{}", self.api_base, &word);
        let mut headers = Headers::new();
        headers.set(XMashapeKey(self.api_token.clone()));
        headers.set(XMashapeHost(self.mashape_host.clone()));

        let resp = self.http_client.get(&uri).headers(headers).send();
        match resp {
            Ok(v) => Ok(WordResponse::new(v)),
            Err(_e) => Err(WordAPIError::RequestError),
        }
    }
}

impl WordResponse {
    fn new(mut request_response: reqwest::Response) -> WordResponse {
        let raw_json = match request_response.text() {
            Err(_e) => "".to_owned(),
            Ok(s) => s,
        };
        let remaining = request_response
            .headers()
            .get::<XRateLimitRemaining>()
            .map(|r| **r)
            .unwrap_or(0);
        let allowed = request_response
            .headers()
            .get::<XRateLimitRequestsLimit>()
            .map(|r| **r)
            .unwrap_or(0);
        WordResponse {
            response_json: raw_json,
            rate_limit_remaining: remaining,
            rate_limit_requests_limit: allowed,
        }
    }

    pub fn try_parse(&self) -> Result<WordData, WordAPIError> {
        try_parse(&self.response_json)
    }
}

pub fn try_parse(word_json: &str) -> Result<WordData, WordAPIError> {
    let result = serde_json::from_str(word_json);
    match result {
        Ok(word_data) => Ok(word_data),
        Err(_e) => Err(WordAPIError::ResultParseError),
    }
}

impl WordRequestType {
    fn to_str(&self) -> String {
        match self {
            &WordRequestType::Everything => "".to_owned(),
            _ => {
                let repr = format!("{:?}", &self);
                let mut result = String::with_capacity(repr.len());
                if !repr.is_empty() {
                    let mut chars = repr.chars();
                    result.push_str(&chars.next().unwrap().to_lowercase().to_string());
                    result.extend(chars);
                }
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use WordClient;
    use API_BASE;
    use MASHAPE_HOST;
    use WordRequestType;

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

    #[test]
    fn it_converts_enum_to_string() {
        assert_eq!(WordRequestType::Everything.to_str(), "");
        assert_eq!(WordRequestType::Definitions.to_str(), "definitions");
        assert_eq!(WordRequestType::Synonyms.to_str(), "synonyms");
        assert_eq!(WordRequestType::Antonyms.to_str(), "antonyms");
        assert_eq!(WordRequestType::Examples.to_str(), "examples");
        assert_eq!(WordRequestType::Rhymes.to_str(), "rhymes");
        assert_eq!(WordRequestType::Frequency.to_str(), "frequency");
        assert_eq!(WordRequestType::IsATypeOf.to_str(), "isATypeOf");
        assert_eq!(WordRequestType::HasTypes.to_str(), "hasTypes");
        assert_eq!(WordRequestType::PartOf.to_str(), "partOf");
        assert_eq!(WordRequestType::HasParts.to_str(), "hasParts");
        assert_eq!(WordRequestType::IsAnInstanceOf.to_str(), "isAnInstanceOf");
        assert_eq!(WordRequestType::HasInstances.to_str(), "hasInstances");
        assert_eq!(WordRequestType::InRegion.to_str(), "inRegion");
        assert_eq!(WordRequestType::RegionOf.to_str(), "regionOf");
        assert_eq!(WordRequestType::UsageOf.to_str(), "usageOf");
        assert_eq!(WordRequestType::HasUsages.to_str(), "hasUsages");
        assert_eq!(WordRequestType::IsAMemberOf.to_str(), "isAMemberOf");
        assert_eq!(WordRequestType::HasMembers.to_str(), "hasMembers");
        assert_eq!(WordRequestType::IsASubstanceOf.to_str(), "isASubstanceOf");
        assert_eq!(WordRequestType::HasSubstances.to_str(), "hasSubstances");
        assert_eq!(WordRequestType::HasAttribute.to_str(), "hasAttribute");
        assert_eq!(WordRequestType::InCategory.to_str(), "inCategory");
        assert_eq!(WordRequestType::HasCategories.to_str(), "hasCategories");
        assert_eq!(WordRequestType::Also.to_str(), "also");
        assert_eq!(WordRequestType::PertainsTo.to_str(), "pertainsTo");
        assert_eq!(WordRequestType::SimilarTo.to_str(), "similarTo");
        assert_eq!(WordRequestType::Entails.to_str(), "entails");
    }
}
