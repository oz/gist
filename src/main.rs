extern crate getopts;
#[macro_use]
extern crate serde_derive;

use getopts::Options;

use std::env;
use std::process;
use std::io::{self, Write};

mod gist;
use gist::Gist;
use gist::gist_file::GistFile;
use gist::response::decode;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const DEFAULT_GIST_NAME: &'static str = "Untitled";
const E_HELP: i32 = 1;
const E_FATAL: i32 = 2;

fn print_usage(program: &str, opts: Options) {
    let brief = opts.short_usage(program);
    print!("{}", opts.usage(&brief));
    process::exit(E_HELP);
}

fn print_version(program: &str) {
    println!("{} {}", program, VERSION);
    process::exit(E_HELP);
}

fn fatal(msg: &str) {
    io::stderr().write(msg.as_bytes()).ok();
    process::exit(E_FATAL);
}

fn parse_args(args: Vec<String>) -> getopts::Matches {
    let mut opts = Options::new();
    opts.optopt("f", "file", "set file name", "NAME");
    opts.optopt("d", "desc", "set description", "DESC");
    opts.optflag("p", "public", "make public");
    opts.optflag("a", "anonymous", "make anonymous");
    opts.optflag("h", "help", "print this");
    opts.optflag("v", "version", "show version");

    let params = opts.parse(&args[1..]).ok().expect("Unknown flag.");
    if params.opt_present("h") {
        print_usage(&args[0].clone(), opts);
    }
    if params.opt_present("v") {
        print_version(&args[0].clone());
    }

    params
}

fn main() {
    let params = parse_args(env::args().collect());
    let is_public = params.opt_present("p");
    let is_anonymous = params.opt_present("a");
    let desc = params.opt_str("d");
    let filename = match params.opt_str("f") {
        Some(name) => name,
        None => DEFAULT_GIST_NAME.to_string(),
    };
    let mut gist = Gist::new(is_public, is_anonymous, desc);

    // Read from stdin, unless we receive a bunch of filenames.
    if params.free.is_empty() {
        let mut g = GistFile::new(filename);
        if g.read_stdin().is_ok() {
            gist.add_file(g);
        }
    } else {
        for file_param in params.free {
            let mut g = GistFile::new(file_param);
            match g.read_file() {
                Ok(_) => gist.add_file(g),
                Err(e) => fatal(&e.to_string()),
            }
        }
    }

    if !gist.is_empty() {
        match gist.create() {
            Ok(response) => {
                let gist = decode(&response).unwrap();
                println!("{}", gist.html_url);
            }
            Err(e) => fatal(&e.to_string()),
        }
    }
}
