// super
use super::*;

// CODE
pub fn get_factory_trait(target_ident: &Ident) -> Result<Ident, TS2> {
    
    type Traits = CrateItems<AbsintheTrait>;

    let mut traits = Traits::get();

    traits
    .filter_attr_with_arg(
        "trait_factory", 
        target_ident
    );

    match traits.exactly_one(
        "Could not find a Trait declaration with the 'factory_tr\
        ait' attribute for this Trait. There must be one.", 
        "There are many Trait declarations with the 'factory_tra\
        it' for this Trait. There must be only one."
    ) {

        Ok(factrait) => Ok(factrait.get_ident().clone()),

        Err(sterr) => Err(sterr)
    }
}