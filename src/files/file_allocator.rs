use crate::errors::helper_errors::LeetCodeHelperError;
use std::fs;

pub struct FileAllocator;

impl FileAllocator {
    pub fn new() -> Self {
        FileAllocator {}
    }

    /// create appropriate directory
    fn create_directory(&self, file_name: DecomposedFileName) -> Result<(), std::io::Error> {
        let dir_name = Self::file_dir(file_name);
        fs::create_dir_all(path)
        Ok(())
    }

    /// move the file from the project top to appropriate directory that is defined in the "file_dir"
    /// function.
    fn move_file() {}

    /// determine file directory
    fn file_dir(file_name: DecomposedFileName) -> String {
        match file_name.extension.as_str() {
            "rs" => format!(
                "{}/src/bin/{}",
                file_name.extension,
                file_name.remove_extension()
            ),
            _ => format!("{}/{}", file_name.extension, file_name.remove_extension()),
        }
    }
}

/// decompose the file name to problem number, problem name, extension.
/// expected file name "problem-number.problem-name.extension"
struct DecomposedFileName {
    problem_number: String,
    problem_name: String,
    extension: String,
}

impl DecomposedFileName {
    pub fn new(file_name: &str) -> Result<Self, LeetCodeHelperError> {
        let mut splited_name = file_name.split(".").collect::<Vec<&str>>();
        let length = splited_name.len();

        if length < 3 {
            return Err(LeetCodeHelperError::ProblemFileNameLengthError(length));
        };

        let problem_number = splited_name.first().unwrap().to_string();
        let extension = splited_name.last().unwrap().to_string();
        let problem_name = splited_name[1..splited_name.len() - 1].join(".");

        Ok(DecomposedFileName {
            problem_number,
            problem_name,
            extension,
        })
    }

    pub(crate) fn remove_extension(&self) -> String {
        format!("{}.{}", self.problem_number, self.problem_name)
    }

    pub(crate) fn file_name(&self) -> String {
        format!(
            "{}.{}.{}",
            self.problem_number, self.problem_number, self.extension
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn decompose_file_name_new_test() {
        let decomposed_or_error = DecomposedFileName::new("1.sum.val.rs");

        assert!(decomposed_or_error.is_ok());

        let decomposed = decomposed_or_error.unwrap();

        assert_eq!(decomposed.problem_number, "1".to_string());
        assert_eq!(decomposed.problem_name, "sum.val".to_string());
        assert_eq!(decomposed.extension, "rs".to_string());
    }

    #[test]
    fn decompose_file_name_incorrect_file_name() {
        let decompose_or_error = DecomposedFileName::new("1.sum");

        assert!(decompose_or_error.is_err());
    }

    #[test]
    fn decompose_file_convination_test() {
        let decomposed = DecomposedFileName::new("1.sum.val.rs").unwrap();

        assert_eq!(decomposed.remove_extension(), "1.sum.val");
    }
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
