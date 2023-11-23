//! reads current directory, if the programming file detected, return.

use std::path::Path;
use std::{fs::read_dir, sync::mpsc::Sender};

use crate::errors::helper_errors::LeetCodeHelperError;
use crate::files::file_name_decomposer::DecomposedFileName;
use notify::{Config, Error, Event, INotifyWatcher, RecommendedWatcher, RecursiveMode, Watcher};

/// reads the current directory and returns program files.
pub fn programming_file_detector(
    path: &str,
) -> Result<Vec<DecomposedFileName>, LeetCodeHelperError> {
    // read current directory files
    let dirs = match read_dir(path) {
        Ok(dir) => dir,
        Err(e) => return Err(LeetCodeHelperError::IoError(e)),
    };

    let mut file_names: Vec<DecomposedFileName> = vec![];

    dirs.for_each(|f| {
        let file_entry = match f {
            Ok(entry) => entry,
            Err(_) => return,
        };

        let file_name_or_error = file_entry.file_name();

        let file_name = match file_name_or_error.to_str() {
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

pub fn watcher(
    path: &str,
    tx: Sender<Result<Event, Error>>,
) -> Result<INotifyWatcher, LeetCodeHelperError> {
    let config = Config::default();

    let mut watcher = match RecommendedWatcher::new(tx, config) {
        Ok(watcher) => watcher,
        Err(e) => return Err(LeetCodeHelperError::NotifyError(e)),
    };

    if let Err(err) = watcher.watch(Path::new(path), RecursiveMode::NonRecursive) {
        return Err(LeetCodeHelperError::NotifyError(err));
    };

    Ok(watcher)
}

pub fn file_detector(e: Result<Event, Error>) -> Result<DecomposedFileName, LeetCodeHelperError> {
    let event = match e {
        Ok(event) => event,
        Err(err) => return Err(LeetCodeHelperError::NotifyError(err)),
    };
    let path = match event.paths[0].to_str() {
        Some(path) => path,
        None => return Err(LeetCodeHelperError::NotifyPathNotFound),
    };

    let file_name = match path.clone().split("/").last() {
        Some(file_name) => file_name,
        None => return Err(LeetCodeHelperError::NotifyPathNotFound),
    };

    return match event.kind {
        notify::EventKind::Create(_) => DecomposedFileName::new(file_name),
        _ => Err(LeetCodeHelperError::NotifyKindNotMatch),
    }
}
