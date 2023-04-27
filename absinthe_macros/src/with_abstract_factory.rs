// super
use super::*;

// crate
use crate::common::*;
use crate::items::*;

// crates.io
use syn::ItemTrait;

// Modules
mod find_absfact;
use find_absfact::find_abstract_factory;

mod get_factory_trait;
use get_factory_trait::get_factory_trait;

mod create_registry;
use create_registry::create_factory_registry;

// CODE

pub fn with_abstract_factory_logic(arg_tokens: TS1, item_tokens: TS1) -> TS1 {
    
    let target_item = parse_macro_input!(item_tokens as ItemTrait);

    let target_absitem = AbsintheTrait::from_syn_itemtrait(
        &target_item
    );

    let target_ident = target_absitem.get_ident();

    let mut output_stream = quote!(

      #target_item
    );

    let registry_name = match validate_arg_str_uppercase(arg_tokens) {
        
        Ok(name) => Some(name),

        Err(errst) => {
            
            add_to_output(errst, &mut output_stream);

            None
        }
    };

    match find_abstract_factory(target_ident) {

        Ok(_) => (),

        Err(errst) => add_to_output(
            errst, 
            &mut output_stream
        )
    };

    let factory_trait_ident = match get_factory_trait(target_ident) {

        Ok(ident) => Some(ident),

        Err(errst) => {

            add_to_output(errst, &mut output_stream);

            None
        }
    };

    let registry_output = create_factory_registry(
        registry_name,
        target_ident,
        factory_trait_ident
    );

    add_to_output(registry_output, &mut output_stream);

    output_stream.into()
}
