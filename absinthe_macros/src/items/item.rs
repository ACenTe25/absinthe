// super
use super::*;

// crates.io
use syn::Lit;

// CODE

pub trait AbsintheItem: Sized + Clone {

    fn get_ident(&self) -> &Ident;

    fn get_attrs(&self) -> &[Attribute];

    fn get_supertraits(&self) -> Option<&[Ident]>;

    fn get_trait(&self) -> Option<&Ident>;

    fn get_attr_arg<T: ?Sized>(&self, attribute: &T) -> Option<String>
    where
    Ident: PartialEq<T> {

        match self
        .get_attrs()
        .iter()
        .find(
            |attr| attr.path().is_ident(attribute)
        ) {

            Some(attr) => match attr.parse_args::<Ident>() {

                Ok(argident) => Some(argident.to_string()),

                _ => match attr.parse_args::<Lit>() {

                    Ok(Lit::Str(arglit)) => Some(arglit.value()),

                    _ => None
                }
            }

            _ => None
        }
    }

    fn is_ident<T: ?Sized>(&self, filter: &T) -> bool 
    where
    Ident: PartialEq<T> {

        self.get_ident() == filter
    }

    fn impls_trait<T: ?Sized>(&self, filter: &T) -> bool
    where
    Ident: PartialEq<T> {

        match self.get_trait() {

            Some(ident) => ident == filter,

            None => false
        }
    }

    fn has_attr<T: ?Sized>(&self, filter: &T) -> bool
    where
    Ident: PartialEq<T> {

        self
        .get_attrs()
        .iter()
        .find(
            |attr| attr.path().is_ident(filter)
        )
        .is_some()
    }

    fn has_attr_with_arg<T: ?Sized, U: ?Sized + ToString>(
        &self,
        filter_attr: &T,
        filter_arg: &U
    ) -> bool
    where
    Ident: PartialEq<T> + PartialEq<U> {

        match self
        .get_attr_arg(filter_attr) {

            Some(arg) => arg == filter_arg.to_string(),

            None => false
        }
    }

    fn has_supertrait<T: ?Sized>(&self, filter: &T) -> bool
    where
    Ident: PartialEq<T> {

        match self.get_supertraits() {

            Some(list) => list.iter().find(
                |st| *st == filter
            )
            .is_some(),

            None => false
        }
    }
}