// super
use super::*;

// CODE

type Impls = CrateItems<AbsintheImpl>;

pub fn validate_unique_abstract_factory(
    concrete_absimpl: &AbsintheImpl
) -> Result<(), TS2> {

    
    let concrete_type = concrete_absimpl.get_ident();

    let mut impls = Impls::get();

    impls
    .filter_ident(concrete_type)
    .filter_with_attr("with_factory");

    match impls.exactly_one(
        "Should never happen: 'impl' block with the 'with_factory' \
        attribute says there are no impl blocks for this concrete type with \
        the attribute. Should at least find itself.",
        "There are many 'impl' blocks with the 'with_factory' \
        attribute for this concrete type (Struct). There should be only one."
    ) {

        Ok(_) => Ok(()),

        Err(errst) => Err(errst)
    }
}

pub fn validate_unique_concrete_factory(
    concrete_absimpl: &AbsintheImpl
) -> Result<(), TS2> {

    let concrete_type = concrete_absimpl.get_ident();

    let mut impls = Impls::get();

    impls
    .filter_attr_with_arg("factory", concrete_type);

    match impls.exactly_one(
        "There is no concrete factory for this type (Struct). An \
        'impl' block implementing the Factory Trait for a Struct should exist, \
        with the 'factory' attribute and the name of this Struct as its argument.",
        "There are many 'impl' blocks with the 'factory' \
        attribute with this type (Struct) as the argument. There should be \
        only one."
    ) {

        Ok(_) => Ok(()),

        Err(errst) => Err(errst)
    }
}