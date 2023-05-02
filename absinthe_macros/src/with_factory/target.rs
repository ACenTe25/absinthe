// super
use super::*;

// CODE

pub fn validate_target_trait(concrete_absimpl: &AbsintheImpl) -> Result<(), TS2> {

    let Some(target_trait_ident) = concrete_absimpl.get_trait() else {
        return Err(quote!(
            compile_error!("Should not happen. AbsintheImpl without a trait.");
        ))
    };    

    match get_absfact_trait_and_registry(target_trait_ident) {

        Ok(_) => Ok(()),

        Err(errst) => Err(errst)
    }
}