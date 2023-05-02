// super
use super::*;

// crate
use crate::items::*;

// crates.io
use syn::{Lit, parse};

// CODE

pub fn validate_arg_str(arg_tokens: TS1) -> Result<String, TS2> {
    
    match parse::<Lit>(arg_tokens) {

        Ok(Lit::Str(arg)) => Ok(arg.value()),

        _ => Err(
            quote!(
                compile_error!("Incorrect attribute argument: must be a \
                literal string.");
            )
        )
    }
}

pub fn validate_arg_ident(arg_tokens: TS1) -> Result<Ident, TS2> {

    match parse::<Ident>(arg_tokens) {

        Ok(ident_ref) => Ok(ident_ref.clone()),

        _ => Err(
            quote!(
                compile_error!("Incorrect attribute argument: must be the name \
                of a Trait (no quotes).");
            )
        )
    }
}

pub fn get_absfact_trait_and_registry(
    ident: &Ident
) -> Result<(Ident, Ident), TS2> {

    type Traits = CrateItems<AbsintheTrait>;

    let mut traits = Traits::get();

    let absfact_trait = traits
    .filter_ident(ident)
    .filter_with_attr("with_abstract_factory")
    .exactly_one(
        "Invalid abstract factory Trait: the trait was not found \
        in this crate or it does not have the 'with_abstract_factory' attribute.",
        "Invalid abstract factory Trait: there are many trait \
        declarations in the crate with the same Ident which also have the \
        'with_abstract_factory' attribute."
    )?;

    let af_ident = absfact_trait.get_ident().clone();

    let Some(af_registry_name) = absfact_trait.get_attr_arg(
        "with_abstract_factory"
    ) else {
        return Err(
            quote!(
                compile_error!("Invalid argument for 'with_abstract_factory' \
                attribute of the target Trait. Must be an uppercase literal \
                string.");
            )
        )
    };

    let af_registry_ident = Ident::new(
        &af_registry_name,
        af_ident.span()
    );

    Ok((af_ident, af_registry_ident))
}

pub fn validate_absfact_ident_arg_and_get_trait_and_registry(
    arg_tokens: TS1
) -> Result<(Ident, Ident), TS2> {
    
    let arg_ident = validate_arg_ident(arg_tokens)?;

    get_absfact_trait_and_registry(&arg_ident)
}

pub fn validate_option_input<T: Clone>(input: Option<T>) -> Result<T, TS2> {

    match input {
        
        Some(val) => Ok(val.clone()),

        None => Err(
            quote!(
                compile_error!("Could not continue applying attribute due to \
                previous errors in this macro.");
            )
        )
    }
}

pub fn add_to_output(
    additional_output: TS2, 
    output_stream: &mut TS2
) {

    *output_stream = quote!(

        #output_stream

        #additional_output
    );
}