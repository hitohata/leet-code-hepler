use super::file_name_decomposer::DecomposedFileName;
use crate::errors::helper_errors::LeetCodeHelperError;
use crate::files::language_files::{language_handler, LanguageHandler};

use std::fs;

pub struct FileAllocator<'a> {
    language: String,
    decomposed_file_name: &'a DecomposedFileName,
    file_handler: Box<dyn LanguageHandler>,
}

impl<'a> FileAllocator<'a> {
    pub fn new(language: String, decomposed_file_name: &'a DecomposedFileName) -> Self {
        let file_handler = language_handler(decomposed_file_name);
        Self {
            language,
            decomposed_file_name,
            file_handler,
        }
    }

    /// move a file from the top directory to an appropriate folder.
    pub fn allocate(&self) -> Result<(), LeetCodeHelperError> {
        let directory_name = self
            .file_handler
            .file_dir(&self.language, &self.decomposed_file_name);

        self.create_directory(&directory_name)?;
        self.move_file(&directory_name)?;
        let _ = self.generate_markdown_file(
            &directory_name,
            &self.decomposed_file_name.remove_extension(),
        );
        Ok(())
    }

    /// create directory
    fn create_directory(&self, direcrroy_name: &str) -> Result<(), LeetCodeHelperError> {
        if let Err(e) = fs::create_dir_all(direcrroy_name) {
            return Err(LeetCodeHelperError::IoError(e));
        }
        Ok(())
    }

    /// move to appropriate folder
    fn move_file(&self, directory_name: &str) -> Result<(), LeetCodeHelperError> {
        if let Err(e) = fs::rename(
            self.decomposed_file_name.file_name().to_string(),
            format!(
                "{}/{}",
                directory_name,
                self.decomposed_file_name.file_name().to_string()
            ),
        ) {
            return Err(LeetCodeHelperError::IoError(e));
        }
        Ok(())
    }

    /// add markdown file for taking notes.
    fn generate_markdown_file(
        &self,
        directory_name: &str,
        file_name: &str,
    ) -> Result<(), LeetCodeHelperError> {
        let markdown_text = format!("# {}", file_name);
        let path = format!("{}/{}.md", directory_name, file_name);
        let _ = fs::write(path, markdown_text);
        Ok(())
    }
}
