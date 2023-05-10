
// super
use super::*;

// crate
use crate::common::*;
use crate::items::*;

// crates.io
use syn::ItemImpl;

// Modules
mod find_target;
use find_target::*;

mod get_idents;
use get_idents::*;

mod register;
use register::*;

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

    let (
        registry_ident,
        factory_trait
    ) = match get_registry_and_trait(&target_trait) {

        Ok((reg, fact)) => (Some(reg), Some(fact)),

        Err(errst) => {

            add_to_output(errst, &mut output_stream);

            (None, None)
        }
    };

    let type_name = match get_concrete_type_name(&concrete_type) {

        Ok(name) => Some(name),

        Err(errst) => {

            add_to_output(errst, &mut output_stream);

            None
        }
    };

    let factory_absitem = AbsintheImpl::from_syn_itemimpl(
        &factory_item
    );

    if factory_absitem.is_none() {

        add_to_output(
            quote!(
                compile_error!(
                    "Incorrect use of attribute: must be applied to an 'impl' \
                    block implementing a Trait for a Struct."
                );
            ),
            &mut output_stream
        );
    }

    let concrete_factory_ident = match factory_absitem {

        Some(itm) => Some(itm.get_ident().clone()),

        None => {
            
            add_to_output(
                quote!(
                    compile_error!(
                        "Previous errors in this macro."
                    );
                ),
                &mut output_stream
            );

            None
        }
    };

    let registration_output = register_concrete_factory(
        registry_ident,
        type_name,
        factory_trait,
        concrete_factory_ident
    );

    add_to_output(registration_output, &mut output_stream);

    output_stream.into()
}