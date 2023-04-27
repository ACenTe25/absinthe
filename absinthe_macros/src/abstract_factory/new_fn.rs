// super
use super::*;

// CODE

pub fn create_new_fn(
    abstract_factory_ident: &Ident,
    target_trait_ident: Option<Ident>,
    registry_ident: Option<Ident>,
    key: Option<String>
) -> TS2 {
    
    let err_output = quote!(
        
        compile_error!("Could not continue applying attribute due to previous \
        errors in this macro.");
    );

    let Some(target_trait) = target_trait_ident else {
        return err_output
    };

    let Some(registry) = registry_ident else {
        return err_output
    };

    let Some(registry_key) = key else {
        return err_output
    };

    quote!(

        impl #abstract_factory_ident {

            pub fn new(
                config: Box<dyn AbsintheConfig>
            ) -> anyhow::Result<Box<dyn #target_trait>> {

                use anyhow::anyhow;

                let concrete_type = config.get(#registry_key.as_str())?;

                let factory_registry = #registry.lock().expect("Poisoned registry.");

                let concrete_factory = factory_registry
                .get(&concrete_type)
                .ok_or_else(
                    || anyhow!(
                        "Error creating new object: unregistered concrete factory ({}).",
                        concrete_type
                    )
                )?;

                concrete_factory.new(config)
            }
        }
    )
}