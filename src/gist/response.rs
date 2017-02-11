extern crate serde_json;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub html_url: String,
}

pub fn decode(res: &str) -> Result<Response, serde_json::Error> {
    serde_json::from_str(&res)
}
