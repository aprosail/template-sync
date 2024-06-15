use std::collections::HashMap;

use serde::{de, Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub require_gitignore: bool,
    pub use_gitignore: bool,
    pub customize: HashMap<String, FileOrFolder>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase", untagged)]
pub enum FileOrFolder {
    File(FileSyncMode),
    Folder(HashMap<String, FileOrFolder>),
}

// #[serde(rename_all = "camelCase", untagged)]
#[derive(Debug, PartialEq)]
pub enum FileSyncMode {
    SyncWholeFile(String),
    Enable,
    Ignore,
    Auto,
}

impl<'de> Deserialize<'de> for FileSyncMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = FileSyncMode;

            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str(
                    "'auto', 'enable', 'ignore', or remote/relative url path",
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "auto" => Ok(FileSyncMode::Auto),
                    "enable" => Ok(FileSyncMode::Enable),
                    "ignore" => Ok(FileSyncMode::Ignore),
                    other => Ok(FileSyncMode::SyncWholeFile(other.to_owned())),
                }
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::{FileOrFolder, FileSyncMode, Options};

    #[test]
    fn test_json() {
        let raw = r#"
        {
            "requireGitignore": true,
            "useGitignore": true,
            "customize": {
                "file1": "enable",
                "file2": "ignore",
                "file3": "invalid file config"
            }
        }
        "#;
        let options: Options = serde_json::from_str(raw).unwrap();
        assert!(options.require_gitignore);
        assert!(options.use_gitignore);

        assert_eq!(
            options.customize.get("file1"),
            Some(&FileOrFolder::File(FileSyncMode::Enable)),
        );
        assert_eq!(
            options.customize.get("file2"),
            Some(&FileOrFolder::File(FileSyncMode::Ignore)),
        );
        assert_eq!(
            options.customize.get("file3"),
            Some(&FileOrFolder::File(FileSyncMode::SyncWholeFile(
                String::from("invalid file config")
            ))),
        );
    }
}
