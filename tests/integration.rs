// lib
use absinthe::*;

// crates.io
use anyhow::*;
use lazy_static::*;

// FOR COMPILATION

#[with_abstract_factory(
    registry = "FOO_REGISTRY", 
    abstract_factory = NewFoo, 
    config_registry_key = "type", 
    factory_trait = FooFactory
)]
pub trait Foo {

    fn bar(&self) -> i32;
}

pub struct Bar;

#[with_factory("bar")]
impl Foo for Bar {

    fn bar(&self) -> i32 { 0 }
}

pub struct BarFactory;

#[factory(Bar)]
impl FooFactory for BarFactory {

    fn new(&self, _config: Box<dyn AbsintheConf>) -> anyhow::Result<Box<dyn Foo>> {

        Ok(Box::new(Bar))
    }
}

pub struct Baz;

#[with_factory("baz")]
impl Foo for Baz {

    fn bar(&self) -> i32 { 1 }
}

pub struct BazFactory;

#[factory(Baz)]
impl FooFactory for BazFactory {

    fn new(&self, _config: Box<dyn AbsintheConf>) -> anyhow::Result<Box<dyn Foo>> {

        Ok(Box::new(Baz))
    }
}

#[derive(Clone)]
pub struct Conf {
    type_: String
}

impl AbsintheConf for Conf {

    fn get(&self, _key: &str) -> Result<String> {
        Ok(self.type_.clone())
    }
}

// FOR USAGE

#[test]
fn factory_works() {

    let config_bar = Conf {
        type_: "bar".to_string()
    };

    let config_baz = Conf {
        type_: "baz".to_string()
    };

    let my_foo_bar = NewFoo::new(
        Box::new(config_bar)
    ).expect("Error building a Bar.");

    let my_foo_baz = NewFoo::new(
        Box::new(config_baz)
    ).expect("Error buliding a Baz.");

    assert_eq!(0, my_foo_bar.bar());
    assert_eq!(1, my_foo_baz.bar());
}