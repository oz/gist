extern crate serde_json;

#[derive(Deserialize)]
pub struct Response {
    pub html_url: String,
    pub description: Option<String>,
    pub created_at: String,
}

pub fn decode(res: &str) -> Result<Response, serde_json::Error> {
    serde_json::from_str(&res)
}
