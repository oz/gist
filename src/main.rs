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
use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::collections::BTreeMap;

const GIST_API:          &'static str = "https://api.github.com/gists";
const GITHUB_TOKEN:      &'static str = "GITHUB_TOKEN";
const USER_AGENT:        &'static str = "Pepito Gist";
const DEFAULT_GIST_NAME: &'static str = "Untitled";

struct Gist {
    filename: String,
    private:  bool,
    contents: String,
}
type Gists = Vec<Gist>;

#[derive(RustcDecodable)]
struct GistResponse {
    html_url: String,
}

impl Gist {
    fn new(filename: String, public: bool) -> Gist {
        Gist {
            private:  !public,
            filename: filename,
            contents: String::new(),
        }
    }

    // Read standard input to contents buffer.
    fn read_stdin(&mut self) -> std::io::Result<usize> {
        std::io::stdin().read_to_string(&mut self.contents)
    }

    // Read file to contents buffer.
    fn read_file(&mut self) -> std::io::Result<usize> {
        let path = Path::new(&self.filename);
        let mut fh = File::open(&path).unwrap();

        fh.read_to_string(&mut self.contents)
    }
}

impl ToJson for Gist {
    fn to_json(&self) -> Json {
        let mut root = BTreeMap::new();
        root.insert("content".to_string(), self.contents.to_json());
        Json::Object(root)
    }
}

// Convert a Gist vector to JSON, suitable for Github's Gist API.
fn gists_to_json(gists : Gists) -> Json {
    let mut root  = BTreeMap::new();
    let mut files = BTreeMap::new();

    root.insert("public".to_string(), (!gists[0].private).to_json());
    for g in gists {
        files.insert(g.filename.clone(), g);
    }
    root.insert("files".to_string(), files.to_json());

    Json::Object(root)
}

fn create_gists(gists : Gists) -> Result<String, error::Error> {
    let client = Client::new();
    let token : String = env::var(&GITHUB_TOKEN.to_string()).unwrap();

    let json_body = gists_to_json(gists).to_string();
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
    let opt_name = match params.opt_str("f") {
        Some(name) => name,
        None       => DEFAULT_GIST_NAME.to_string(),
    };
    let mut gists: Gists = vec![];

    // If we receive filenames, read them, else use STDIN.
    if !params.free.is_empty() {
        for file_param in params.free {
            let mut g = Gist::new(file_param, public);
            g.read_file().unwrap();
            gists.push(g);
        }
    } else {
        let mut gist = Gist::new(opt_name, public);
        gist.read_stdin().unwrap();
        gists.push(gist);
    }

    let res = create_gists(gists);
    if res.is_ok() {
        let gist: GistResponse = json::decode(&res.unwrap()).unwrap();
        println!("{}", gist.html_url);
    }
}
