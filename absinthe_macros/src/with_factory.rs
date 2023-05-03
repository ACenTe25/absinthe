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

mod validate;
use validate::*;

// CODE

pub fn with_factory_logic(arg_tokens: TS1, item_tokens: TS1) -> TS1 {

    let concrete_item = parse_macro_input!(item_tokens as ItemImpl);

    let mut output_stream = quote!(

        #concrete_item
    );

    match validate_arg_str(arg_tokens) {

        Ok(_) => (),

        Err(errst) => add_to_output(
            errst, 
            &mut output_stream
        )
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
 
    match validate_unique_abstract_factory(&concrete_absimpl) {

        Ok(_) => (),

        Err(errst) => add_to_output(
            errst,
            &mut output_stream
        )
    };

    match validate_unique_concrete_factory(&concrete_absimpl) {

        Ok(_) => (),

        Err(errst) => add_to_output(
            errst,
            &mut output_stream
        )
    };

    output_stream.into()
}