// super
use super::*;

// crate
use crate::items::*; 
use crate::common::*;


// crates.io
use syn::ItemStruct;

// Modules
mod new_fn;
use new_fn::create_new_fn;

mod check_impls;
use check_impls::check_target_trait_impls;

// CODE

pub fn abstract_factory_logic(arg_tokens: TS1, item_tokens: TS1) -> TS1 {

    let abstract_factory_item = parse_macro_input!(
        item_tokens as ItemStruct
    );

    let absinthe_factory_item = AbsintheStruct::from_syn_itemstruct(
        &abstract_factory_item
    );

    let abstract_factory_ident = absinthe_factory_item.get_ident();

    let mut output_stream = quote!(

        #abstract_factory_item
    );

    let mut key: Option<String> = None;

    let mut target_trait_ident: Option<Ident> = None;

    _ = parse_named_str_and_ident(
        arg_tokens, 
        "key",
        &mut key, 
        &mut target_trait_ident
    );

    let target_ident = match validate_option_input(target_trait_ident) {

        Ok(id) => id,

        Err(errst) => {
            add_to_output(
                errst, 
                &mut output_stream
            );

            return output_stream.into()
        }
    };

    let (
        target_trait_ident,
        registry_ident
    ) = match get_absfact_trait_and_registry(
        &target_ident
    ) {

        Ok((a, b)) => (Some(a), Some(b)),

        Err(errst) => {

            add_to_output(errst, &mut output_stream);

            (None, None)
        }
    };

    let new_fn_output = create_new_fn(
        abstract_factory_ident,
        target_trait_ident,
        registry_ident,
        key
    );

    add_to_output(new_fn_output, &mut output_stream);

    // falta validar los impls del trait para ver que tengan attr 'with_factory'
    match check_target_trait_impls(&target_ident) {

        Ok(_) => (),

        Err(errst) => add_to_output(
            errst, 
            &mut output_stream
        )
    };

    output_stream.into()
}