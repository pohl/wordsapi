use hyper::Error;
use hyper::header::HeaderName;
use hyper::Body;
use hyper::{client::connect::HttpConnector};
use hyper::Request;
use hyper_tls::HttpsConnector;
use log::{debug, error, trace};
use serde::de::DeserializeOwned;
use futures_util::future::TryFutureExt;

use crate::HasRequestType;
use crate::RequestError;
use crate::RequestType;
use crate::Response;

static X_RATE_LIMIT_REMAINING: &[u8] = b"x-ratelimit-requests-remaining";
static X_RATE_LIMIT_REQUESTS_LIMIT: &[u8] = b"x-ratelimit-requests-limit";
static X_MASHAPE_KEY: &[u8] = b"x-mashape-key";
static X_MASHAPE_HOST: &[u8] = b"x-mashape-host";
static API_BASE: &str = "https://wordsapiv1.p.mashape.com/words/";
static MASHAPE_HOST: &str = "wordsapiv1.p.mashape.com";

pub struct Client {
    https_client: hyper::Client<HttpsConnector<HttpConnector>>,
    api_base: String,
    api_token: String,
    mashape_host: String,
}

impl Client {
    pub fn new<T: Into<String>>(token: T) -> Self {
        let https = HttpsConnector::new();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        Self {
            https_client: client,
            api_base: API_BASE.to_owned(),
            api_token: token.into(),
            mashape_host: MASHAPE_HOST.to_owned(),
        }
    }

    pub fn look_up<T: DeserializeOwned + HasRequestType>(
        &self,
        word: &str,
    ) -> Result<Response<T>, RequestError> {
        // TODO return future
        trace!("looking up {}", word);
        let uri = self.request_url(word, &T::request_type());
        let request = Request::builder()
            .method("GET")
            .uri(uri)
            .header(X_MASHAPE_KEY, self.api_token.to_owned())
            .header(X_MASHAPE_HOST, self.mashape_host.to_owned())
            .body(Body::empty())
            .unwrap();
        let work = self
            .https_client
            .request(request)
            .and_then(|response| {
                debug!("the api responded");
                let remaining = response
                    .headers()
                    .get(HeaderName::from_lowercase(X_RATE_LIMIT_REMAINING).unwrap())
                    .map(|hv| hv.to_str().unwrap().to_string())
                    .map_or(0, |v| v.parse::<usize>().unwrap());
                let allowed = response
                    .headers()
                    .get(HeaderName::from_lowercase(X_RATE_LIMIT_REQUESTS_LIMIT).unwrap())
                    .map(|hv| hv.to_str().unwrap().to_string())
                    .map_or(0, |v| v.parse::<usize>().unwrap());
                hyper::body::to_bytes(response).and_then(move |buf| {
                    async move {Ok((
                        String::from_utf8(buf.to_vec()).unwrap(),
                        allowed,
                        remaining,
                    ))}
                }).map_err(Error::from)
            })
            .map_err(|err| {
                error!("api error {}", err);
                Err(RequestError::RequestError)
            });
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result = runtime.block_on(work);
        match result {
            Ok(r) => Ok(Response::new(r.0, r.1, r.2)),
            Err(e) => e,
        }
    }

    fn request_url(&self, word: &str, request_type: &RequestType) -> String {
        let suffix = match *request_type {
            RequestType::Word => "",
            RequestType::Definitions => "/definitions",
            RequestType::Synonyms => "/synonyms",
            RequestType::Antonyms => "/antonyms",
            RequestType::Examples => "/examples",
            RequestType::Rhymes => "/rhymes",
            RequestType::Frequency => "/frequency",
            RequestType::IsATypeOf => "/isATypeOf",
            RequestType::HasTypes => "/hasTypes",
            RequestType::PartOf => "/partOf",
            RequestType::HasParts => "/hasParts",
            RequestType::IsAnInstanceOf => "/isAnInstanceOf",
            RequestType::HasInstances => "/hasInstances",
            RequestType::InRegion => "/inRegion",
            RequestType::RegionOf => "/regionOf",
            RequestType::UsageOf => "/usageOf",
            RequestType::HasUsages => "/hasUsages",
            RequestType::IsAMemberOf => "/isAMemberOf",
            RequestType::HasMembers => "/hasMembers",
            RequestType::IsASubstanceOf => "/isASubstanceOf",
            RequestType::HasSubstances => "/hasSubstances",
            RequestType::HasAttribute => "/hasAttribute",
            RequestType::InCategory => "/inCategory",
            RequestType::HasCategories => "/hasCategories",
            RequestType::Also => "/also",
            RequestType::PertainsTo => "/pertainsTo",
            RequestType::SimilarTo => "/similarTo",
            RequestType::Entails => "/entails",
        };
        format!("{}{}{}", self.api_base, word, suffix)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::API_BASE;
    use crate::client::MASHAPE_HOST;
    use crate::Client;
    use crate::RequestType;

    #[test]
    fn it_has_api_token() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        assert_eq!(word_client.api_token, token);
    }

