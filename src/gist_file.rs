extern crate serde_json;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct GistFile {
    #[serde(skip_serializing, default = "GistFile::default_name")]
    pub name: String,

    #[serde(rename(serialize = "content"))]
    pub content: String,
}

impl GistFile {
    pub fn new(name: String) -> GistFile {
        GistFile {
            name: name,
            content: String::new(),
        }
    }

    pub fn default_name() -> String {
        "content".to_string()
    }

    // Read standard input to content buffer.
    pub fn read_stdin(&mut self) -> io::Result<()> {
        try!(io::stdin().read_to_string(&mut self.content));
        Ok(())
    }

    // Read file to content buffer.
    pub fn read_file(&mut self) -> io::Result<()> {
        let path = Path::new(&self.name);
        let mut fh = try!(File::open(&path));

        try!(fh.read_to_string(&mut self.content));
        Ok(())
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
}
