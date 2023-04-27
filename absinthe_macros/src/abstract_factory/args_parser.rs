// super
use super::*;

// crates.io
use syn::{Lit, meta::parser};

// CODE

pub fn parse_abstract_factory_args(
    arg_tokens: TS1,
    key: &mut Option<String>,
    target_trait_ident: &mut Option<Ident>
) -> TS1 {
    
    let mut key: Option<String> = None;

    let mut target_trait_ident: Option<Ident> = None;

    // magia
    let abstract_factory_parser = parser(
        |meta| {
            if meta.path.is_ident("key") {

                key = match meta.value()?.parse::<Lit>()? {

                    Lit::Str(txt) => Some(txt.value()),

                    _ => return Err(meta.error("argument key must be liter\
                    al string."))
                };

                Ok(())

            } else {

                target_trait_ident = match meta.path.get_ident() {

                    Some(id) => Some(id.clone()),

                    None => None
                };

                Ok(())
            }
        }
    );

    parse_macro_input!(arg_tokens with abstract_factory_parser);

    quote!().into()
}