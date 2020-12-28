// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
// use std::fs;
// use std::fs::Metadata as MD;
// use std::io;
// use std::io::prelude::*;

// type Folder = HashMap<String, Dir>;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Dir {
//     pub name: String,
//     pub metadata: Metadata,
// }

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Metadata {
//     pub is_dir: bool,
//     pub is_file: bool,
//     pub is_text: bool,
//     pub is_binary: bool,
//     pub size: u64,
// }

// impl Metadata {
//     fn from(metadata: MD, is_text: bool, is_binary: bool) -> Self {
//         Metadata {
//             size: metadata.len(),
//             is_dir: metadata.is_dir(),
//             is_file: metadata.is_file(),
//             is_text: is_text,
//             is_binary: is_binary,
//         }
//     }
// }

// impl Dir {
//     fn new(name: String, metadata: Metadata) -> Self {
//         Dir { name, metadata }
//     }
// }

// pub fn lis_dir(path: String) -> io::Result<Folder> {
//     // Function for getting contents of a given dir

//     let mut folder: Folder = HashMap::new();

//     for entry in fs::read_dir(path)? {
//         let entry = entry?;
//         let metadata = Metadata::from(entry.metadata().unwrap(), false, false);
//         let dir = Dir::new(entry.file_name().into_string().unwrap(), metadata);
//         folder.insert(entry.file_name().into_string().unwrap(), dir);
//     }
//     Ok(folder)
// }

// pub fn copy_file(from: String, to: String) -> io::Result<()> {
//     fs::copy(from, to)?;
//     Ok(())
// }

// pub fn move_file(from: String, to: String) -> std::io::Result<()> {
//     fs::rename(from, to)?; // move file
//     Ok(())
// }

// pub mod file_picker {
//     use super::Metadata;
//     use content_inspector::{inspect, ContentType};
//     use native_dialog::{FileDialog, MessageDialog, MessageType};
//     use serde::{Deserialize, Serialize};
//     use std::ffi::OsString;
//     use std::fs;
//     use std::fs::read;

//     #[derive(Serialize, Deserialize, Debug)]
//     #[serde(rename_all = "camelCase")]
//     pub struct FileResult {
//         pub path: String,
//         pub bytes: Vec<u8>,
//         pub metadata: Metadata,
//     }

//     impl FileResult {
//         pub fn new(path: String, bytes: Vec<u8>, metadata: Metadata) -> Self {
//             Self {
//                 path,
//                 bytes,
//                 metadata,
//             }
//         }
//     }

//     pub fn open_file(path: &String, filters: Option<Vec<String>>) -> FileResult {
//         // use crate::std_api::fs::crud::Crud;
//         let file_path = || {
//             if let Some(filters) = filters {
//                 let converted_filter: Vec<&str> = filters.iter().map(AsRef::as_ref).collect();
//                 let result = FileDialog::new()
//                     .set_location(&path)
//                     .add_filter("", converted_filter.as_slice())
//                     .show_open_single_file()
//                     .unwrap();
//                 return result.unwrap();
//             } else {
//                 let result = FileDialog::new()
//                     .set_location(&path)
//                     .show_open_single_file()
//                     .unwrap();
//                 return result.unwrap();
//             }
//         };
//         let path = String::from(file_path().into_os_string().to_str().unwrap());
//         // let file = Crud::open_file(, "rw".to_string()).unwrap();
//         let bytes = read(path.clone()).unwrap();
//         let mut is_text: bool = false;
//         let mut is_binary: bool = false;
//         match inspect(&bytes) {
//             ContentType::BINARY => is_binary = true,
//             _ => is_text = true,
//         }
//         let metadata = Metadata::from(fs::metadata(&path).unwrap(), is_text, is_binary);
//         println!("finished opening file");
//         FileResult::new(path, bytes, metadata)
//     }

//     pub fn open_multiple_file(path: String, filters: Option<Vec<String>>) {
//         let file_path = || {
//             if let Some(filters) = filters {
//                 let converted_filter: Vec<&str> = filters.iter().map(AsRef::as_ref).collect();
//                 let result = FileDialog::new()
//                     .set_location(&path)
//                     .add_filter("", converted_filter.as_slice())
//                     .show_open_multiple_file()
//                     .unwrap();
//                 return result;
//             } else {
//                 let result = FileDialog::new()
//                     .set_location(&path)
//                     .show_open_multiple_file()
//                     .unwrap();
//                 return result;
//             }
//         };
//     }

//     pub fn open_dir(path: String) {
//         let result = FileDialog::new()
//             .set_location(&path)
//             .show_open_single_dir()
//             .unwrap();
//         println!("{:?}", result);
//     }
// }
