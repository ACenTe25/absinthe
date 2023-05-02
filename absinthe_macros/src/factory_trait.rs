// super
use super::*;

// crate
use crate::common::*;
use crate::items::*;

// crates.io
use syn::ItemTrait;

// Modules

// CODE

pub fn factory_trait_logic(arg_tokens: TS1, item_tokens: TS1) -> TS1 {
    
    let factory_trait_item = parse_macro_input!(
        item_tokens as ItemTrait
    );

    let fact_trait_absitem = AbsintheTrait::from_syn_itemtrait(
        &factory_trait_item
    );

    let mut output_stream = quote!(

        #factory_trait_item
    );

    if !fact_trait_absitem.has_supertrait("Send") {

        add_to_output(
            quote!(
                compile_error!("The factory Trait is not 'Send'. Add 'Send' as \
                a supertrait for the factory Trait.");
            ),
            &mut output_stream
        );
    }

    output_stream.into()
}