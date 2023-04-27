// super
use super::*;

// CODE

pub fn check_target_trait_impls(target_ident: &Ident) -> Result<(), TS2> {
    
    type Impls = CrateItems<AbsintheImpl>;

    let mut impls = Impls::get();

    impls.filter_impls_trait(target_ident);

    let impls_num = impls.count();

    impls.filter_with_attr("with_factory");

    if impls.count() == impls_num {

        Ok(())

    } else {

        Err(
            quote!(
                compile_error!("Not every 'impl' block for the target Trait has \
                the 'with_factory' attribute. This is required.");
            )
        )
    }
}