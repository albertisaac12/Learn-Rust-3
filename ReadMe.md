# Rust Contd...

**For the rest refer to the Learn-Rust-2 Repo**

## Modules Contd..

By using modules, we can group related definitions together and name why they’re related. Programmers using this code can navigate the code based on the groups rather than having to read through all the definitions, making it easier to find the definitions relevant to them. Programmers adding new functionality to this code would know where to place the code to keep the program organized

```plaintext
crate // lib
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist // function
     │   └── seat_at_table // function
     └── serving
         ├── take_order //function
         ├── serve_order // function
         └── take_payment // function

Example of a module tree
crate => src/lib.rs or sec/lib/mod.rs
front_of_house => src/front_of_house.rs or src/front_of_house/mod.rs
hosting => src/front_of_house/hosting.rs or src/front_of_house/hosting/mod.rs
serving => src/front_of_house/serving.rs or src/front_of_house/serving/mod.rs

```

**A path can take two forms**:

1. An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
2. A relative path starts from the current module and uses `self`, `super`, or an identifier in the current module.

## Note :

`In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default. If you want to make an item like a function or struct private, you put it in a module.
`

## Note :

1. When a `struct` is made public only the struct becomes public the rest of the struct field need to be made public individually
2. When an `enum` is made public all of the fields inside will also become public

## Note :

1. when we bring a module into scope using the `use` it is only valid within the scope where the use was defined.

example :

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

// hosting is valid only in the scope of use since within the mod customer the scope changes the hosting goes out of scope

```

2. Be careful while bringing in 2 different Modules with same types

example :

```rust
use std::io::Result;
use std::fmt::Result;

// here is the issue since both bring in the Result struct into the scope the rust compiler will not understand which Result type to use

//solution 1:

use std::io;
use std::fmt;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}

// solution 2 :
use std::io::Result;
use std::fmt::Result as fmtResult; // use as to name your module import

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

## Ways to bring modules or types or traits into scope

```rust

use std::io;
use std::cmp::Ordering;
// for the above two instead use this
use std::{io,cmp::Ordering};

use std::io;
use std::io::Write;
// for the above two instead use this
use std::io::{self,Write};
```

## The Glob operator \* If we want to bring all public items defined in a path into scope

```rust
use std::collections::*;
```
