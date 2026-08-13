pub mod dir_reader {
    use std::{
        fs::{DirEntry, read_dir},
        io::{self, Error},
        path::Path,
    };

    pub fn dir_reader() -> Result<Vec<String>, Error> {
        let path = Path::new(".");
        let mut entries: Vec<String> = read_dir(path)?
            .map(|res| res.map(|e: DirEntry| String::from(e.path().to_str().unwrap())))
            .collect::<Result<Vec<_>, Error>>()?;

        entries.sort();

        Ok(entries)
    }

    pub fn list_files() -> io::Result<String> {
        let files = dir_reader()?;
        let files_to_html: Vec<String> = files
            .iter()
            .map(|file| format!("<p>{}</p>", file))
            .collect();

        let file_list = files_to_html.as_slice().join("\n");

        Ok(file_list)
    }
}
