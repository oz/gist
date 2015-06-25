extern crate getopts;
extern crate rustc_serialize;
#[macro_use] extern crate hyper;

use getopts::Options;
use hyper::{Client, header, error};
use hyper::header::{Authorization, Bearer};
use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};

use std::env;
use std::process;
use std::io::{self, Read};
use std::path::Path;
use std::fs::File;
use std::collections::BTreeMap;

const GIST_API:          &'static str = "https://api.github.com/gists";
const GITHUB_TOKEN:      &'static str = "GITHUB_TOKEN";
const USER_AGENT:        &'static str = "Pepito Gist";
const DEFAULT_GIST_NAME: &'static str = "Untitled";

struct Gist {
    filename: String,
    contents: String,
}

struct Gists {
    public: bool,
    files:  Vec<Gist>,
}

impl Gists {
    fn new(public: bool) -> Gists {
        Gists {
            public: public,
            files:  vec![],
        }
    }

    // Add a file.
    fn push(&mut self, gist : Gist) {
        self.files.push(gist);
    }

    // Sent to Github.
    fn create(&mut self) -> Result<String, error::Error> {
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
    fn new(filename: String) -> Gist {
        Gist {
            filename: filename,
            contents: String::new(),
        }
    }

    // Read standard input to contents buffer.
    fn read_stdin(&mut self) -> Result<&Gist, io::Error> {
        try!(io::stdin().read_to_string(&mut self.contents));
        Ok(self)
    }

    // Read file to contents buffer.
    fn read_file(&mut self) -> Result<&Gist, io::Error> {
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

#[derive(RustcDecodable)]
struct GistResponse {
    html_url: String,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn parse_args(args : Vec<String>) -> getopts::Matches {
    let mut opts = Options::new();
    opts.optopt("f", "file", "set file name", "NAME");
    opts.optflag("p", "public", "make public");
    opts.optflag("h", "help", "print this help menu");

    let params = match opts.parse(&args[1..]) {
        Ok(m)  => m,
        Err(f) => panic!(f.to_string())
    };
    if params.opt_present("h") {
        print_usage(&args[0].clone(), opts);
        process::exit(1);
    }

    params
}

fn main() {
    let params = parse_args(env::args().collect());
    let public = params.opt_present("p");
    let filename = match params.opt_str("f") {
        Some(name) => name,
        None       => DEFAULT_GIST_NAME.to_string(),
    };
    let mut gists = Gists::new(public);

    // If we receive filenames, read them, else use STDIN.
    if !params.free.is_empty() {
        for file_param in params.free {
            let mut g = Gist::new(file_param);
            g.read_file().ok().expect("Cannot read file");
            gists.push(g);
        }
    } else {
        let mut g = Gist::new(filename);
        if g.read_stdin().is_ok() { gists.push(g); }
    }

    if !gists.files.is_empty() {
        match gists.create() {
            Ok(r) => {
                let gist: GistResponse = json::decode(&r).unwrap();
                println!("{}", gist.html_url);
            },
            Err(e) => panic!("{}", e)
        }
    }
}
