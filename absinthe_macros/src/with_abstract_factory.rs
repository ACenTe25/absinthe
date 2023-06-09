// super
use super::*;

// crate
use crate::common::*;
use crate::items::*;

// crates.io
use syn::ItemTrait;

// Modules
mod parse_args;
use parse_args::parse_with_absfact_args;

mod check_impls;
use check_impls::check_target_trait_impls;

mod create_absfact;
use create_absfact::create_abstract_factory;

mod create_trait;
use create_trait::create_factory_trait;

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

    let mut registry_name: Option<String> = None;

    let mut absfact_struct_ident: Option<Ident> = None;

    let mut config_registry_key: Option<String> = None;

    let mut factory_trait_ident: Option<Ident> = None;

    _ = parse_with_absfact_args(
        arg_tokens, 
        &mut registry_name,
        &mut absfact_struct_ident,
        &mut config_registry_key,
        &mut factory_trait_ident
    );

    if registry_name.is_none() 
    || absfact_struct_ident.is_none() 
    || config_registry_key.is_none()
    || factory_trait_ident.is_none() {

        add_to_output(
            quote!(
                compile_error!("Incorrect or incomplete arguments. You must \
                specify 'registry', 'abstract_factory', 'config_registry_key', \
                and 'factory_trait'. See documentation for more details.");
            ),
            &mut output_stream
        );
    }

    match &registry_name {

        None => (),

        Some(name) => if name.to_uppercase().as_str() != name.as_str() {

            add_to_output(
                quote!(
                    compile_error!("Incorrect 'registry' argument: must be \
                    uppercase.");
                ),
                &mut output_stream
            );
        }
    };
    
    let registry_output = create_factory_registry(
        &registry_name,
        &target_ident,
        &factory_trait_ident
    );

    add_to_output(registry_output, &mut output_stream);

    let absfact_output = create_abstract_factory(
        &target_ident,
        &registry_name,
        &absfact_struct_ident,
        &config_registry_key
    );

    add_to_output(absfact_output, &mut output_stream);

    let factory_trait_output = create_factory_trait(
        &target_ident,
        &factory_trait_ident
    );

    add_to_output(factory_trait_output, &mut output_stream);

    match check_target_trait_impls(&target_ident) {

        Ok(_) => (),

        Err(errst) => add_to_output(
            errst, 
            &mut output_stream
        ),
    };

    output_stream.into()
}
