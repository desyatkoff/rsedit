use std::{
    path::{
        PathBuf,
        Path,
    },
    fmt,
    fmt::Display,
};

#[derive(Default, Debug)]
pub struct FileInfo {
    file_path: Option<PathBuf>,
}

impl FileInfo {
    pub fn from(file_name: &str) -> Self {
        return Self {
            file_path: Some(PathBuf::from(file_name)),
        };
    }

    pub fn get_path(&self) -> Option<&Path> {
        return self.file_path.as_deref();
    }

    pub const fn has_path(&self) -> bool {
        return self.file_path.is_some();
    }
}

impl Display for FileInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = self
            .get_path()
            .and_then(
                |path| path.file_name()
            )
            .and_then(
                |name| name.to_str()
            )
            .unwrap_or("No file open");

        return write!(
            formatter,
            "{result}"
        );
    }
}