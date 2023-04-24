// crates.io
use anyhow::Result;

// CODE

/// Cloning trait for `AbsintheConf` trait objects. 
/// 
/// It allows for cloning these trait objects safely. You may need to do this 
/// if you have nested Absinthe abstract factories (`absfact`s).
/// 
/// This is automatically implemented for every type `T: AbsintheConf + Clone`.
/// 
/// If you don't want to implement `Clone` but still need to implement 
/// `AbsintheConf`, you must implement `CloneConf` manually.
pub trait CloneConf {
    
    /// Clones the `AbsintheConf` trait object.
    fn clone_conf(&self) -> Box<dyn AbsintheConf>;
}

impl<T: AbsintheConf + Clone> CloneConf for T {
    
    fn clone_conf(&self) -> Box<dyn AbsintheConf> {
        
        Box::new(self.clone())
    }
}

/// Configuration trait required for `Absinthe` factories.
/// 
/// You should implement the Clone trait in any type in which you implement 
/// AbsintheConf, in order for it to get the `CloneConf` supertrait automatically
/// and fulfill the supertrait requirement.
pub trait AbsintheConf: CloneConf + 'static {
    
    /// Gets a `String` configuration value corresponding to a string `key` in 
    /// the trait object. The output `String` is wrapped as a `anyhow::Result`.
    fn get(&self, key: &str) -> Result<String>; 
}