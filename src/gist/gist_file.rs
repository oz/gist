extern crate rustc_serialize;

use self::rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct GistFile {
    pub name: String,
    pub contents: String,
}


impl GistFile {
    pub fn new(name: String) -> GistFile {
        GistFile {
            name: name,
            contents: String::new(),
        }
    }

    // Read standard input to contents buffer.
    pub fn read_stdin(&mut self) -> Result<usize, io::Error> {
        io::stdin().read_to_string(&mut self.contents)
    }

    // Read file to contents buffer.
    pub fn read_file(&mut self) -> Result<usize, io::Error> {
        let path = Path::new(&self.name);
        let mut fh = File::open(&path).unwrap();
        fh.read_to_string(&mut self.contents)
    }
}

impl ToJson for GistFile {
    fn to_json(&self) -> Json {
        let mut root = BTreeMap::new();
        root.insert("content".to_string(), self.contents.to_json());
        Json::Object(root)
    }
}