    #[test]
    fn it_has_api_base() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        assert_eq!(word_client.api_base, API_BASE);
    }

    #[test]
    fn it_has_mashape_host() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        assert_eq!(word_client.mashape_host, MASHAPE_HOST);
    }

    #[test]
    fn it_makes_uri_everything() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::Word);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example"
        );
    }

    #[test]
    fn it_makes_uri_definitions() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::Definitions);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/definitions"
        );
    }

    #[test]
    fn it_makes_uri_synonyms() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::Synonyms);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/synonyms"
        );
    }

    #[test]
    fn it_makes_uri_antonyms() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::Antonyms);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/antonyms"
        );
    }

    #[test]
    fn it_makes_uri_examples() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::Examples);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/examples"
        );
    }

    #[test]
    fn it_makes_uri_rhymes() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::Rhymes);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/rhymes"
        );
    }

    #[test]
    fn it_makes_uri_frequency() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::Frequency);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/frequency"
        );
    }

    #[test]
    fn it_makes_uri_is_a_type_of() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::IsATypeOf);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/isATypeOf"
        );
    }

    #[test]
    fn it_makes_uri_has_types() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::HasTypes);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasTypes"
        );
    }

    #[test]
    fn it_makes_uri_part_of() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::PartOf);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/partOf"
        );
    }

    #[test]
    fn it_makes_uri_has_parts() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::HasParts);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasParts"
        );
    }

    #[test]
    fn it_makes_uri_is_an_instance_of() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::IsAnInstanceOf);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/isAnInstanceOf"
        );
    }

    #[test]
    fn it_makes_uri_has_instances() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::HasInstances);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasInstances"
        );
    }

    #[test]
    fn it_makes_uri_in_region() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::InRegion);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/inRegion"
        );
    }

    #[test]
    fn it_makes_uri_region_of() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::RegionOf);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/regionOf"
        );
    }

    #[test]
    fn it_makes_uri_usage_of() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::UsageOf);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/usageOf"
        );
    }

    #[test]
    fn it_makes_uri_has_usages() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::HasUsages);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasUsages"
        );
    }

    #[test]
    fn it_makes_uri_is_a_member_of() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::IsAMemberOf);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/isAMemberOf"
        );
    }

    #[test]
    fn it_makes_uri_has_members() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::HasMembers);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasMembers"
        );
    }

    #[test]
    fn it_makes_uri_is_a_substance_of() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::IsASubstanceOf);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/isASubstanceOf"
        );
    }

    #[test]
    fn it_makes_uri_has_substances() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::HasSubstances);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasSubstances"
        );
    }

    #[test]
    fn it_makes_uri_has_attribute() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::HasAttribute);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasAttribute"
        );
    }

    #[test]
    fn it_makes_uri_in_category() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::InCategory);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/inCategory"
        );
    }

    #[test]
    fn it_makes_uri_has_categories() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::HasCategories);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/hasCategories"
        );
    }

    #[test]
    fn it_makes_uri_also() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::Also);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/also"
        );
    }

    #[test]
    fn it_makes_uri_pertains_to() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::PertainsTo);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/pertainsTo"
        );
    }

    #[test]
    fn it_makes_uri_similar_to() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::SimilarTo);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/similarTo"
        );
    }

    #[test]
    fn it_makes_uri_entails() {
        let token = "TEST_TOKEN";
        let word_client = Client::new(token);
        let word = "example";
        let request_uri = word_client.request_url(word, &RequestType::Entails);
        assert_eq!(
            request_uri,
            "https://wordsapiv1.p.mashape.com/words/example/entails"
        );
    }
}
