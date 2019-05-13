#![forbid(unsafe_code)]
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate serde_derive;
extern crate log;
extern crate serde_json;

pub mod word;
pub use self::word::Entry;
pub use self::word::Word;

pub mod error;
pub use self::error::RequestError;

pub mod client;
pub use self::client::Client;

pub mod request;
pub use self::request::HasRequestType;
pub use self::request::RequestType;

pub mod response;
pub use self::response::try_parse;
pub use self::response::Response;
