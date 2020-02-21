//! # path
//!
//! Path utility functions and traits.
//!

pub mod as_path;
pub mod from_path;

#[cfg(feature = "temp-path")]
mod temp_path;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

use crate::error::{ErrorInfo, FsIOError};
use as_path::AsPath;
use from_path::FromPath;

/// Returns a canonicalized string from the provided path value.
///
/// # Arguments
///
/// * `path` - The path value
///
/// # Example
///
/// ```
/// extern crate fsio;
///
/// use fsio::path;
/// use fsio::path::as_path::AsPath;
/// use std::path::Path;
///
/// fn main() {
///     let path_obj = Path::new("./src/path/mod.rs");
///
///     let path1 = path::canonicalize_as_string(&path_obj);
///     let path2 = path::canonicalize_as_string(&"./src/path/mod.rs".to_string());
///
///     assert_eq!(path1.unwrap(), path2.unwrap());
/// }
/// ```
pub fn canonicalize_as_string<T: AsPath + ?Sized>(path: &T) -> Result<String, FsIOError> {
    let path_obj = path.as_path();

    match path_obj.canonicalize() {
        Ok(path_buf) => {
            let path_string = FromPath::from_path(&path_buf);
            Ok(path_string)
        }
        Err(error) => Err(FsIOError {
            info: ErrorInfo::IOError("Unable to canonicalize path.".to_string(), Some(error)),
        }),
    }
}

/// Returns a canonicalized string from the provided path value.
///
/// # Arguments
///
/// * `path` - The path value
///
/// # Example
///
/// ```
/// extern crate fsio;
///
/// use fsio::path;
/// use fsio::path::as_path::AsPath;
/// use std::path::Path;
///
/// fn main() {
///     let path_obj = Path::new("./src/path/mod.rs");
///
///     let path1 = path::canonicalize_as_string(&path_obj);
///     let path2 = path::canonicalize_or("./src/path/mod.rs", "/src/path/mod.rs");
///
///     assert_eq!(path1.unwrap(), path2);
/// }
/// ```
pub fn canonicalize_or<T: AsPath + ?Sized>(path: &T, or_value: &str) -> String {
    match canonicalize_as_string(path) {
        Ok(value) => value,
        Err(_) => or_value.to_string(),
    }
}

/// Returns the last path component (file name or last directory name).
///
/// # Arguments
///
/// * `path` - The path value
///
/// # Example
///
/// ```
/// extern crate fsio;
///
/// use fsio::path;
/// use fsio::path::as_path::AsPath;
/// use std::path::Path;
///
/// fn main() {
///     let basename = path::get_basename("./src/path/mod.rs");
///
///     assert_eq!(basename.unwrap(), "mod.rs");
/// }
/// ```
pub fn get_basename<T: AsPath + ?Sized>(path: &T) -> Option<String> {
    let path_obj = path.as_path();

    match path_obj.file_name() {
        Some(name) => Some(name.to_string_lossy().into_owned()),
        None => None,
    }
}

/// Returns the parent path.
///
/// # Arguments
///
/// * `path` - The path value
///
/// # Example
///
/// ```
/// extern crate fsio;
///
/// use fsio::path;
/// use fsio::path::as_path::AsPath;
/// use std::path::Path;
///
/// fn main() {
///     let dirname = path::get_parent_directory("./src/path/mod.rs");
///
///     assert_eq!(dirname.unwrap(), "./src/path");
/// }
/// ```
pub fn get_parent_directory<T: AsPath + ?Sized>(path: &T) -> Option<String> {
    let path_obj = path.as_path();

    let directory = path_obj.parent();
    match directory {
        Some(directory_path) => {
            let directory_path_string: String = FromPath::from_path(directory_path);

            if directory_path_string.is_empty() {
                None
            } else {
                Some(directory_path_string)
            }
        }
        None => None,
    }
}

/// Returns a temporary file path.
/// The file does not exist after this function exists and can be used to create
/// a file in the OS temporary directory.
///
/// # Arguments
///
/// * `extension` - The file extension
///
/// # Feature
///
/// This function requires that the **temp-path** feature will be used.
///
/// # Example
///
/// ```
/// extern crate fsio;
///
/// use fsio::path;
///
/// fn main() {
///     let temp_file = path::get_temporary_file_path("txt");
///
///     assert!(temp_file.ends_with(".txt"));
/// }
/// ```
#[cfg(feature = "temp-path")]
pub fn get_temporary_file_path(extension: &str) -> String {
    temp_path::get(extension)
}