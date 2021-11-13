use serde_json;

use crate::gist_file::GistFile;
use anyhow::Result;
use std::collections::BTreeMap;
use std::env;
use ureq;

const GIST_API: &'static str = "https://api.github.com/gists";
const GITHUB_TOKEN: &'static str = "GITHUB_TOKEN";
const GITHUB_GIST_TOKEN: &'static str = "GITHUB_GIST_TOKEN";
const GITHUB_GIST_API_ENDPOINT_ENV_NAME: &str = "GITHUB_GIST_API_ENDPOINT";
const USER_AGENT: &'static str = "Pepito Gist";
const CONTENT_TYPE: &'static str = "application/json";

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
            None => panic!("Missing GITHUB_GIST_TOKEN or GITHUB_TOKEN environment variable."),
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

    fn auth_header(&mut self) -> String {
        format!("bearer {}", self.token)
    }

    // Send to Github.
    pub fn create(&mut self) -> Result<String> {
        let res = ureq::post(&self.api)
            .set("Authorization", &self.auth_header())
            .set("User-Agent", USER_AGENT)
            .set("Content-Type", CONTENT_TYPE)
            .send_string(&self.to_json());
        if res.ok() {
            Ok(res.into_string()?)
        } else {
            let body = res.into_string().unwrap();
            Err(anyhow!("{}", body))
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
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

        let public_json = public.to_json().to_string();
        assert_eq!(
            public_json,
            "{\"public\":true,\"files\":{\"file.txt\":{\"content\":\"public file \
             contents\"}}}"
        );
    }

    #[test]
    fn private_json() {
        let mut private = Gist::new(false, None);
        private.add_file(fake_gist_file("secret.txt", Some("private file contents")));

        let private_json = private.to_json().to_string();
        assert_eq!(
            private_json,
            "{\"public\":false,\"files\":{\"secret.txt\":{\"content\":\"private file \
             contents\"}}}"
        );
    }

    #[test]
    fn gist_with_description() {
        let desc = Some("description".to_string());
        let mut private = Gist::new(false, desc);
        private.add_file(fake_gist_file("secret.txt", Some("private file contents")));

        let private_json = private.to_json().to_string();
        assert_eq!(
            private_json,
            "{\"public\":false,\"files\":{\"secret.txt\":{\"content\":\
             \"private file contents\"}},\"description\":\"description\"}"
        );
    }
}
