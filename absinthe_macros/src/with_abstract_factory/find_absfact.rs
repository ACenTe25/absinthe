// super
use super::*;

// CODE 

pub fn find_abstract_factory(target_ident: &Ident) -> Result<(), TS2> {
    
    type Structs = CrateItems<AbsintheStruct>;

    let mut structs = Structs::get();

    structs
    .filter_attr_with_arg(
        "abstract_factory", 
        target_ident
    );

    match structs.exactly_one(
        "Abstract Factory Struct not found in crate: a struct \
        declaration with the 'abstract_factory' attribute is needed, with this \
        Trait as its argument.", 
        "There are many Abstract Factory Structs for this Trait \
        in the crate. Only one is allowed."
    ) {

        Ok(_) => Ok(()),

        Err(errst) => Err(errst)
    }
}