use crate::{Error, Result};
use content_inspector::{inspect, ContentType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use tinyfiledialogs::{
    open_file_dialog, open_file_dialog_multi, save_file_dialog, select_folder_dialog,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileResult {
    /// A basic representation of a file. Note- Don't use this struct directly
    /// use helper functions from filesystem module to build this struct.

    /// Path of the file
    pub path: String,
    /// Data of a file in bytes.
    pub bytes: Vec<u8>,
    /// Metadata of file.
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// MetaData of a file or folder.

    /// returns true if a path points to a folder.
    pub is_dir: bool,
    /// returns true if a path points to a file.
    pub is_file: bool,
    /// returns true if it is a  text file.
    pub is_text: bool,
    /// returns true if it is a binary file.
    pub is_binary: bool,
    /// returns the length size of a file in bytes.
    pub size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct File {
    /// Representation of a file.
    pub name: String,
    pub metadata: Metadata,
}

impl File {
    fn new(name: String, metadata: Metadata) -> Self {
        Self { name, metadata }
    }
}

impl FileResult {
    pub fn new(path: String, bytes: Vec<u8>, metadata: Metadata) -> Self {
        // construct a new file struct.
        Self {
            path,
            bytes,
            metadata,
        }
    }
}

impl Metadata {
    fn from(metadata: fs::Metadata, is_text: bool, is_binary: bool) -> Self {
        // construct metadata of a file and only includeds relevant fields.
        Metadata {
            size: metadata.len(),
            is_dir: metadata.is_dir(),
            is_file: metadata.is_file(),
            is_text,
            is_binary,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FilePath {
    // this enum tells ezgui if a user has choosen multiple or a single file.
    /// Path of a selected file.
    Single(String),
    /// list of file paths.
    Multiple(Vec<String>),
}

pub fn open_dialog(multiple: bool) -> Result<FilePath> {
    // function for opening a native file chooser dialog.
    if multiple {
        let path = open_file_dialog_multi("select file", ".", None);
        match path {
            Some(path) => Ok(FilePath::Multiple(path)), // return file path
            None => Err(Error::DialogError {
                detail: String::from("User did not selected any file."),
            }), // return None if no path has been selected.
        }
    } else {
        let path = open_file_dialog("select file", ".", None);
        match path {
            Some(path) => Ok(FilePath::Single(path)), // return file path
            None => Err(Error::DialogError {
                detail: String::from("User did not selected any file."),
            }), // return None if no path has been selected.
        }
    }
}

pub fn select_folder() -> Result<String> {
    // function for opening a native dialog for selecting a folder.
    // returns path of a selected folder.
    match select_folder_dialog("select folder", ".") {
        Some(path) => Ok(path),
        None => Err(Error::DialogError {
            detail: String::from("User did not selected any file."),
        }),
    }
}

/// function for saving bytes of data to a file.
pub fn save_file<P: std::convert::AsRef<Path>>(
    path: P,
    content: &[u8],
    mode: String,
) -> Result<String> {
    use fs::OpenOptions;

    if path.as_ref().exists() && path.as_ref().is_file() {
        let mut buffer = match mode.as_str() {
            "w" => OpenOptions::new().read(true).write(true).open(path)?,
            "a" => OpenOptions::new().append(true).open(path)?,
            _ => {
                return Err(Error::IoError {
                    source: std::io::Error::new(std::io::ErrorKind::Other, "Invalid mode."),
                })
            }
        };
        write_file(&mut buffer, content)
    } else {
        match save_file_dialog("save file", path.as_ref().to_str().unwrap()) {
            Some(path) => {
                let mut buffer = OpenOptions::new().write(true).create_new(true).open(path)?;
                write_file(&mut buffer, content)
            }
            None => Err(Error::DialogError {
                detail: String::from("User did not save the file."),
            }),
        }
    }
}

pub fn read_dir(path: String) -> Result<HashMap<String, File>> {
    // function for reading contenrs of a directory.
    let mut folder: HashMap<String, File> = HashMap::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = Metadata::from(entry.metadata().unwrap(), false, false);
        let dir = File::new(entry.file_name().into_string().unwrap(), metadata);
        folder.insert(entry.file_name().into_string().unwrap(), dir);
    }
    Ok(folder)
}

pub fn create_dir(path: String) -> Result<String> {
    /* Function for creating a directory
    Creates directory in a given path
    */
    fs::DirBuilder::new().create(path)?;
    Ok("success".to_string())
}

pub fn create_file(path: String) -> Result<String> {
    /* Function for creating a file.
    creates a file in a given path
    */
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    Ok("success".to_string())
}

pub fn read_file(path: String) -> FileResult {
    // read contents of a file.
    let bytes = fs::read(&path).unwrap();
    let mut is_text: bool = false;
    let mut is_binary: bool = false;
    match inspect(&bytes) {
        ContentType::BINARY => is_binary = true,
        _ => is_text = true,
    }
    let metadata = Metadata::from(fs::metadata(&path).unwrap(), is_text, is_binary);
    FileResult::new(path, bytes, metadata)
}

pub fn read_text_file(path: String) -> Result<String> {
    //read a text file

    let file = fs::read_to_string(&path)?;
    Ok(file)

    // match inspect(&bytes) {
    //     ContentType::BINARY => Er::IoError { source: "here" }),
    //     _ => Ok(base64::encode(&bytes)),
    // }
}

pub fn write_file(file: &mut fs::File, content: &[u8]) -> Result<String> {
    file.write_all(content)?;
    Ok("success".to_string())
}

pub fn copy_file(from: String, to: String) -> Result<String> {
    // copy a file from a to b, where a is current path of a file
    // and b is a path where you want it to be copied.
    fs::copy(from, to)?;
    Ok("success".to_string())
}

pub fn rename_file(from: String, to: String) -> Result<String> {
    //rename or move file
    fs::rename(from, to)?; // move file
    Ok("success".to_string())
}

pub fn remove_file(path: String) -> Result<String> {
    // remove a file
    fs::remove_file(path)?;
    Ok("success".to_string())
}

pub fn remove_dir(path: String) -> Result<String> {
    // remove a directory and all its contents. USE VERY CAREFULLY
    fs::remove_dir_all(path)?;
    Ok("success".to_string())
}
