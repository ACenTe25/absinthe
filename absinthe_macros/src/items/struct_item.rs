// super
use super::*;

// CODE

#[derive(Clone)]
pub struct AbsintheStruct {
    attributes: Vec<Attribute>,
    ident: Ident
}

impl AbsintheItem for AbsintheStruct {

    fn get_ident(&self) -> &Ident {
        
        &self.ident
    }

    fn get_attrs(&self) -> &[Attribute] {
        
        &self.attributes
    }

    fn get_trait(&self) -> Option<&Ident> {
        None   
    }

    fn get_supertraits(&self) -> Option<&[Ident]> {
        None
    }

    fn impls_trait<T: ?Sized>(&self, _filter: &T) -> bool
        where
        Ident: PartialEq<T> {
        false
    }

    fn has_supertrait<T: ?Sized>(&self, _filter: &T) -> bool
        where
        Ident: PartialEq<T> {
        false
    }
}

impl FromSynItem for AbsintheStruct {

    fn from_syn_item(item: &Item) -> Option<Self> {
        
        match item {

            Item::Struct(structitem) => {

                let attributes = structitem.attrs.clone();

                let ident = structitem.ident.clone();

                Some(
                    Self { attributes, ident }
                )
            }

            _ => None
        }
    }
}