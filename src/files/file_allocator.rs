use super::file_name_decomposer::DecomposedFileName;
use crate::errors::helper_errors::LeetCodeHelperError;
use crate::files::language_files::{language_handler, LanguageHandler};

use std::fs;

struct FileAllocator<'a> {
    language: String,
    decomposed_file_name: &'a DecomposedFileName,
    file_handler: Box<dyn LanguageHandler>,
}

impl<'a> FileAllocator<'a> {
    fn new(language: String, decomposed_file_name: &'a DecomposedFileName) -> Self {
        let file_handler = language_handler(decomposed_file_name);
        Self {
            language,
            decomposed_file_name,
            file_handler,
        }
    }

    /// create directory
    fn create_directory(&self) -> Result<(), LeetCodeHelperError> {
        let dir_name = self
            .file_handler
            .file_dir(&self.language, &self.decomposed_file_name);
        if let Err(e) = fs::create_dir_all(dir_name) {
            return Err(LeetCodeHelperError::IoError(e));
        }
        Ok(())
    }

    /// move to appropriate folder
    fn move_file(&self) -> Result<(), LeetCodeHelperError> {
        let dir_name = self
            .file_handler
            .file_dir(&self.language, &self.decomposed_file_name);
        if let Err(e) = fs::rename(
            self.decomposed_file_name.file_name().to_string(),
            format!(
                "{}/{}",
                dir_name,
                self.decomposed_file_name.file_name().to_string()
            ),
        ) {
            return Err(LeetCodeHelperError::IoError(e));
        }
        Ok(())
    }
}
