//! Handles language specific .

use crate::errors::helper_errors::LeetCodeHelperError;
use crate::files::file_name_decomposer::DecomposedFileName;
use toml_edit::{value, ArrayOfTables, Document, Item, Table};

/// generate a language handler
pub(crate) fn language_handler(
    decomposed_file_name: &DecomposedFileName,
) -> Box<dyn LanguageHandler> {
    match decomposed_file_name.extension().as_str() {
        "rs" => Box::new(RustHandler),
        _ => Box::new(GeneralHandler),
    }
}

pub(crate) trait LanguageHandler {
    /// determine file directory
    fn file_dir(&self, language_name: &str, file_name: &DecomposedFileName) -> String;
    fn language_specific_process(
        &self,
        _file_name: &DecomposedFileName,
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
            "{}/{}",
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
        let _ = std::fs::write(cargo_toml, updated);
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
