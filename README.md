# ***WORK IN PROGRESS***

The crate is not finished yet. Please don't use it, I may be 
"finished" today, and even then I will work on tests and 
documentation. So yeah... this is still a WIP...

# Absinthe: Rust abstract factory procedural macros

### What is Absinthe?

Absinthe is a Rust crate with some procedural macros to help 
developers implement abstract factories in Rust in a simplified 
manner.

It has 3 attribute-like procedural macros and 1 Trait:

1. `#[with_abstract_factory]` should be used on a Trait 
   declaration. This attribute means that there should be an abstract factory that produces Trait Objects from this Trait. 
   We will call this Trait the **Target Trait**. As arguments to 
   this attribute, we must include:

   - `registry`: an uppercase string literal, which we will call 
     the **Registry Name**,
   - `abstract_factory`: an `Ident`, which is the name we wish to 
     call our abstract factory `Struct`, and we will call it our 
     **Abstract Factory**,
   - `config_registry_key`: a string literal, which is the key in 
     the configuration which represents which concrete type to 
     produce with the abstract factory. We will call it our 
     **Registry Key**.
   - `factory_trait`: an `Ident`, which is the name we wish to 
     call our Factory `Trait`. This is a `Trait` which concrete 
     factories must implement in order to work with the 
     **Abstract Factory**. We will call this our **Factory Trait**.

2. `#[with_factory]` should be used on an `impl` block where the 
   **Target Trait** is implemented for a `Struct`, which I will 
   call a **Concrete Type**. This attribute means that this 
   **Concrete Type** can be produced as a **Target Trait** Trait 
   Object through the **Abstract Factory**. It needs one argument, 
   which should be a string literal matching the representation of 
   this concrete type in the configuration under the **Registry Key**.

3. `#[factory]` should be used on an `impl` block where the 
   **Factory Trait** is implemented for a `Struct`, which I will 
   call a **Concrete Factory**. It needs one argument, which should be an `Ident`, which must be the name of the **Concrete Type** 
   that this **Concrete Factory** will produce.

4. `AbsintheConfig`: this is a Trait which lets its Trait Object 
   to be cloned, and has a `get(key: &str) -> anyhow::Result<String>` method. If you have a Type which lets 
   you store the configuration of an object in order to build it, 
   just implement this Trait in order to make it compatible with 
   the whole **Abstract Factory** stuff.

### How does Absinthe work?

#### `with_abstract_factory` attribute

The `with_abstract_factory` attribute will create a registry, 
called **Registry Name**, using `lazy_static`, and it will be 
a `Mutex<HashMap<String, Box<dyn Factory Trait>>>`, where the 
keys are the names representing a concrete type, and their values 
hold a **Concrete Factory** represented as a **Factory Trait** Object.

It will also create an **Abstract Factory** `Struct` which 
will have a `new(config: Box<dyn AbsintheConfig>) -> anyhow::Result<Box<dyn Target Trait>>` associated function. This 
function will read the **Registry Key** value from the config, and 
use that as the key to get the corresponding **Concrete Factory** 
from the registry. It will then use the `new` method of that 
**Concrete Factory** in order to produce a **Target Trait** Object.

It will also create the **Factory Trait**, which will be `Send` 
so it can be stored in the registry, and it will have a 
`new(&self, config: Box<dyn AbsintheConfig>) -> anyhow::Result<Box<dyn Target Trait>>` method which you must 
implement for each **Concrete Factory**.

Finally, it will check that every `impl` block which implements 
the **Target Trait** for another type, has the `with_factory` 
attribute, so you don't miss any set up when you add a new 
**Concrete Type**.

#### `with_factory` attribute

The `with_factory` attribute will just check that the `Trait` 
being implemented in its `impl` block has the 
`with_abstract_factory` attribute, and it will check that this is 
the only `impl` block for this **Concrete Type** with the 
`with_factory` attribute (so you don't try to create different 
Trait Objects from a single **Concrete Type**), and that there is 
exactly one `impl` block with the `factory` attribute and this 
**Concrete Type** as its argument, so you don't miss it, and also 
so you don't provide two different **Concrete Factories** for one 
**Concrete Type**.

#### `factory` attribute

The `factory` attribute will first check that its argument is the 
`Ident` of a **Concrete Type**. It will do so by looking for 
`impl` blocks with the `with_factory` attribute implementing a 
`Trait` for that `Ident`. It will also get the **Concrete Type** 
name from the argument of that `with_factory` attribute.

Then, it will get the `Trait` from that `impl` block, and since 
it should have the `with_abstract_factory` attribute (given the 
`with_factory` attribute of the `impl` block it came from), it 
will get the **Registry Name**, and the **Factory Trait**.

Then, it will get the **Concrete Factory**, and use all these 
values to register this **Concrete Factory** in the registry for 
the **Abstract Factory**, using the **Concrete Type** name as the 
key, and itself as a **Factory Trait** Object as the value.

### Comments, Suggestions, Issues...

This is my first Rust crate, and the first time I delve into 
procedural macros and abstract factories, so I'm sure there 
will be a lot of potential improvements.

Also, there may be some issues in certain use cases.

If you find any issues, or have some suggestions to improve this 
crate, please submit an issue and I'll do my best to follow up on 
it.

Or send a pull request :D