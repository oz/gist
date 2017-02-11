pub mod gist_file;
pub mod response;

extern crate serde_json;
extern crate hyper;

use self::hyper::Client as HyperClient;
use self::hyper::header::{Authorization, Bearer, UserAgent, ContentType};
use self::hyper::status::StatusCode;

use std::collections::BTreeMap;
use std::env;
use std::io::Read;

const GIST_API: &'static str = "https://api.github.com/gists";
const GITHUB_TOKEN: &'static str = "GITHUB_TOKEN";
const GITHUB_GIST_TOKEN: &'static str = "GITHUB_GIST_TOKEN";
const USER_AGENT: &'static str = "Pepito Gist";

#[derive(Serialize, Deserialize, Debug)]
pub struct Gist {
    #[serde(skip_serializing,skip_deserializing)]
    anonymous: bool,
    #[serde(skip_serializing,skip_deserializing)]
    token: String,

    public: bool,
    files: BTreeMap<String, gist_file::GistFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl Gist {
    pub fn new(public: bool, anonymous: bool, desc: Option<String>) -> Gist {
        let mut token = "".to_string();
        if !anonymous {
            match Gist::get_token(vec![GITHUB_GIST_TOKEN, GITHUB_TOKEN]) {
                Some(t) => token = t,
                None => panic!("Missing GITHUB_GIST_TOKEN or GITHUB_TOKEN environment variable."),
            }
        }

        Gist {
            token: token,
            anonymous: anonymous,
            public: public,
            files: BTreeMap::new(),
            description: desc,
        }
    }

    fn get_token(tokens: Vec<&str>) -> Option<String> {
        for token in tokens.iter() {
            match env::var(token) {
                Ok(t) => return Some(t),
                Err(_) => {}
            }
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    // Add a file.
    pub fn add_file(&mut self, gist: gist_file::GistFile) {
        let fullpath = gist.name.clone();
        let v: Vec<&str> = fullpath.split('/').collect();
        let name: String = v.last().unwrap().to_string();
        self.files.insert(name, gist);
        // self.files.push(gist);
    }

    // Sent to Github.
    pub fn create(&mut self) -> Result<String, String> {
        let client = HyperClient::new();
        let json_body = self.to_json().to_string();
        let uri = &GIST_API.to_string();

        let mut req = client.post(uri);
        if !self.anonymous {
            req = req.header(Authorization(Bearer { token: self.token.to_owned() }))
        }

        let mut res = req.header(UserAgent(USER_AGENT.to_owned()))
            .header(ContentType::json())
            .body(json_body.as_bytes())
            .send()
            .unwrap();
        if res.status == StatusCode::Created {
            let mut body = String::new();
            res.read_to_string(&mut body).unwrap();
            return Ok(body);
        }
        Err("API error".to_owned())
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_gist_file(name: &str, content: Option<&str>) -> gist_file::GistFile {
        let mut f = gist_file::GistFile::new(name.to_string());
        if content.is_some() {
            f.content = content.unwrap().to_string();
        }
        return f;
    }

    #[test]
    fn add_files() {
        let mut g = Gist::new(true, true, None);
        g.add_file(fake_gist_file("/path/to/file.txt", None));
        g.add_file(fake_gist_file("/path/to/other_file.txt", None));
        assert_eq!(g.files.len(), 2);
    }

    #[test]
    fn emptyness() {
        let mut g = Gist::new(true, true, None);
        assert!(g.is_empty());

        g.add_file(fake_gist_file("file.txt", None));
        assert!(!g.is_empty());
    }

    #[test]
    fn public_json() {
        let mut public = Gist::new(true, true, None);
        public.add_file(fake_gist_file("file.txt", Some("public file contents")));

        let public_json = public.to_json().to_string();
        assert_eq!(public_json,
                   "{\"public\":true,\"files\":{\"file.txt\":{\"content\":\"public file \
                    contents\"}}}");
    }

    #[test]
    fn private_json() {
        let mut private = Gist::new(false, true, None);
        private.add_file(fake_gist_file("secret.txt", Some("private file contents")));

        let private_json = private.to_json().to_string();
        assert_eq!(private_json,
                   "{\"public\":false,\"files\":{\"secret.txt\":{\"content\":\"private file \
                    contents\"}}}");
    }

    #[test]
    fn gist_with_description() {
        let desc = Some("description".to_string());
        let mut private = Gist::new(false, true, desc);
        private.add_file(fake_gist_file("secret.txt", Some("private file contents")));

        let private_json = private.to_json().to_string();
        assert_eq!(private_json,
                   "{\"public\":false,\"files\":{\"secret.txt\":{\"content\":\
                    \"private file contents\"}},\"description\":\"description\"}");
    }
}
