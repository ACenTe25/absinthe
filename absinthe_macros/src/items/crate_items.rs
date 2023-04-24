// super
use super::*;

// crate
use crate::crate_files::*;

// Rust Standard distribution
use std::fs::read_to_string;
use std::iter::empty;
use std::ops::{Deref, DerefMut};

// crates.io
use syn::parse_file;

// CODE

pub struct CrateItems<T>
where T: FromSynItem + 'static {
    pub items: Vec<T>
}

impl<T> Deref for CrateItems<T>
where T: FromSynItem + 'static {

    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<T> DerefMut for CrateItems<T>
where T: FromSynItem + 'static {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl<T> CrateItems<T>
where T: FromSynItem + 'static {

    pub fn get() -> Self {

        let files = CrateRustFiles::new();

        let items: Vec<T> = files
        .files_iter
        .flat_map(Self::search)
        .collect();
        
        Self { items }
    }

    fn search(file: RustFile) -> Box<dyn Iterator<Item = T>> {
        
        match read_to_string(file.0.path()) {

            Ok(code) => match parse_file(&code) {

                Ok(parsed_code) => {

                    Box::new(
                        parsed_code
                        .items
                        .iter()
                        .filter_map(
                            |item| T::from_syn_item(item)
                        )
                        .collect::<Vec<T>>()
                        .into_iter()
                    )
                }

                _ => Box::new(empty())
            }

            _ => Box::new(empty())
        }
    }
}
