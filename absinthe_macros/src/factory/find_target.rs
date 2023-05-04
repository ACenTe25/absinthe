// super
use super::*;

// crate
use crate::items::*;

// CODE
pub fn find_target_trait(concrete_type: &Ident) -> Result<Ident, TS2> {

    type Impls = CrateItems<AbsintheImpl>;

    let mut impls = Impls::get();

    impls
    .filter_ident(concrete_type)
    .filter_with_attr("with_factory");

    let concrete_absimpl = impls
    .exactly_one(
        "Invalid argument: there are no 'impl' blocks for that \
        type with the 'with_factory' attribute in this crate.",
        "There are many 'impl' blocks for that type with the \
        'with_factory' attribute in this crate. There should be only one."
    )?;

    let Some(target_trait_ident) = concrete_absimpl.get_trait() else {
        return Err(
            quote!(
                compile_error!("Should never happen: AbsintheImpl which is not \
                implementing a Trait.");
            )
        )
    };

    Ok(target_trait_ident.clone())
}

pub fn get_registry_ident(target_trait: &Ident) -> Result<Ident, TS2> {

    type Traits = CrateItems<AbsintheTrait>;

    let mut traits = Traits::get();

    traits
    .filter_ident(target_trait)
    .filter_with_attr("with_abstract_factory");

    unimplemented!()
}