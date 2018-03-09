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

    pub fn look_up(&self, word: &str, request_type: &WordRequestType) -> Result<WordResponse, WordAPIError> {
        let uri = self.request_url(word, request_type);
        let mut headers = Headers::new();
        headers.set(XMashapeKey(self.api_token.clone()));
        headers.set(XMashapeHost(self.mashape_host.clone()));

        let resp = self.http_client.get(&uri).headers(headers).send();
        match resp {
            Ok(v) => Ok(WordResponse::new(v)),
            Err(_e) => Err(WordAPIError::RequestError),
        }
    }

    fn request_url(&self, word: &str, request_type: &WordRequestType) -> String {
        let suffix = match *request_type {
            WordRequestType::Everything => "",
            WordRequestType::Definitions => "/definitions",
            WordRequestType::Synonyms => "/synonyms",
            WordRequestType::Antonyms => "/antonyms",
            WordRequestType::Examples => "/examples",
            WordRequestType::Rhymes => "/rhymes",
            WordRequestType::Frequency => "/frequency",
            WordRequestType::IsATypeOf => "/isATypeOf",
            WordRequestType::HasTypes => "/hasTypes",
            WordRequestType::PartOf => "/partOf",
            WordRequestType::HasParts => "/hasParts",
            WordRequestType::IsAnInstanceOf => "/isAnInstanceOf",
            WordRequestType::HasInstances => "/hasInstances",
            WordRequestType::InRegion => "/inRegion",
            WordRequestType::RegionOf => "/regionOf",
            WordRequestType::UsageOf => "/usageOf",
            WordRequestType::HasUsages => "/hasUsages",
            WordRequestType::IsAMemberOf => "/isAMemberOf",
            WordRequestType::HasMembers => "/hasMembers",
            WordRequestType::IsASubstanceOf => "/isASubstanceOf",
            WordRequestType::HasSubstances => "/hasSubstances",
            WordRequestType::HasAttribute => "/hasAttribute",
            WordRequestType::InCategory => "/inCategory",
            WordRequestType::HasCategories => "/hasCategories",
            WordRequestType::Also => "/also",
            WordRequestType::PertainsTo => "/pertainsTo",
            WordRequestType::SimilarTo => "/similarTo",
            WordRequestType::Entails => "/entails",
        };
        format!("{}{}{}", self.api_base, word, suffix)
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
    fn it_makes_uri_everything() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::Everything);
        assert_eq!(request_uri, "https://wordsapiv1.p.mashape.com/words/example");
    }

    #[test]
    fn it_makes_uri_definitions() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::Definitions);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/definitions"
        );
    }

    #[test]
    fn it_makes_uri_synonyms() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::Synonyms);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/synonyms"
        );
    }

    #[test]
    fn it_makes_uri_antonyms() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::Antonyms);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/antonyms"
        );
    }

    #[test]
    fn it_makes_uri_examples() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::Examples);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/examples"
        );
    }

    #[test]
    fn it_makes_uri_rhymes() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::Rhymes);
        assert_eq!(request_uri, "https://wordsapiv1.p.mashape.com/words/example/rhymes");
    }

    #[test]
    fn it_makes_uri_frequency() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::Frequency);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/frequency"
        );
    }

    #[test]
    fn it_makes_uri_is_a_type_of() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::IsATypeOf);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/isATypeOf"
        );
    }

    #[test]
    fn it_makes_uri_has_types() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::HasTypes);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasTypes"
        );
    }

    #[test]
    fn it_makes_uri_part_of() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::PartOf);
        assert_eq!(request_uri, "https://wordsapiv1.p.mashape.com/words/example/partOf");
    }

    #[test]
    fn it_makes_uri_has_parts() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::HasParts);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasParts"
        );
    }

    #[test]
    fn it_makes_uri_is_an_instance_of() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::IsAnInstanceOf);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/isAnInstanceOf"
        );
    }

    #[test]
    fn it_makes_uri_has_instances() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::HasInstances);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasInstances"
        );
    }

    #[test]
    fn it_makes_uri_in_region() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::InRegion);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/inRegion"
        );
    }

    #[test]
    fn it_makes_uri_region_of() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::RegionOf);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/regionOf"
        );
    }

    #[test]
    fn it_makes_uri_usage_of() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::UsageOf);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/usageOf"
        );
    }

    #[test]
    fn it_makes_uri_has_usages() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::HasUsages);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasUsages"
        );
    }

    #[test]
    fn it_makes_uri_is_a_member_of() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::IsAMemberOf);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/isAMemberOf"
        );
    }

    #[test]
    fn it_makes_uri_has_members() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::HasMembers);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasMembers"
        );
    }

    #[test]
    fn it_makes_uri_is_a_substance_of() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::IsASubstanceOf);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/isASubstanceOf"
        );
    }

    #[test]
    fn it_makes_uri_has_substances() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::HasSubstances);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasSubstances"
        );
    }

    #[test]
    fn it_makes_uri_has_attribute() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::HasAttribute);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasAttribute"
        );
    }

    #[test]
    fn it_makes_uri_in_category() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::InCategory);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/inCategory"
        );
    }

    #[test]
    fn it_makes_uri_has_categories() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::HasCategories);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasCategories"
        );
    }

    #[test]
    fn it_makes_uri_also() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::Also);
        assert_eq!(request_uri, "https://wordsapiv1.p.mashape.com/words/example/also");
    }

    #[test]
    fn it_makes_uri_pertains_to() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::PertainsTo);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/pertainsTo"
        );
    }

    #[test]
    fn it_makes_uri_similar_to() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::SimilarTo);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/similarTo"
        );
    }

    #[test]
    fn it_makes_uri_entails() {
        let token = "TEST_TOKEN";
        let word_client = WordClient::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word,&WordRequestType::Entails);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/entails"
        );
    }

}
