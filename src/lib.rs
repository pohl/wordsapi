extern crate reqwest;
#[macro_use] extern crate hyper;
#[macro_use] extern crate serde_derive;

use hyper::header::Headers;
use reqwest:: { Error, Client };
use std::collections::HashMap;

header! { (XMashapeKey, "X-Mashape-Key") => [String] }
header! { (XMashapeHost, "X-Mashape-Host") => [String] }

#[derive(Serialize, Deserialize, Debug)]
struct WordData {
    word: String,
    frequency: f32,
    pronunciation: HashMap<String, String>,
    results: Vec<WordEntry>
}

#[derive(Serialize, Deserialize, Debug)]
struct WordEntry {
    definition: String,
    #[serde(rename="partOfSpeech")]
    part_of_speech: String,
    derivation: Option<Vec<String>>,
    #[serde(rename="hasSubstances")]
    has_substances: Option<Vec<String>>,
    #[serde(rename="typeOf")]
    type_of: Option<Vec<String>>,
    #[serde(rename="verbGroup")]
    verb_group: Option<Vec<String>>,
    #[serde(rename="hasTypes")]
    has_types: Option<Vec<String>>,
    #[serde(rename="hasParts")]
    has_parts: Option<Vec<String>>,
    #[serde(rename="memberOf")]
    member_of: Option<Vec<String>>,
    synonyms: Option<Vec<String>>,
    antonyms: Option<Vec<String>>,
    examples: Option<Vec<String>>,
    #[serde(rename="similarTo")]
    similar_to: Option<Vec<String>>
}

pub fn look_up_word(word: &str, token: &str)  -> Result<(), Error> {
    let api_base = "https://wordsapiv1.p.mashape.com/words/".to_owned();
    let mashape_host = "wordsapiv1.p.mashape.com".to_owned();
    let uri = format!("{}{}", &api_base, &word);
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set(XMashapeKey(token.to_owned()));
    headers.set(XMashapeHost(mashape_host.to_owned()));

    let resp = client.get(&uri).headers(headers).send();
    match resp {
        Ok(mut v) => { 
            println!("parsing...");
            let data: WordData = v.json()?;
            println!("got something {:?}", data);
        },
        Err(e) => return Err(e),
    }
    Ok(())
}

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}