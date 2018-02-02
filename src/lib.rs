extern crate futures;
#[macro_use] extern crate hyper;
extern crate tokio_core;


use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use hyper::Method;
//use hyper::Request;
use tokio_core::reactor::Core;
//use hyper::header::Header;

header! { (XMashapeKey, "X-Mashape-Key") => [String] }
header! { (XMashapeHost, "X-Mashape-Host") => [String] }

pub fn look_up_word(word: &str, token: &str) -> Result<(), hyper::error::Error> {
    let api_base = "https://wordsapiv1.p.mashape.com/words/".to_owned();
    let mashape_host = "wordsapiv1.p.mashape.com".to_owned();

    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    
    let uri = format!("{}{}", &api_base, &word).parse()?;
    let mut request = hyper::Request::new(Method::Get, uri);
    &request.headers_mut().set(XMashapeKey(token.to_owned()));
    &request.headers_mut().set(XMashapeHost(mashape_host.to_owned()));
    println!("Request...");
    let work = client
	.request(request)
    	.and_then(|res| {
    	println!("Response: {}", res.status());
	res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map_err(From::from)
	})
    });
    core.run(work)?;
    Ok(())
}

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
