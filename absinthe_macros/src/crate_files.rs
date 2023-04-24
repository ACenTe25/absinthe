// Rust Standard distribution
use std::env::var;
use std::fs::{DirEntry, ReadDir, read_dir};
use std::iter::{empty, FlatMap};
use std::path::Path;

// CODE

pub struct RustFile(pub DirEntry);

impl RustFile {

    pub fn from_direntry(direntry: DirEntry) -> Option<Self> {

        if direntry.path().is_file() && 
        direntry.path().to_string_lossy().ends_with(".rs") {

            Some(Self(direntry))

        } else {

            None
        }
    }
}

type CrateFile = Box<dyn Iterator<Item = RustFile>>;

fn get_rust_files(res: Result<DirEntry, std::io::Error>) -> CrateFile {

    match res {

        Ok(direntry) => {

            let path = direntry.path();

            if path.is_dir() {

                let contents = read_dir(path).unwrap();

                Box::new(contents.flat_map(get_rust_files))

            } else {

                match RustFile::from_direntry(direntry) {

                    Some(file) => Box::new(vec![file].into_iter()),

                    None => Box::new(empty())
                }
            }
        }

        _ => Box::new(empty())
    }
}

type GetFn = fn(
    Result<DirEntry, std::io::Error>
) -> Box<dyn Iterator<Item = RustFile> + 'static>;

pub struct CrateRustFiles {
    pub files_iter: FlatMap<ReadDir, CrateFile, GetFn> 
}

impl CrateRustFiles {

    pub fn new() -> Self {

        let crate_dir = var("CARGO_MANIFEST_DIR").unwrap();

        let crate_path = Path::new(&crate_dir);

        Self {
            files_iter: read_dir(crate_path)
            .unwrap()
            .flat_map(get_rust_files)
        }
    }
}