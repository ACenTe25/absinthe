// super
use super::*;

// CODE

pub fn create_factory_trait(
    target_ident: &Ident,
    factory_trait_ident: &Option<Ident>
) -> TS2 {

    let error_output = quote!(
        
        compile_error!("Could not create abstract factory due to previous \
        errors in this macro.");
    );

    let Some(factory_trait) = factory_trait_ident else {
        return error_output
    };

    quote!(

        pub trait #factory_trait {

            fn new(
                config: Box<dyn AbsintheConfig>
            ) -> anyhow::Result<Box<dyn #target_ident>>;
        }
    )
}