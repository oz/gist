extern crate getopts;
extern crate gist;

use getopts::Options;

use std::env;
use std::io::{self, Write};
use std::process;

use gist::gist::Gist;
use gist::gist_file::GistFile;
use gist::gist_repo::GistRepo;
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
    opts.optflag("p", "public", "make things public");
    opts.optflagopt("l", "list", "list gists", "LOGIN");
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

// Fetch a bunch of gists and show them...
fn list_gists(gist: &mut Gist, login: Option<String>) {
    match gist.list(login.clone()) {
        Ok(list) => {
            if login.is_some() {
                println!("Latest Gists for {}:\n", login.unwrap());
            } else {
                println!("Latest public Gists:\n");
            }
            for item in list.iter() {
                println!(
                    "- {} ({})\n  {}",
                    item.html_url,
                    item.created_at,
                    item.description
                        .as_ref()
                        .unwrap_or(&("no desc".to_string()))
                );
            }
        }
        Err(msg) => println!("Failed to list gists: {:?}\n", msg),
    };
}

fn main() {
    let params = parse_args(env::args().collect());
    let is_public = params.opt_present("p");
    let desc = params.opt_str("d");
    let filename = match params.opt_str("f") {
        Some(name) => name,
        None => DEFAULT_GIST_NAME.to_string(),
    };
    let mut gist = Gist::new(is_public, desc);

    // List gists?
    if params.opt_present("l") {
        let login = params.opt_str("l");
        return list_gists(&mut gist, login);
    }

    // Read from stdin, unless we receive a bunch of filenames.
    if params.free.is_empty() {
        let mut g = GistFile::new(filename);
        if g.read_stdin().is_ok() {
            gist.add_file(g);
        }
    } else {
        for param in params.free {
            // Does that look like a gist URL?
            if let Some(url) = GistRepo::find_url(&param) {
                GistRepo::clone(&url);
                break;
            }
            // ... nope. Probably a file.
            let mut g = GistFile::new(param);
            match g.read_file() {
                Ok(_) => gist.add_file(g),
                Err(e) => fatal(&e.to_string()),
            }
        }
    }

    // Create a Gist if we got any files from flags or params...
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
