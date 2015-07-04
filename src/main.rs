extern crate rustc_serialize;
extern crate getopts;

use getopts::Options;
use rustc_serialize::json;

use std::env;
use std::process;

mod gists;

const DEFAULT_GIST_NAME: &'static str = "Untitled";

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
    opts.optflag("a", "anonymous", "make anonymous");
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
    let anonymous = params.opt_present("a");
    let filename = match params.opt_str("f") {
        Some(name) => name,
        None       => DEFAULT_GIST_NAME.to_string(),
    };
    let mut gists = gists::Gists::new(public, anonymous);

    // If we receive filenames, read them, else use STDIN.
    if !params.free.is_empty() {
        for file_param in params.free {
            let mut g = gists::GistFile::new(file_param);
            g.read_file().ok().expect("Cannot read file");
            gists.push(g);
        }
    } else {
        let mut g = gists::GistFile::new(filename);
        if g.read_stdin().is_ok() { gists.push(g); }
    }

    if !gists.is_empty() {
        match gists.create() {
            Ok(r) => {
                let gist: GistResponse = json::decode(&r).unwrap();
                println!("{}", gist.html_url);
            },
            Err(e) => panic!("{}", e)
        }
    }
}
