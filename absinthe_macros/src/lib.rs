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

mod with_abstract_factory;
use with_abstract_factory::with_abstract_factory_logic;

mod abstract_factory;
use abstract_factory::abstract_factory_logic;

mod factory_trait;
use factory_trait::factory_trait_logic;

mod with_factory;
use with_factory::with_factory_logic;

mod factory;
use factory::factory_logic;

// CODE
#[proc_macro_attribute]
pub fn with_abstract_factory(arg_tokens: TS1, item_tokens: TS1) -> TS1 {
    with_abstract_factory_logic(arg_tokens, item_tokens)
}

#[proc_macro_attribute]
pub fn abstract_factory(arg_tokens: TS1, item_tokens: TS1) -> TS1 {
    abstract_factory_logic(arg_tokens, item_tokens)
}

#[proc_macro_attribute]
pub fn factory_trait(arg_tokens: TS1, item_tokens: TS1) -> TS1 {
    factory_trait_logic(arg_tokens, item_tokens)
}

#[proc_macro_attribute]
pub fn with_factory(arg_tokens: TS1, item_tokens: TS1) -> TS1 {
    with_factory_logic(arg_tokens, item_tokens)
}

#[proc_macro_attribute]
pub fn factory(arg_tokens: TS1, item_tokens: TS1) -> TS1 {
    factory_logic(arg_tokens, item_tokens)
}