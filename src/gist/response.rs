extern crate rustc_serialize;

use self::rustc_serialize::json;
use self::rustc_serialize::json::DecoderError;

#[derive(RustcDecodable)]
pub struct Response {
    pub html_url: String,
}

pub fn decode(res: &str) -> Result<Response, DecoderError> {
    json::decode(res)
}
