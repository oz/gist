extern crate rustc_serialize;
extern crate hyper;

use self::hyper::{Client, header, error};
use self::hyper::header::{Authorization, Bearer};
use rustc_serialize::json::{ToJson, Json};

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

const GIST_API:          &'static str = "https://api.github.com/gists";
const GITHUB_TOKEN:      &'static str = "GITHUB_TOKEN";
const USER_AGENT:        &'static str = "Pepito Gist";

pub struct Gist {
    filename: String,
    contents: String,
}

pub struct Gists {
    public: bool,
    files:  Vec<Gist>,
}

impl Gists {
    pub fn new(public: bool) -> Gists {
        Gists {
            public: public,
            files:  vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    // Add a file.
    pub fn push(&mut self, gist : Gist) {
        self.files.push(gist);
    }

    // Sent to Github.
    pub fn create(&mut self) -> Result<String, error::Error> {
        let client = Client::new();
        let token : String = env::var(&GITHUB_TOKEN.to_string()).unwrap();
        let json_body = self.to_json().to_string();
        let mut res = try!(client.post(&GIST_API.to_string())
            .header(header::Authorization(Bearer { token: token.to_owned() }))
            .header(header::UserAgent(USER_AGENT.to_owned()))
            .header(header::ContentType::json())
            .body(json_body.as_bytes())
            .send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));
        Ok(body)
    }
}

impl Gist {
    pub fn new(filename: String) -> Gist {
        Gist {
            filename: filename,
            contents: String::new(),
        }
    }

    // Read standard input to contents buffer.
    pub fn read_stdin(&mut self) -> Result<&Gist, io::Error> {
        try!(io::stdin().read_to_string(&mut self.contents));
        Ok(self)
    }

    // Read file to contents buffer.
    pub fn read_file(&mut self) -> Result<&Gist, io::Error> {
        let path = Path::new(&self.filename);
        let mut fh = try!(File::open(&path));
        try!(fh.read_to_string(&mut self.contents));
        Ok(self)
    }
}

impl ToJson for Gist {
    fn to_json(&self) -> Json {
        let mut root = BTreeMap::new();
        root.insert("content".to_string(), self.contents.to_json());
        Json::Object(root)
    }
}

impl ToJson for Gists {
    fn to_json(&self) -> Json {
        let mut root  = BTreeMap::new();
        let mut files = BTreeMap::new();

        root.insert("public".to_string(), self.public.to_json());
        for g in self.files.iter() {
            files.insert(g.filename.clone(), g.to_json());
        }
        root.insert("files".to_string(), files.to_json());
        Json::Object(root)
    }
}

