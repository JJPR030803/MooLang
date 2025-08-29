use std::path::{Path, PathBuf};
use crate::utils::file_reader_errors::FileReaderError;

pub enum FileType {
    Moo(String),
    Muu(String),
}

impl FileType {
    pub fn check_extension(path: &Path) -> Result<FileType, FileReaderError> {
        if let Some(extension) = path.extension() {
            if extension == "moo" {
                Ok(FileType::Moo(path.to_str().unwrap().to_string()))
            } else if extension == "muu" {
                Ok(FileType::Muu(path.to_str().unwrap().to_string()))
            } else {
                return Err(FileReaderError::WrongFileType(
                    "File must have .moo (English) or .muu (Español) extension".to_string()
                ));
            }
        } else {
            return Err(FileReaderError::Other(
                "File has no extension".to_string()
            ));
        }
    }

    pub fn read_file(path: &Path) -> Result<String, FileReaderError> {
        let file_type = FileType::check_extension(path)?;

        match file_type {
            FileType::Muu(path_str) => {
                let content = std::fs::read_to_string(&path_str)
                    .map_err(|e| FileReaderError::Other(e.to_string()))?;
                FileType::process_muu_file(&content)
            },
            FileType::Moo(path_str) => {
                let content = std::fs::read_to_string(&path_str)
                    .map_err(|e| FileReaderError::Other(e.to_string()))?;
                FileType::process_moo_file(&content)
            }
        }
    }
    
    fn process_moo_file(content: &str) -> Result<String, FileReaderError> {
        if content.is_empty() {
            return Err(FileReaderError::EmptyFile(PathBuf::new()));
        }

        //TODO custom extension logic
        Ok(content.to_string())
    }

    fn process_muu_file(content: &str) -> Result<String, FileReaderError> {
        if content.is_empty() {
            return Err(FileReaderError::EmptyFile(PathBuf::new()));
        }
        Ok(content.to_string())
    }
}
