use crate::source;
use std::io::Read;

#[derive(serde::Deserialize)]
pub struct Config {
    source: source::config::Config,
}

impl Config {
    pub fn new(reader: impl Read) -> Self {
        Self::parse_config_file(reader)
    }

    fn parse_config_file(reader: impl Read) -> Self {
        serde_yaml::from_reader(reader).expect("Failed to parse config file")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::io::Cursor;
    use std::panic::catch_unwind;

    #[test]
    fn test_new_config_is_ok() {
        let yaml_str = "\
            source:\n\
            +data_go_kr:\n\
            ++host: host_for_test\n\
            ++service_key: service_key_for_test\n"
            .replace("+", " ");

        let reader = Cursor::new(yaml_str);

        let test = catch_unwind(|| {
            Config::new(reader);
        });

        assert!(test.is_ok());
    }

    #[test]
    fn test_new_config_is_ok_without_data_go_kr_host() {
        let yaml_str = "\
            source:\n\
            +data_go_kr:\n\
            ++service_key: service_key_for_test\n"
            .replace("+", " ");

        let reader = Cursor::new(yaml_str);

        let test = catch_unwind(|| {
            Config::new(reader);
        });

        assert!(test.is_ok());
    }
}
