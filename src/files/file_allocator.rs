use toml_edit::{value, ArrayOfTables, Document, Item, Table};

use super::file_name_decomposer::DecomposedFileName;
use crate::errors::helper_errors::LeetCodeHelperError;
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

/// generate a language handler
pub fn language_handler(decomposed_file_name: &DecomposedFileName) -> Box<dyn LanguageHandler> {
    match decomposed_file_name.extension().as_str() {
        "rs" => Box::new(RustHandler),
        _ => Box::new(GeneralHandler),
    }
}

trait LanguageHandler {
    /// determine file directory
    fn file_dir(&self, language_name: &str, file_name: &DecomposedFileName) -> String;
    ///
    fn language_specific_process(
        &self,
        file_name: &DecomposedFileName,
    ) -> Result<(), LeetCodeHelperError> {
        Ok(())
    }
}

/// general language
struct GeneralHandler;
/// for Rust
struct RustHandler;

impl LanguageHandler for GeneralHandler {
    fn file_dir(&self, language_name: &str, file_name: &DecomposedFileName) -> String {
        format!(
            "{}/src/{}",
            language_name,
            file_name.remove_extension().to_string()
        )
    }
}

impl LanguageHandler for RustHandler {
    fn file_dir(&self, language_name: &str, file_name: &DecomposedFileName) -> String {
        format!(
            "{}/src/bin/{}",
            language_name,
            file_name.remove_extension().to_string()
        )
    }

    /// read the Cargo.toml and add a [[bin]] table to it.
    fn language_specific_process(
        &self,
        file_name: &DecomposedFileName,
    ) -> Result<(), LeetCodeHelperError> {
        let cargo_toml = "./rust/Cargo.toml";

        // read
        let contents = match std::fs::read_to_string(cargo_toml) {
            Ok(c) => c,
            Err(e) => return Err(LeetCodeHelperError::IoError(e)),
        };

        let mut toml_doc = match contents.parse::<Document>() {
            Ok(doc) => doc,
            Err(e) => return Err(LeetCodeHelperError::TomlError(e)),
        };

        // add a [[bin]] table
        if let Some(Item::ArrayOfTables(bin_array)) = toml_doc.get_mut("bin") {
            // there are already bin, add new one.
            bin_array.push(self.bin_table(file_name));
        } else {
            // there isn't bin, create new one.
            let mut new_array = ArrayOfTables::new();

            new_array.push(self.bin_table(file_name));

            toml_doc["bin"] = Item::ArrayOfTables(new_array);
        }

        let updated = toml_doc.to_string();

        std::fs::write(cargo_toml, updated);

        Ok(())
    }
}

impl RustHandler {
    /// return a new bin table
    fn bin_table(&self, file_name: &DecomposedFileName) -> Table {
        let mut bin_table = Table::new();
        bin_table["name"] = value(file_name.problem_number());
        bin_table["path"] = value(format!(
            "rust/src/bin/{}/{}",
            file_name.remove_extension(),
            file_name.file_name()
        ));

        bin_table
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
