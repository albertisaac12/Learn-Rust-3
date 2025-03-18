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

## Strings

```rust
// use format macro to concat the strings

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{s1}-{s2}-{s3}");

```

The `format!` macro works like println!, but instead of printing the output to the screen, it returns a String with the contents.

### NOTE : String is essentially a wrapper around the Vec<u8>

Look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).

```rust
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // + => fn add(self, s: &str) -> String {

    // But wait—the type of &s2 is &String, not &str, as specified in the second parameter to add. So why does it compile?

    // The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..].
```

## Error Handling

Rust doesn’t have exceptions. Instead, it has the type `Result<T, E>` for `recoverable errors` and the `panic!` macro that stops execution when the program encounters an `unrecoverable error`.

Unwinding the Stack or Aborting in Response to a Panic
By default, when a panic occurs the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. However, walking back and cleaning up is a lot of work. Rust, therefore, allows you to choose the alternative of immediately aborting, which ends the program without cleaning up.

Memory that the program was using will then need to be cleaned up by the operating system. If in your project you need to make the resultant binary as small as possible, you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate `[profile]` sections in your Cargo.toml file. For example, if you want to abort on panic in release mode, add this:

```rust
[profile.release]
panic = 'abort'
```

```bash
RUST_BACKTRACE = 1 cargo run
```

## Error Propagation

`?`

```rust

use std::fs::File;
use std::io::{self, Read};
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// ?
//If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

```

There is a difference between what the match expression and what the ? operator does: error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert values from one type into another. When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.
