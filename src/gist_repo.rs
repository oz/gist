use std::io;
use std::process::Command;

pub struct GistRepo {
    pub name: String,
}

impl GistRepo {
    pub fn clone(url: &str) -> bool {
        let mut child = Command::new("git")
            .arg("clone")
            .arg(&url)
            .spawn()
            .expect("Failed to run git");
        let ecode = child.wait().expect("Failed to clone repository");
        ecode.success()
    }

    pub fn git_https_url(url: String) -> String {
        if (&url).ends_with(".git") {
            return url;
        }
        let parts: Vec<&str> = url.split('/').collect();
        match parts.last() {
            Some(gist_id) => format!("https://gist.github.com/{}.git", gist_id),
            None => panic!("Can't find a repository here!"),
        }
    }
}
