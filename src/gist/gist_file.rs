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
    pub fn read_stdin(&mut self) -> io::Result<()> {
        try!(io::stdin().read_to_string(&mut self.contents));
        Ok(())
    }

    // Read file to contents buffer.
    pub fn read_file(&mut self) -> io::Result<()> {
        let path = Path::new(&self.name);
        let mut fh = try!(File::open(&path));

        try!(fh.read_to_string(&mut self.contents));
        Ok(())
    }
}

impl ToJson for GistFile {
    fn to_json(&self) -> Json {
        let mut root = BTreeMap::new();
        root.insert("content".to_string(), self.contents.to_json());
        Json::Object(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_invalid_file() {
        let mut f = GistFile::new("/not/found.txt".to_string());
        assert!(f.read_file().is_err());
    }

    #[test]
    fn read_valid_file() {
        let mut f = GistFile::new("Cargo.toml".to_string());
        assert!(f.read_file().is_ok());
    }

    #[test]
    fn read_closed_stdin() {
        let mut f = GistFile::new("Cargo.toml".to_string());
        assert!(f.read_stdin().is_err());
    }
}
