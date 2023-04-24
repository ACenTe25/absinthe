// super
use super::*;

// crates.io
use syn::TypeParamBound;

// CODE

#[derive(Clone)]
pub struct AbsintheTrait {
    attributes: Vec<Attribute>,
    ident: Ident,
    supertraits: Vec<Ident>
}

impl AbsintheItem for AbsintheTrait {

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
        
        Some(&self.supertraits)
    }

    fn impls_trait<T: ?Sized>(&self, _filter: &T) -> bool
        where
        Ident: PartialEq<T> {
        false
    }
}

impl FromSynItem for AbsintheTrait {

    fn from_syn_item(item: &Item) -> Option<Self> {
        
        match item {

            Item::Trait(traititem) => {

                let attributes = traititem.attrs.clone();

                let ident = traititem.ident.clone();

                let supertraits: Vec<Ident> = traititem
                .supertraits
                .iter()
                .filter_map(
                    |tpb| {

                        match tpb {

                            TypeParamBound::Trait(traitbound) => match traitbound
                            .path.get_ident() {

                                Some(ident) => Some(ident.clone()),

                                None => None
                            }

                            _ => None
                        }
                    }
                ).collect();

                Some(
                    Self { attributes, ident, supertraits }
                )
            }

            _ => None
        }
    }
}