// super
use super::*;

// CODE

pub fn create_abstract_factory(
    target_ident: &Ident,
    registry_name: &Option<String>,
    absfact_struct_ident: &Option<Ident>,
    config_registry_key: &Option<String>,
) -> TS2 {

    let error_output = quote!(
        
        compile_error!("Could not create abstract factory due to previous \
        errors in this macro.");
    );

    let Some(registry) = registry_name else {
        return error_output
    };

    let Some(absfact) = absfact_struct_ident else {
        return error_output
    };

    let Some(config_key) = config_registry_key else {
        return error_output
    };

    let registry_ident = Ident::new(
        registry.as_str(), 
        target_ident.span()
    );

    quote!(

        pub struct #absfact;

        impl #absfact {

            pub fn new(
                config: Box<dyn AbsintheConf>
            ) -> anyhow::Result<Box<dyn #target_ident>> {

                use anyhow::anyhow;

                let concrete_type = config.get(#config_key)?;

                let factory_registry = #registry_ident
                .lock()
                .expect("Poisoned registry.");

                let concrete_factory = factory_registry
                .get(&concrete_type)
                .ok_or_else(
                    || anyhow!(
                        "Error creating new object: unregistered concrete \
                        factory ({}).", 
                        concrete_type
                    )
                )?;

                concrete_factory.new(config)
            }
        }
    )
}