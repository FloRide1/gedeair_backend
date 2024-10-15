//! File related models

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize)]
pub struct FileParams {
    #[serde(alias = "type")]
    pub file_type: FileType,
}

/// Enum representing the different types of files.
///
/// This enum is used to differentiate between various types of file.
/// It is deserialized from lowercase string values.
#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    None,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("");
    }
}
