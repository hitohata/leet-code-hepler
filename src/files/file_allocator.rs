use super::file_name_decomposer::DecomposedFileName;
use crate::errors::helper_errors::LeetCodeHelperError;
use std::fs;

trait FileHandler {
    /// create directory
    fn create_directory(&self, file_name: &DecomposedFileName) -> Result<(), LeetCodeHelperError> {
        let dir_name = Self::file_dir(file_name)?;
        if let Err(e) = fs::create_dir_all(dir_name) {
            return Err(LeetCodeHelperError::IoError(e));
        }
        Ok(())
    }

    /// move to appropriate folder
    fn move_file(&self, file_name: &DecomposedFileName) -> Result<(), LeetCodeHelperError> {
        let dir_name = Self::file_dir(file_name)?;
        if let Err(e) = fs::rename(
            file_name.file_name(),
            format!("{}/{}", dir_name, file_name.file_name()),
        ) {
            return Err(LeetCodeHelperError::IoError(e));
        }
        Ok(())
    }

    /// determine file directory
    fn file_dir(file_name: &DecomposedFileName) -> Result<String, LeetCodeHelperError>;
}

/// general language
struct GeneralAllocator;

impl FileHandler for GeneralAllocator {
    fn file_dir(file_name: &DecomposedFileName) -> Result<String, LeetCodeHelperError> {
        let require_extension = "rs";
        if file_name.extension() != require_extension {
            return Err(LeetCodeHelperError::ExtensionMismatchError(
                require_extension.to_string(),
                file_name.extension().to_string(),
            ));
        };

        Ok(format!(
            "{}/src/{}",
            file_name.extension(),
            file_name.remove_extension()
        ))
    }
}

/// for Rust
struct RustAllocator;

impl FileHandler for RustAllocator {
    fn file_dir(file_name: &DecomposedFileName) -> Result<String, LeetCodeHelperError> {
        let require_extension = "rs";
        if file_name.extension() != require_extension {
            return Err(LeetCodeHelperError::ExtensionMismatchError(
                require_extension.to_string(),
                file_name.extension().to_string(),
            ));
        };

        Ok(format!(
            "{}/src/bin/{}",
            file_name.extension(),
            file_name.remove_extension()
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    //
    // #[test]
    // fn file_dir_test() {
    //     let file_allicator = FileAllocator::new();
    //
    //     assert_eq!(
    //         FileAllocator::file_dir("1.sum.ts", "typescript"),
    //         "typescript/1.sum"
    //     );
    // }
}
