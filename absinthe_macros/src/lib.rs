// Rust Standard distribution
use proc_macro::TokenStream as TS1;

// crates.io
use proc_macro2::TokenStream as TS2;
use quote::quote;
use syn::{Ident, parse_macro_input};

// Modules
mod common;
mod crate_files;
mod items;

// CODE
#[proc_macro_attribute]
pub fn with_abstract_factory(arg_tokens: TS1, item_tokens: TS1) -> TS1 {
    unimplemented!()
}

#[proc_macro_attribute]
pub fn abstract_factory(arg_tokens: TS1, item_tokens: TS1) -> TS1 {
    unimplemented!()
}

#[proc_macro_attribute]
pub fn factory_trait(arg_tokens: TS1, item_tokens: TS1) -> TS1 {
    unimplemented!()
}

#[proc_macro_attribute]
pub fn with_factory(arg_tokens: TS1, item_tokens: TS1) -> TS1 {
    unimplemented!()
}

#[proc_macro_attribute]
pub fn factory(arg_tokens: TS1, item_tokens: TS1) -> TS1 {
    unimplemented!()
}