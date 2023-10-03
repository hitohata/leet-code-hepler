//! This is leet-code helper.
//! This code watches the project root. The leet-code plugin gets the problem and saves it to the project root.
//! Then this tool moves it to language-name/number-of-problem/file-name.
//! if the language is rust, also add bin information to the Cargo.toml
//! To detect the extension of language and to map the language name to it, need the language.json file.

mod errors;
mod files;
mod languages;

use files::file_allocator::FileAllocator;
use files::file_name_decomposer::DecomposedFileName;
use files::watcher::file_detector;
use files::watcher::programming_file_detector;
use files::watcher::watcher;
use languages::language_reader::LanguageAndExtension;
use std::sync::mpsc::channel;

fn main() {
    let dir = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => panic!("directory not found"),
    };

    let target_direcroy = match dir.to_str() {
        Some(d) => d,
        None => panic!("directory not found"),
    };

    let languages = LanguageAndExtension::new();

    let current_dir_files = match programming_file_detector(&target_direcroy) {
        Ok(files) => files,
        Err(e) => panic!("{}", e),
    };

    // read the current directory.
    current_dir_files
        .iter()
        .for_each(|f| file_allocate(&languages, f));

    let (tx, rx) = channel();

    let _ = watcher(target_direcroy, tx);

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
