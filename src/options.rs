use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub require_gitignore: bool,
    pub use_gitignore: bool,
    pub customize: HashMap<String, FileOrFolder>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum FileOrFolder {
    File(FileSyncMode),
    Folder(HashMap<String, FileOrFolder>),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", untagged)]
pub enum FileSyncMode {
    SyncWholeFile(String),
    Enable,
    Ignore,
    Auto,
}

#[cfg(test)]
mod tests {
    use super::Options;

    #[test]
    fn test_deserialize() {
        let raw = r#"
        {
            "requireGitignore": true,
            "useGitignore": true,
            "customize": {
                "folder1": {
                    "file1": "https://example.com/file1",
                    "file2": "auto"
                },
                "file1": "enable",
                "file2": "ignore",
                "file3": "invalid property"
            }
        }
        "#;
        let options: Options = serde_json::from_str(raw).unwrap();
        assert!(options.require_gitignore);
        assert!(options.use_gitignore);
    }
}
