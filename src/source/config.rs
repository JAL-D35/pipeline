use crate::source::*;

#[derive(serde::Deserialize)]
pub struct Config {
    data_go_kr: data_go_kr::config::Config,
}
