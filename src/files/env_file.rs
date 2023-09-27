use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

/// languages and its extensions
#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
struct LanguageAndExtensionJson {
    pub languageName: String,
    pub languageExtension: String,
}

pub struct LanguageAndExtension {
    pub extention_language: HashMap<String, String>,
}

impl LanguageAndExtension {
    /// read a languages.json on the top dir of this project.
    /// the structure of that file will be following:
    /// ```json
    /// [
    ///     {
    ///         languageName: "rust",
    ///         languageExtension: "rs"
    ///     },
    /// ]
    /// ```
    pub fn new() -> Self {
        let file_name = if cfg!(test) {
            "./test_data/test_language.json"
        } else {
            "language.json"
        };

        let mut extention_language: HashMap<String, String> = HashMap::new();

        let file = Self::open_language_file(file_name);
        let buf_reader = BufReader::new(file);

        let language_and_exetention_json: Vec<LanguageAndExtensionJson> =
            match serde_json::from_reader(buf_reader) {
                Ok(data) => data,
                Err(_) => panic!("reading the language.json error."),
            };

        language_and_exetention_json.iter().for_each(|lang_ex| {
            extention_language.insert(
                lang_ex.languageExtension.clone(),
                lang_ex.languageName.clone(),
            );
        });

        Self { extention_language }
    }

    //
    fn open_language_file(file_path: &str) -> File {
        match File::open(file_path) {
            Ok(file) => file,
            Err(_) => panic!("Cannot find 'language.json' file. make sure you deploy it to project root directory.")

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // #[should_panic(expected = "the .env file was not found, make sure it exists.")]
    // fn env_file_not_found() {
    //     TargetPath::read_file();
    // }

    #[test]
    fn language_extention_file_open() {
        let lang_extention = LanguageAndExtension::new();
        assert_eq!(lang_extention.extention_language.len(), 1);

        let first_one = &lang_extention.extention_language.get("rs");
        assert!(first_one.is_some());
        assert_eq!(first_one.unwrap(), "rust");
    }

    #[test]
    #[should_panic(
        expected = "Cannot find 'language.json' file. make sure you deploy it to project root directory."
    )]
    fn language_file_not_found() {
        LanguageAndExtension::open_language_file("nofile");
    }
}
