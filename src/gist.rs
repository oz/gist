use serde_json;

use crate::config::Config;
use crate::gist_file::GistFile;
use anyhow::Result;
use response::Response;
use std::collections::BTreeMap;
use std::env;
use ureq;

const GIST_API: &'static str = "https://api.github.com";
const GITHUB_TOKEN: &'static str = "GITHUB_TOKEN";
const GITHUB_GIST_TOKEN: &'static str = "GITHUB_GIST_TOKEN";
const GITHUB_GIST_API_ENDPOINT_ENV_NAME: &str = "GITHUB_GIST_API_ENDPOINT";
const USER_AGENT: &'static str = "oz/gist";
const CONTENT_TYPE: &'static str = "application/vnd.github.v3+json";
const ACCEPT_CONTENT: &'static str = "application/json";

// A Gist as received by Github's v3 API.
#[derive(Serialize, Deserialize, Debug)]
pub struct Gist {
    #[serde(skip_serializing, skip_deserializing)]
    token: String,

    #[serde(skip_serializing, skip_deserializing)]
    api: String,

    public: bool,
    files: BTreeMap<String, GistFile>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl Gist {
    pub fn new(public: bool, desc: Option<String>) -> Gist {
        let token: String;
        match Gist::get_token(vec![GITHUB_GIST_TOKEN, GITHUB_TOKEN]) {
            Some(t) => token = t,
            None => {
                if let Some(config) = Config::read() {
                    token = config.gist_token;
                } else {
                    panic!("Missing GITHUB_GIST_TOKEN or GITHUB_TOKEN environment variable.")
                }
            }
        }

        Gist {
            token: token,
            public: public,
            files: BTreeMap::new(),
            description: desc,
            api: env::var(GITHUB_GIST_API_ENDPOINT_ENV_NAME).unwrap_or(GIST_API.to_owned()),
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
    pub fn add_file(&mut self, gist: GistFile) {
        let fullpath = gist.name.clone();
        let v: Vec<&str> = fullpath.split('/').collect();
        let name: String = v.last().unwrap().to_string();
        self.files.insert(name, gist);
    }

    // Send a new Gist to GitHub.
    pub fn create(&mut self) -> Result<String> {
        let url = self.api.clone() + "/gists";
        let resp = ureq::post(&url)
            .header("Authorization", &self.auth_header())
            .header("Accept", ACCEPT_CONTENT)
            .header("User-Agent", USER_AGENT)
            .header("Content-Type", CONTENT_TYPE)
            .send_json(self.to_json()?);
        match resp {
            Err(ureq::Error::StatusCode(code)) => {
                Err(anyhow!("HTTP error while posting gist: {}", code))
            }
            Err(e) => Err(anyhow!("Error posting gist: {}", e.to_string())),
            Ok(mut response) => Ok(response.body_mut().read_to_string().unwrap()),
        }
    }

    pub fn to_json(&self) -> Result<serde_json::Value> {
        Ok(serde_json::to_value(self)?)
    }

    // List the gists of a given user, or public gists when login is None.
    pub fn list(&mut self, login: Option<String>) -> Result<Vec<Response>> {
        let url = self.gist_list_url(login);
        let resp = ureq::get(&url)
            .header("Authorization", &self.auth_header())
            .header("User-Agent", USER_AGENT)
            .header("Accept", ACCEPT_CONTENT)
            .header("Content-Type", CONTENT_TYPE)
            .query("per_page", "10")
            .call();
        match resp {
            Err(ureq::Error::StatusCode(code)) => {
                Err(anyhow!("HTTP error while listing gists: {}", code))
            }
            Err(e) => Err(anyhow!("Error listing gists: {}", e.to_string())),
            Ok(mut response) => {
                let gist_list = response.body_mut().read_json::<Vec<Response>>()?;
                Ok(gist_list)
            }
        }
    }

    fn auth_header(&mut self) -> String {
        format!("bearer {}", self.token)
    }

    fn gist_list_url(&mut self, login: Option<String>) -> String {
        match login {
            Some(user) => self.api.clone() + "/users/" + &user + "/gists",
            _ => self.api.clone() + "/gists/public",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gist_file::GistFile;

    fn fake_gist_file(name: &str, content: Option<&str>) -> GistFile {
        let mut f = GistFile::new(name.to_string());
        if content.is_some() {
            f.content = content.unwrap().to_string();
        }
        return f;
    }

    #[test]
    fn add_files() {
        let mut g = Gist::new(true, None);
        g.add_file(fake_gist_file("/path/to/file.txt", None));
        g.add_file(fake_gist_file("/path/to/other_file.txt", None));
        assert_eq!(g.files.len(), 2);
    }

    #[test]
    fn emptyness() {
        let mut g = Gist::new(true, None);
        assert!(g.is_empty());

        g.add_file(fake_gist_file("file.txt", None));
        assert!(!g.is_empty());
    }

    #[test]
    fn public_json() {
        let mut public = Gist::new(true, None);
        public.add_file(fake_gist_file("file.txt", Some("public file contents")));

        let public_json = public.to_json().unwrap().to_string();
        assert_eq!(
            public_json,
            r#"{"files":{"file.txt":{"content":"public file contents"}},"public":true}"#
        );
    }

    #[test]
    fn private_json() {
        let mut private = Gist::new(false, None);
        private.add_file(fake_gist_file("secret.txt", Some("private file contents")));

        let private_json = private.to_json().unwrap().to_string();
        assert_eq!(
            private_json,
            r#"{"files":{"secret.txt":{"content":"private file contents"}},"public":false}"#
        );
    }

    #[test]
    fn gist_with_description() {
        let desc = Some("description".to_string());
        let mut private = Gist::new(false, desc);
        private.add_file(fake_gist_file("secret.txt", Some("private file contents")));

        let private_json = private.to_json().unwrap().to_string();
        assert_eq!(
            private_json,
            r#"{"description":"description","files":{"secret.txt":{"content":"private file contents"}},"public":false}"#
        );
    }
}
