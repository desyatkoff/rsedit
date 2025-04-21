use std::{
    fmt,
    fmt::Display,
    path::PathBuf,
};

#[derive(Default, Debug, Clone)]
pub struct FileInfo {
    pub file_path: Option<PathBuf>,
}

impl FileInfo {
    pub fn from(file_name: &str) -> Self {
        return Self {
            file_path: Some(PathBuf::from(file_name)),
        };
    }
}

impl Display for FileInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = self
            .file_path
            .as_ref()
            .and_then(
                |path| path.file_name()
            )
            .and_then(
                |name| name.to_str()
            )
            .unwrap_or("UNTITLED");

        return write!(
            formatter,
            "{result}"
        );
    }
}

