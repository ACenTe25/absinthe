// super
use super::*;

// Rust Standard distribution
use std::ops::DerefMut;

// CODE

pub trait AbsintheFilter<T>: DerefMut<Target = Vec<T>>
where T: FromSynItem + 'static {

    fn filter_ident<U: ?Sized>(&mut self, filter: &U) -> &mut Self
    where Ident: PartialEq<U> {

        self.retain(
            |item| item.is_ident(filter)
        );

        self
    }

    fn filter_with_attr<U: ?Sized>(&mut self, filter: &U) -> &mut Self
    where Ident: PartialEq<U> {

        self.retain(
            |item| item.has_attr(filter)
        );
        
        self
    }

    fn filter_attr_with_arg<U: ?Sized, V: ?Sized + ToString>(
        &mut self,
        filter_attr: &U,
        filter_arg: &V
    ) -> &mut Self
    where Ident: PartialEq<U> + PartialEq<V> {

        self.retain(
            |item| item.has_attr_with_single_arg(filter_attr, filter_arg)
        );

        self
    }

    fn filter_impls_trait<U: ?Sized>(&mut self, filter: &U) -> &mut Self
    where Ident: PartialEq<U> {

        self.retain(
            |item| item.impls_trait(filter)
        );

        self
    }

    fn filter_has_supertrait<U: ?Sized>(&mut self, filter: &U) -> &mut Self
    where Ident:PartialEq<U> {

        self.retain(
            |item| item.has_supertrait(filter)
        );

        self
    }
}

impl<T, U> AbsintheFilter<T> for U
where
T: FromSynItem + 'static,
U: DerefMut<Target = Vec<T>> {}