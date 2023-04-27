// super
use super::*;

// CODE

pub fn create_factory_registry(
    registry_name: Option<String>,
    target_ident: &Ident,
    factory_trait_ident: Option<Ident>
) -> TS2 {

    let err_output = quote!(
        compile_error!("Could not continue applying attribute due to previous \
        errors in this macro.");
    );

    let Ok(reg_name) = validate_option_input(registry_name) else {
        return err_output
    };

    let Ok(fact_trait_ident) = validate_option_input(factory_trait_ident) else {
        return err_output
    };

    let registry_ident = Ident::new(
        &reg_name, 
        target_ident.span()
    );

    quote!(
        use std::collections::HashMap;
        use std::sync::Mutex;
        use lazy_static::lazy_static;

        lazy_static! {

            static ref #registry_ident:
            Mutex<HashMap<String, Box<dyn #fact_trait_ident>>> =
            Mutex::new(HashMap::new());
        }
    )
}