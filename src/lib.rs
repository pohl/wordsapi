extern crate futures;
#[macro_use] extern crate hyper;
extern crate tokio_core;


use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use hyper::Method;
use tokio_core::reactor::Core;


header! { (XMashapeKey, "X-Mashape-Key") => [String] }
header! { (XMashapeHost, "X-Mashape-Host") => [String] }

pub fn look_up_word(word: &str, token: &str) -> Result<(), hyper::error::Error> {
    let api_base = "https://wordsapiv1.p.mashape.com/words/".to_owned();
    let mashape_host = "wordsapiv1.p.mashape.com".to_owned();

    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    
    let uri = format!("{}{}", &api_base, &word).parse()?;
    println!("URI: {:?}", &uri);
    let mut request = hyper::Request::new(Method::Get, uri);
    &request.headers_mut().set(XMashapeKey(token.to_owned()));
    &request.headers_mut().set(XMashapeHost(mashape_host.to_owned()));
    println!("Request: {:?}", &request);
    let response = core.run(client.request(request)).unwrap();
    println!("{} {}", response.version(), response.status());
    for header in response.headers().iter() {
        print!("{}", header);
    }

    // Finish off our request by fetching all of the body.
    let body = core.run(response.body().concat2()).unwrap();
    println!("{}", String::from_utf8_lossy(&body));

    /*
    let work = client
	.request(request)
    	.and_then(|res| {
            println!("Response: {}", res.status());
	    res.body().for_each(|chunk| {
                println!("Chunk...");
                io::stdout()
                    .write_all(&chunk)
                    .map_err(From::from)

	})
    });
    let result = core.run(work)?;
    println!("Resultd: {:?}", result);
    */
    Ok(())
}

#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
