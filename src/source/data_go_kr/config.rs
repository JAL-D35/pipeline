#[derive(serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    pub(crate) host: String,
    pub(crate) service_key: String,
}

fn default_host() -> String {
    "http://apis.data.go.kr".to_string()
}
