// super
use super::*;

// crates.io
use syn::{ItemImpl, Type};

// CODE

#[derive(Clone)]
pub struct AbsintheImpl {
    attributes: Vec<Attribute>,
    self_type: Ident,
    trait_: Ident
}

impl AbsintheItem for AbsintheImpl {

    fn get_ident(&self) -> &Ident {
        
        &self.self_type
    }

    fn get_attrs(&self) -> &[Attribute] {
        
        &self.attributes
    }

    fn get_trait(&self) -> Option<&Ident> {
        
        Some(&self.trait_)
    }

    fn get_supertraits(&self) -> Option<&[Ident]> {
        
        None
    }

    fn has_supertrait<T: ?Sized>(&self, _filter: &T) -> bool
        where
        Ident: PartialEq<T> {
        false
    }
}

impl FromSynItem for AbsintheImpl {

    fn from_syn_item(item: &Item) -> Option<Self> {
        
        match item {

            Item::Impl(implitem) => {

                Self::from_syn_itemimpl(implitem)
            }

            _ => None
        }
    }
}

impl AbsintheImpl {

    pub fn from_syn_itemimpl(itemimpl: &ItemImpl) -> Option<Self> {

        let attributes = itemimpl.attrs.clone();

        let self_type = match itemimpl.self_ty.as_ref() {

            Type::Path(path) => match path.path.get_ident() {

                Some(ident) => ident.clone(),

                None => return None
            }

            _ => return None
        };

        let trait_ = match itemimpl.trait_.as_ref() {

            Some((_, path, _)) => match path.get_ident() {

                Some(ident) => ident.clone(),

                None => return None
            }

            _ => return None
        };

        Some(
            Self { attributes, self_type, trait_}
        )

    }
}