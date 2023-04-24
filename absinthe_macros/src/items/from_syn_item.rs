// super
use super::*;

// CODE

pub trait FromSynItem: AbsintheItem {

    fn from_syn_item(item: &Item) -> Option<Self>;
}