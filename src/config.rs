use std::io::Read;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {}

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

    use std::panic::catch_unwind;

    struct MockReader {
        content: String,
    }

    impl Read for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            buf[..self.content.len()].copy_from_slice(self.content.as_bytes());
            Ok(self.content.len())
        }
    }

    #[test]
    fn test_new_config_is_ok() {
        let reader = MockReader {
            content: "".to_string(),
        };

        let test = catch_unwind(|| {
            Config::new(reader);
        });

        assert!(test.is_ok());
    }
}
