// super
use super::*;

// crates.io
use syn::{Lit, meta::parser};

// CODE

pub fn parse_with_absfact_args(
    arg_tokens: TS1,
    registry_name: &mut Option<String>,
    absfact_struct: &mut Option<Ident>,
    config_registry_key: &mut Option<String>,
    factory_trait: &mut Option<Ident>
) -> TS1 {

    let parser = parser(
        |meta| {

            if meta.path.is_ident("registry") {

                *registry_name = match meta.value()?.parse::<Lit>()? {

                    Lit::Str(txt) => Some(txt.value()),

                    _ => return Err(meta.error("argument 'registry' must \
                    be a literal string."))
                };

            } else if meta.path.is_ident("abstract_factory") {

                *absfact_struct = match meta.value()?.parse::<Ident>() {

                    Ok(ident) => Some(ident.clone()),

                    Err(_) => return Err(meta.error("argument 'abstract_\
                    factory' must be a name for a Struct (unquoted)."))
                };

            } else if meta.path.is_ident("factory_trait") {

                *factory_trait = match meta.value()?.parse::<Ident>() {

                    Ok(ident) => Some(ident.clone()),

                    Err(_) => return Err(meta.error("argument 'factory_\
                    trait' must be a name for a Trait (unquoted)."))
                };

            } else if meta.path.is_ident("config_registry_key") {

                *config_registry_key = match meta.value()?.parse::<Lit>()? {

                    Lit::Str(txt) => Some(txt.value()),

                    _ => return Err(meta.error("argument 'factory_registry\
                    _key' must be a literal string."))
                };
            }

            Ok(())
        }
    );

    parse_macro_input!(arg_tokens with parser);

    quote!().into()
}