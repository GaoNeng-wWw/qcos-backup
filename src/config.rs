use std::path::PathBuf;

use config::Config;

use serde::Deserialize;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct AppConfig {
    pub glob:Option<String>,
    pub pub_key_name: Option<String>,
    pub pri_key_name: Option<String>,
    pub chunk_limit: Option<u64>,
    pub access_key_id: String,
    pub secret_access_key:String,
    pub region:String,
    pub endpoint:String,
    pub bucket: String,
    pub password: String
}

pub fn config(root:&PathBuf) -> AppConfig{
    let conf = Config::builder()
    .add_source(
        config::File::with_name(root.join("config.toml").display().to_string().as_ref())
    )
    .build()
    .unwrap();
    let config:AppConfig = conf.try_deserialize().unwrap();
    config
}