// super
use super::*;

// crate
use crate::common::*;

// crates.io
use syn::ItemImpl;

// Modules
mod find_target;
use find_target::*;

// CODE

pub fn factory_logic(arg_tokens: TS1, item_tokens: TS1) -> TS1 {

    let factory_item = parse_macro_input!(item_tokens as ItemImpl);

    let mut output_stream = quote!(

        #factory_item
    );

    let concrete_type = match validate_arg_ident(arg_tokens) {

        Ok(ident) => ident,

        Err(errst) => {

            add_to_output(errst, &mut output_stream);

            return output_stream.into()
        }
    };
    
    let target_trait = match find_target_trait(&concrete_type) {

        Ok(ident) => ident,

        Err(errst) => {

            add_to_output(errst, &mut output_stream);

            return output_stream.into()
        }
    };

    // get the registry ident

    // get the factory trait ident and the concrete_factory_ident (this self_type)

    // register the factory in the registry

    output_stream.into()
}