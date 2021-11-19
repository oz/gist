use std::process::Command;

const BASE_URL: &'static str = "https://gist.github.com/";

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

    // Examine val to find a git repository URL.
    pub fn find_url(val: &str) -> Option<String> {
        if !val.starts_with(BASE_URL) {
            return None;
        }
        if val.ends_with(".git") {
            return Some(val.to_string());
        }
        let parts: Vec<&str> = val.split('/').collect();

        match parts.last() {
            Some(gist_id) => Some(format!("https://gist.github.com/{}.git", gist_id)),
            None => None,
        }
    }
}
