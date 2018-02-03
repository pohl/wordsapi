extern crate reqwest;
#[macro_use] extern crate hyper;

use hyper::header::Headers;
use std::io::Read;
use reqwest:: { Error, Client };

header! { (XMashapeKey, "X-Mashape-Key") => [String] }
header! { (XMashapeHost, "X-Mashape-Host") => [String] }

pub fn look_up_word(word: &str, token: &str)  -> Result<(), reqwest::Error> {
    let api_base = "https://wordsapiv1.p.mashape.com/words/".to_owned();
    let mashape_host = "wordsapiv1.p.mashape.com".to_owned();
    let uri = format!("{}{}", &api_base, &word);
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set(XMashapeKey(token.to_owned()));
    headers.set(XMashapeHost(mashape_host.to_owned()));

    let mut resp = client.get(&uri).headers(headers).send();
    match resp {
        Ok(v) => println!("got something {:?}", v),
        Err(e) => println!("error: : {:?}", e),
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