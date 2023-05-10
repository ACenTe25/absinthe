// super
use super::*;

// crates.io
use syn::{Expr, Lit, Meta, punctuated::Punctuated, Token};

// CODE

pub trait AbsintheItem: Sized + Clone {

    fn get_ident(&self) -> &Ident;

    fn get_attrs(&self) -> &[Attribute];

    fn get_supertraits(&self) -> Option<&[Ident]>;

    fn get_trait(&self) -> Option<&Ident>;

    fn get_attr_single_arg<T: ?Sized>(&self, attribute: &T) -> Option<String>
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

    fn get_attr_pair_arg_str<T: ?Sized + ToString, U: ?Sized + ToString>(
        &self, 
        attr: &T, 
        arg: &U
    ) -> Option<String> {

        let attrs = self.get_attrs();

        let filtered_attrs = attrs
        .iter()
        .filter(
            |x| x.path().is_ident(attr.to_string().as_str())
        );

        for attribute in filtered_attrs {

            let args = match attribute.
            parse_args_with(
                Punctuated::<Meta, Token![,]>::parse_terminated
            ) {

                Ok(parsed_args) => parsed_args,

                Err(_) => continue
            };

            for parsed_arg in args {

                match parsed_arg {

                    Meta::NameValue(namevalue) => {

                        if namevalue.path.is_ident(arg.to_string().as_str()) {

                            match namevalue.value {

                                Expr::Lit(exprlit) => match exprlit.lit {

                                    Lit::Str(txt) => return Some(
                                        txt.value()
                                    ),

                                    _ => return None,
                                },

                                _ => return None
                            }
                        }
                    }

                    _ => continue
                }
            }
        }

        None
    }

    fn get_attr_pair_arg_ident<T: ?Sized + ToString, U: ?Sized + ToString>(
        &self,
        attr: &T,
        arg: &U
    ) -> Option<Ident> {

        let attrs = self.get_attrs();

        let filtered_attrs = attrs
        .iter()
        .filter(
            |x| x.path().is_ident(attr.to_string().as_str())
        );

        for attribute in filtered_attrs {

            let args = match attribute.
            parse_args_with(
                Punctuated::<Meta, Token![,]>::parse_terminated
            ) {

                Ok(parsed_args) => parsed_args,

                Err(_) => continue
            };

            for parsed_arg in args {

                match parsed_arg {

                    Meta::NameValue(namevalue) => {

                        if namevalue.path.is_ident(arg.to_string().as_str()) {

                            match namevalue.value {

                                Expr::Path(exprpath) => return exprpath
                                .path
                                .get_ident()
                                .cloned(),

                                _ => return None
                            }
                        }
                    }

                    _ => continue
                }
            }
        }

        None
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

    fn has_attr_with_single_arg<T: ?Sized, U: ?Sized + ToString>(
        &self,
        filter_attr: &T,
        filter_arg: &U
    ) -> bool
    where
    Ident: PartialEq<T> + PartialEq<U> {

        match self
        .get_attr_single_arg(filter_attr) {

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