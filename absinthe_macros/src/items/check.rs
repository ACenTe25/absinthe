// super
use super::*;

// Rust Standard distribution
use std::collections::HashSet;

// CODE

pub trait AbsintheCheck<T>: AbsintheFilter<T>
where T: FromSynItem + 'static {
    
    fn count(&self) -> usize {

        self.len()
    }

    fn unique_arg<U: ?Sized>(&self, attribute: &U) -> bool 
    where Ident: PartialEq<U> {

        let mut hashargs: HashSet<String> = HashSet::new();

        for item in self.iter() {

            match item.get_attr_arg(attribute) {

                Some(arg) => if hashargs.contains(&arg) {

                    return false

                } else {

                    hashargs.insert(arg);
                }

                None => return false
            }
        }

        true
    }

    fn exactly_one(
        &self,
        zero_message: &str,
        many_message: &str
    ) -> Result<T, TS2> {

        match self.count() {

            0 => Err(
                quote!(
                    compile_error!(#zero_message);
                )
            ),

            1 => Ok(self[0].clone()),

            _ => Err(
                quote!(
                    compile_error!(#many_message);
                )
            )
        }
    }
}

impl<T, U> AbsintheCheck<T> for U
where
T: FromSynItem + 'static,
U: AbsintheFilter<T> {}