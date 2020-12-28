pub mod Crud {
    use std::ffi::OsString;
    use std::fs;
    use std::io::{self, Error, ErrorKind, Write};

    pub fn create_folder(folder_name: String, path: Option<String>) -> io::Result<()> {
        /* Function for creating a directory
        Creates a folder in current working
        directory if path is none, else creates folder in given path.
        */

        match path {
            Some(path) => fs::DirBuilder::new().create(path + &folder_name)?,
            None => fs::DirBuilder::new().create(folder_name)?,
        }
        Ok(())
    }

    pub fn create_file(file_name: String, path: Option<String>) -> io::Result<fs::File> {
        /* Function for creating a file.
        Creates a folder in current working
        directory if path is none, else creates folder in given path.
        */

        match path {
            Some(path) => fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path + &file_name),
            None => fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(file_name),
        }
    }

    pub fn open_file(path: OsString, mode: String) -> Result<fs::File, Error> {
        /* Function for opening a file
         */
        use fs::OpenOptions;

        match mode.as_str() {
            "r" => OpenOptions::new().read(true).open(path),
            "w" => OpenOptions::new().write(true).open(path),
            "rw" => OpenOptions::new().read(true).write(true).open(path),
            "a" => OpenOptions::new().append(true).open(path),
            _ => Err(Error::new(ErrorKind::Other, "Invalid mode.")),
        }
    }

    pub fn write(file: &mut fs::File, data: Vec<u8>) -> io::Result<()> {
        file.write_all(&data)?;
        Ok(())
    }
}
