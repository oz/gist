#[macro_use]
extern crate serde_derive;

extern crate serde_json;
#[macro_use]
extern crate failure;
extern crate ureq;

pub mod error;
pub mod gist;
pub mod gist_file;
pub mod response;
