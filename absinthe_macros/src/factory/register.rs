// super
use super::*;

// crates.io
use proc_macro2::Span;

// CODE

pub fn register_concrete_factory(
    registry_ident: Option<Ident>,
    concrete_factory_name: Option<String>,
    factory_trait: Option<Ident>,
    concrete_factory: Option<Ident>
) -> TS2 {

    let err_prev = quote!(
        compile_error!(
            "Could not register concrete factory due to previous errors in \
            this macro."
        );
    );

    // checar todo a ver si es none para salir con error
    let Some(registry) = registry_ident else {
        return err_prev
    };

    let Some(factory_name) = concrete_factory_name else {
        return err_prev
    };

    let Some(factory) = factory_trait else {
        return err_prev
    };

    let Some(factory_ident) = concrete_factory else {
        return err_prev
    };

    // Ya que esté desencapsulado, hacer cálculos de nombres de funciones
    // etc..
    let registry_fn_name = format!(
        "REGISTER_{}",
        factory_name.to_uppercase()
    );

    let registry_fn_ident = Ident::new(
        &registry_fn_name,
        Span::call_site().into()
    );

    let factory_value = quote!(
        Box::new(#factory_ident) as Box<dyn #factory>
    );

    quote!(

        #[allow(non_uppercase_globals)]
        #[doc(hidden)]
        #[used]
        #[link_section = ".init_array"]
        pub static #registry_fn_ident: fn() = {
            fn __add_to_registry() {
                let mut registry = #registry
                .lock()
                .expect("Poisoned registry");

                registry.insert(#factory_name.to_string(), #factory_value);
            }

            __add_to_registry
        };
    )
}