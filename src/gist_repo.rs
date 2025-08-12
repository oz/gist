use std::process::Command;

const BASE_URL: &str = "https://gist.github.com/";

pub struct GistRepo {
    pub name: String,
}

impl GistRepo {
    pub fn clone(url: &str) -> bool {
        let mut child = Command::new("git")
            .arg("clone")
            .arg(url)
            .spawn()
            .expect("Failed to run git");
        let ecode = child.wait().expect("Failed to clone repository");
        ecode.success()
    }

    // Examine val to find a git repository URL.
    pub fn find_url(val: &str) -> Option<String> {
        if !val.starts_with(BASE_URL) || val.len() <= BASE_URL.len() {
            return None;
        }
        if val.ends_with(".git") {
            return Some(val.to_string());
        }
        let parts: Vec<&str> = val.split('/').collect();

        parts
            .last()
            .map(|gist_id| format!("https://gist.github.com/{}.git", gist_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_url_with_url() {
        let res = GistRepo::find_url("https://gist.github.com/abcd");
        assert_eq!(res.is_some(), true);
        if let Some(url) = res {
            assert_eq!(url, "https://gist.github.com/abcd.git");
        }
    }

    #[test]
    fn find_url_with_git_repo_url() {
        let res = GistRepo::find_url("https://gist.github.com/abcd.git");
        assert_eq!(res.is_some(), true);
        if let Some(url) = res {
            assert_eq!(url, "https://gist.github.com/abcd.git");
        }
    }

    #[test]
    fn find_url_with_non_url() {
        let mut res = GistRepo::find_url("lalala");
        assert!(res.is_none());

        res = GistRepo::find_url("https://gist.github.com/");
        assert!(res.is_none());

        res = GistRepo::find_url("https://gist.github.com");
        assert!(res.is_none());
    }
}
