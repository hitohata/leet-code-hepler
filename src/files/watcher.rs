//! reads current directory, if the programming file detected, return.

use std::fs::read_dir;

use crate::errors::helper_errors::LeetCodeHelperError;
use crate::files::file_name_decomposer::DecomposedFileName;
use crate::languages::language_reader::LanguageAndExtension;

/// reads the current directory and returns program files.
pub fn programming_file_detector(
    path: &str,
    languages: &LanguageAndExtension,
) -> Result<Vec<DecomposedFileName>, LeetCodeHelperError> {
    // read current directory files
    let dirs = match read_dir(path) {
        Ok(dir) => dir,
        Err(e) => return Err(LeetCodeHelperError::IoError(e)),
    };

    let file_names: Vec<DecomposedFileName> = vec![];

    dirs.for_each(|f| {
        let file_entry = match f {
            Ok(entry) => entry,
            Err(e) => return Err(LeetCodeHelperError::IoError(e)),
        };

        let file_name = match file_entry.file_name().to_str() {
            Some(name) => name,
            None => return,
        };

        match DecomposedFileName::new(file_name) {
            Ok(decomposed) => file_names.push(decomposed),
            Err(_) => return, // ignore
        }
    });

    Ok(file_names)
}
