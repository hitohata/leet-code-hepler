//! This is leet-code helper.
//! This code watches the project root. The leet-code plugin gets the problem and saves it to the project root.
//! Then this tool moves it to language-name/number-of-problem/file-name.
//! if the language is rust, also add bin information to the Cargo.toml
//! To detect the extension of language and to map the language name to it, need the language.json file.

mod cli;
mod errors;
mod files;
mod languages;

use std::sync::mpsc::channel;

use cli::arguments::Cli;
use files::file_allocator::FileAllocator;
use files::file_name_decomposer::DecomposedFileName;
use files::watcher::{file_detector, programming_file_detector, watcher};
use languages::language_reader::LanguageAndExtension;

fn main() {
    let arguments = cli::arguments::arguments();

    let target_directory = target_directory(&arguments);
    let languages = LanguageAndExtension::new(&arguments.language_path);

    let current_dir_files = match programming_file_detector(&target_directory) {
        Ok(files) => files,
        Err(e) => panic!("{}", e),
    };

    // read the current directory.
    current_dir_files
        .iter()
        .for_each(|f| file_allocate(&languages, f));

    // watch directory
    let (tx, rx) = channel();

    let w = watcher(&target_directory, tx);

    for e in rx {
        if let Ok(file_name) = file_detector(e) {
            file_allocate(&languages, &file_name)
        };
    }
}

fn file_allocate(languages: &LanguageAndExtension, file_name: &DecomposedFileName) {
    if let Some(file) = languages.get_language_name(&file_name.extension()) {
        let allocator = FileAllocator::new(file, file_name);
        let _ = allocator.allocate();
    }
}

fn target_directory(directory: &Cli) -> String {
    // check arguments
    if directory.path.is_some() {
        return directory.path.to_owned().unwrap();
    };

    // if path is not provided as an argument.
    // get a current path.
    let dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => panic!("directory not found"),
    };

    let target_direcroy = match dir.to_str() {
        Some(d) => d,
        None => panic!("directory not found"),
    };

    target_direcroy.to_string()
}
