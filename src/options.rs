use std::collections::HashMap;

use serde::{de, Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    /// Once enabled, if there's no `.gitignore` file
    /// in the given root folder, the program won't run.
    /// This is designed to prevent accidental call a error folder, because
    /// common repos managed by Git usually contains a `.gitignore` file.
    #[serde(default = "enable")]
    pub require_gitignore: bool,

    /// If enabled, the program will use `.gitignore` file to ignore files.
    /// You can also configure [`FileSyncMode::Enable`] to enable this feature
    /// for files ignore by `.gitignore` manually.
    #[serde(default = "enable")]
    pub use_gitignore: bool,

    /// Options details for specified files and folders.
    /// You can configure it in tree structure like this:
    ///
    /// ```json
    /// {
    ///     // other options...
    ///     "folder1": {
    ///         "file1": "enable",
    ///         "file2": "ignore"
    ///         // other files or folders...
    ///     },
    ///     "file3": "https://example.com/xxx"
    ///     // other files or folders...
    /// }
    /// ```
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

/// Encapsulation of `true` when configuring serde default value.
fn enable() -> bool {
    true
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
    use std::collections::HashMap;

    use super::{FileOrFolder, FileSyncMode, Options};

    #[test]
    fn json() {
        let raw = r#"
        {
            "requireGitignore": true,
            "customize": {
                "file1": "enable",
                "file2": "ignore",
                "file3": "invalid file config",
                "folder1": {
                    "file4": "auto",
                    "file5": "https://example.com/file"
                }
            }
        }
        "#;
        let options: Options = serde_json::from_str(raw).unwrap();

        // Normal options.
        assert!(options.require_gitignore); // Configured value.
        assert!(options.use_gitignore); // Default value.

        // Customize direct file.
        assert_eq!(
            options.customize.get("file1"),
            Some(&FileOrFolder::File(FileSyncMode::Enable))
        );
        assert_eq!(
            options.customize.get("file2"),
            Some(&FileOrFolder::File(FileSyncMode::Ignore))
        );
        assert_eq!(
            options.customize.get("file3"),
            Some(&FileOrFolder::File(FileSyncMode::SyncWholeFile(
                String::from("invalid file config")
            )))
        );

        // Custom folder.
        assert_eq!(
            options.customize.get("folder1"),
            Some(&FileOrFolder::Folder(HashMap::from([
                (
                    String::from("file4"),
                    FileOrFolder::File(FileSyncMode::Auto)
                ),
                (
                    String::from("file5"),
                    FileOrFolder::File(FileSyncMode::SyncWholeFile(
                        String::from("https://example.com/file")
                    ))
                )
            ])))
        );
    }
}
