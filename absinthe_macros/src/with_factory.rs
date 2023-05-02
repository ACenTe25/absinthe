// super
use super::*;

// crate
use crate::common::*;
use crate::items::*;

// crates.io
use syn::ItemImpl;

// Modules
mod target;
use target::*;

// CODE

pub fn with_factory_logic(arg_tokens: TS1, item_tokens: TS1) -> TS1 {

    let concrete_item = parse_macro_input!(item_tokens as ItemImpl);

    let mut output_stream = quote!(

        #concrete_item
    );

    let concrete_name = match validate_arg_str(arg_tokens) {

        Ok(txt) => Some(txt),

        Err(errst) => {

            add_to_output(errst, &mut output_stream);

            None
        }
    };

    let Some(concrete_absimpl) = AbsintheImpl::from_syn_itemimpl(
        &concrete_item
    ) else {

        add_to_output(
            quote!(
                compile_error!("Attribute should be applied to an 'impl' block \
                which implements a Trait for a Struct.");
            ),
            &mut output_stream
        );
        return output_stream.into()
    };

    match validate_target_trait(&concrete_absimpl) {

        Ok(_) => (),

        Err(errst) => add_to_output(
            errst, 
            &mut output_stream
        )
    };

    // Validar que este struct solo tenga este impl con atributo "with_factory"

    // Validar que solo exista un bloque impl con el atributo "factory" y argumento
    // igual a este self_type.

    output_stream.into()
}