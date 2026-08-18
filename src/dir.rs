pub mod dir_reader {
    use std::{
        fs::{DirEntry, read_dir},
        io::{self, Error},
        path::Path,
    };

    pub fn dir_reader(path: &Path) -> Result<Vec<String>, Error> {
        let mut entries: Vec<String> = read_dir(path)?
            .map(|res| res.map(|e: DirEntry| String::from(e.path().to_str().unwrap())))
            .collect::<Result<Vec<_>, Error>>()?;

        entries.sort();

        Ok(entries)
    }

    pub fn list_files(path: &Path) -> io::Result<String> {
        let files = dir_reader(path)?;
        let files_to_html: Vec<String> = files
            .iter()
            .map(|file| format!("<p>{}</p>", file))
            .collect();

        let file_list = files_to_html.as_slice().join("\n");

        Ok(file_list)
    }
}

// TODO: Use structs and enums to simplify process.
// ex: struct for --http requests--(DONE), list of paths, creating listener
