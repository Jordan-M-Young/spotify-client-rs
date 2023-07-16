use serde::Deserialize;


pub fn load_file_string(filepath: &str) -> Result<String, FromUtf8Error> {
    let file_bytes = fs::read(filepath).expect("couldn't load file");

    let file_string = String::from_utf8(file_bytes)?;
    Ok(file_string)
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct SpotifyConfig {
    pub key: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct AppConfig {
    pub port: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct TomlConfig {
    pub spotify: SpotifyConfig,
    pub app: AppConfig,
}

pub fn load_toml(toml_filename: &str) -> TomlConfig {
    let toml_string = load_file_string(toml_filename).unwrap();
    let toml_config: TomlConfig = toml::from_str(&toml_string).unwrap();
    toml_config
}
