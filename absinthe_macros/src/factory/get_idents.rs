// super
use super::*;

// CODE

pub fn get_registry_and_trait(target_ident: &Ident) -> Result<(Ident, Ident), TS2> {
    
    type Traits = CrateItems<AbsintheTrait>;

    let mut traits = Traits::get();

    traits
    .filter_ident(target_ident)
    .filter_with_attr("with_abstract_factory");
    
    // Checar que sea uno, extraerlo, y luego usar el get_attr_named_arg bla bla 
    let target_trait_item = traits
    .exactly_one(
        "Could not find the Target Trait corresponding to this \
        concrete factory. There must be one.", 
        "Found many Traits with the name of the Target Trait for \
        this concrete factory, declared with the 'with_abstract_factory' \
        attribute. There must be only one."
    )?;

    let Some(registry_name) = target_trait_item
    .get_attr_pair_arg_str(
        "with_abstract_factory", 
        "registry"
    ) else {

        return Err(quote!(compile_error!(
            "Error in 'with_abstract_factory' argument 'registry'."
        );))
    };

    let Some(factory_trait_ident) = target_trait_item
    .get_attr_pair_arg_ident(
        "with_abstract_factory",
        "factory_trait"
    ) else {

        return Err(quote!(compile_error!(
            "Error in 'with_abstract_factory' argument 'factory_trait'."
        );))
    };

    let registry_ident = Ident::new(
        &registry_name, 
        factory_trait_ident.span()
    );

    Ok((registry_ident, factory_trait_ident))
}

pub fn get_concrete_type_name(concrete_type: &Ident) -> Result<String, TS2> {
    
    type Impls = CrateItems<AbsintheImpl>;

    let mut impls = Impls::get();

    impls
    .filter_ident(concrete_type)
    .filter_with_attr("with_factory");

    let concrete_type_item = impls
    .exactly_one(
        "Could not find an 'impl' block for this concrete type \
        with the 'with_factory' attribute. There must be one.", 
        "Found many 'impl' blocks for this concrete type with the \
        'with_factory' attribute. There must be only one."
    )?;

    let Some(name) = concrete_type_item
    .get_attr_single_arg("with_factory") else {

        return Err(quote!(compile_error!(
            "Error in 'with_factory' argument."
        );))
    };

    Ok(name)
}