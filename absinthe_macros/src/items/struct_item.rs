// super
use super::*;

// crates.io
use syn::ItemStruct;

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

                Some(
                    Self::from_syn_itemstruct(structitem)
                )
            }

            _ => None
        }
    }
}

impl AbsintheStruct {

    pub fn from_syn_itemstruct(itemstruct: &ItemStruct) -> Self {

        let attributes = itemstruct.attrs.clone();

        let ident = itemstruct.ident.clone();

        Self { attributes, ident }
    }
}