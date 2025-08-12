use std::fs;

const CONFIG_FILE: &str = "config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub gist_token: String,
}

impl Config {
    pub fn read() -> Option<Self> {
        if let Some(config_path) = dirs_next::home_dir().map(|home_dir| {
            home_dir
                .join(format!(".{}", env!("CARGO_PKG_NAME")))
                .join(CONFIG_FILE)
        }) && let Ok(contents) = fs::read_to_string(config_path)
        {
            return serde_json::from_str(&contents).ok();
        }
        None
    }
}
