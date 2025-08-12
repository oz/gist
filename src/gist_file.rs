extern crate serde_json;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GistFile {
    #[serde(skip_serializing, default = "GistFile::default_name")]
    pub name: String,
    #[serde(rename(serialize = "content"))]
    pub content: String,
}

impl GistFile {
    pub fn new(name: String) -> GistFile {
        GistFile {
            name,
            content: String::new(),
        }
    }

    pub fn default_name() -> String {
        String::from("content")
    }

    // Read standard input to content buffer.
    pub fn read_stdin(&mut self) -> io::Result<usize> {
        io::stdin().read_to_string(&mut self.content)
    }

    // Read file to content buffer.
    pub fn read_file(&mut self) -> io::Result<usize> {
        let path = Path::new(&self.name);
        let mut fh = File::open(path)?;
        fh.read_to_string(&mut self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_invalid_file() {
        let mut f = GistFile::new(String::from("/not/found.txt"));
        assert!(f.read_file().is_err());
    }

    #[test]
    fn read_valid_file() {
        let mut f = GistFile::new(String::from("Cargo.toml"));
        assert!(f.read_file().is_ok());
    }
}
