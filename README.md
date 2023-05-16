# ***WORK IN PROGRESS***

The crate is not tested yet. Please don't use it, I may be 
"tested" today, and even then I will work on its 
documentation. So yeah... this is still a WIP...

**Warning:** Items using these attributes should not be declared inside of a 
code block or function. Currently, the macros may show errors if this happens, 
or may not work as expected. So these items (trait declarations, impl blocks, 
struct declarations) should live in the file 'root' level, not inside of other 
items such as modules (declared within the same file) or functions.

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

### Next steps / todo

1. So... There may be use cases where the macros will be used on items which are 
   declared inside a block, which could be a module, a function or something else.

   In these cases, the lookup functions searching for items with the other 
   attributes will not find these items, because they will find the blocks first 
   and since those won't match any of the expected declaration forms, they will
   skip the blocks and say they didn't find any matches.

   So it would be nice to improve these lookup functions in order to go inside 
   other blocks to find the items we're looking for, but how far does the rabbit 
   hole go? How would this impact performance for very large projects? Is there 
   another way?

   Early on I thought about a metaregistry for all abstract factories and their 
   attributes in order to look there, but I haven't tested that idea yet. That 
   may complicate the macros themselves a bit, since they would have to 
   self-register in the meta registry, but maybe, if it works, it could make the
   lookup logic more efficient and the handling of lookup results a bit simpler.

2. Maybe find a way to skip the dependence on the anyhow crate and implement 
   our own Error types? Make it a bit more formal... I don't see any issues with
   that and it means I won't force anyhow on users of the API.

3. This crate is completely focused on abstract factories depending on a 
   configuration object. I want to look at different ways in which abstract 
   factories are used in the real world and see if this logic works for other
   use cases, and if it should be modified or enhanced in order to fit those 
   as well.

4. Code is very dirty, I don't like it. Maybe rewrite it once it's stabilized, 
   and make it cleaner this time.

5. Right now, configuration items must be Clone, the concrete types must be 
   Structs, and the concrete factories must be Send. Does this leave out many 
   use cases? Should I find a way to remove these constraints? I never really 
   tried to go around them, just accepted what the compiler was asking for and 
   didn't question it beyond this.

6. Are Box objects the way to go for the Trait objects? In all cases? I haven't
   tried to do it differently. Maybe we need &dyn, Rc, or... (?)

7. Overall, follow the recommendations in 
   https://towardsdatascience.com/nine-rules-for-creating-procedural-macros-in-rust-595aa476a7ff 
   using a workspace, using proc_macro_errors, etc...

8. If the name of a concrete type contains invalid identifier characters, the 
   macro will panic when trying to name the registry function for that concrete 
   factory. ('factory' attribute). Example: TL-SG3428 contains a hyphen, which 
   is an invalid character for identifiers. Same goes for all other characters 
   which are invalid identifiers. I should fix it!