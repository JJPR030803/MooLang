use std::path::PathBuf;
use std::io;

/// Represents various errors that can occur while reading a file.
///
/// # Variants
///
/// - `FileNotFound(PathBuf)`
///   Indicates that the specified file could not be found.
///   - `PathBuf`: The path of the file that was not found.
///
/// - `PermissionDenied(PathBuf)`
///   Indicates that the application does not have permission to access the specified file.
///   - `PathBuf`: The path of the file for which permission was denied.
///
/// - `InvalidPath(PathBuf)`
///   Indicates that the provided file path is invalid.
///   - `PathBuf`: The invalid file path.
///
/// - `IoError(io::Error)`
///   Indicates that an I/O error occurred while attempting to read the file.
///   - `io::Error`: The specific I/O error that occurred.
///
/// - `EmptyFile(PathBuf)`
///   Indicates that the specified file is empty and contains no data.
///   - `PathBuf`: The path of the empty file.
///
/// - `FileTooLarge { path: PathBuf, size: u64, max_size: u64 }`
///   Indicates that the file size exceeds a predefined maximum limit.
///   - `path`: The path of the file that is too large.
///   - `size`: The size of the file in bytes.
///   - `max_size`: The maximum allowed size of the file in bytes.
///
/// - `EncodingError { path: PathBuf, message: String }`
///   Indicates that an error occurred while decoding the file contents due to encoding issues.
///   - `path`: The path of the file that caused the encoding error.
///   - `message`: A descriptive error message providing additional context on the encoding issue.
///
/// This enum is used to encapsulate various file-related issues, 
/// providing detailed information about the cause and allowing for more robust error handling.
#[derive(Debug)]
pub enum FileReaderError {
    FileNotFound(PathBuf),
    PermissionDenied(PathBuf),
    InvalidPath(PathBuf),
    IoError(io::Error),
    EmptyFile(PathBuf),
    FileTooLarge { path: PathBuf, size: u64, max_size: u64 },
    EncodingError { path: PathBuf, message: String },
    WrongFileType(String),
    Other(String),
}

impl std::fmt::Display for FileReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileReaderError::FileNotFound(path) => write!(f, "File not found: {}", path.display()),
            FileReaderError::PermissionDenied(path) => write!(f, "Permission denied: {}", path.display()),
            FileReaderError::InvalidPath(path) => write!(f, "Invalid path: {}", path.display()),
            FileReaderError::IoError(error) => write!(f, "IO error: {}", error),
            FileReaderError::EmptyFile(path) => write!(f, "Empty file: {}", path.display()),
            FileReaderError::FileTooLarge { path, size, max_size } => {
                write!(f, "File too large: {} (size: {}, max: {})", path.display(), size, max_size)
            }
            FileReaderError::EncodingError { path, message } => {
                write!(f, "Encoding error in {}: {}", path.display(), message)
            }
            FileReaderError::WrongFileType(message) => write!(f, "Wrong file type: {}", message),
            FileReaderError::Other(message) => write!(f, "Other error: {}", message),
        }
    }
}

impl std::error::Error for FileReaderError {}

pub fn encoding_error(path: PathBuf, message: String) -> FileReaderError {
    FileReaderError::EncodingError { path, message }
}
