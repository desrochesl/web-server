use std::{
    fs::{DirEntry, read_dir},
    io::Error,
    path::{Path, PathBuf},
};

pub fn dir_reader() -> Result<Vec<PathBuf>, Error> {
    let path = Path::new(".");
    let mut entries: Vec<PathBuf> = read_dir(path)?
        .map(|res| res.map(|e: DirEntry| e.path()))
        .collect::<Result<Vec<_>, Error>>()?;

    entries.sort();

    Ok(entries)
}
