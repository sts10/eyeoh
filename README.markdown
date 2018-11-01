# eyeoh

This Rust library aims to make getting command line input easier for simple Rust command line tools. So far it only retrieves `String`s.

## Example

This library provides a `gets` function that is similar to Ruby's `gets` function (short for "get string").

Here's an example:

In your project's `src/main.rs` file:
```rust
extern crate eyeoh;
use eyeoh::gets;
fn main() {
    println!("Enter a string");
    let test_string = gets().unwrap();
    println!("I received {}", test_string);

    let parsed_int: usize = test_string.parse::<usize>().unwrap();
    println!("As an int, that's {}", parsed_int);
}

```

## Adding this library to your project

Since I haven't yet published this to the main crate directory, to use this library, add the following to your `Cargo.toml`:

```toml
[dependencies]
eyeoh = { git = "https://github.com/sts10/eyeoh" }
```

If you'd rather clone down the library and use a local version, use:

```toml
[dependencies]
eyeoh = { path = "/path/to/directory/eyeoh" }

```

## I have a suggestion/idea

Please submit an issue or pull request! 

Somethings I'm still figuring out:
  - Should the `gets` method return a type of `Result` or something else?  Maybe just `String`? `&str`?
  - Should I include an easy way to get an integer (maybe `usize`) or `float` from the command line?
  - Is there some cool way I can use traits/enums to allow a `gets` method to accept a type and attempt to parse the returning value into that type. For example: `let amount:f64 = gets().unwrap();` or `let amount = gets(f64).unwrap();`
  - Also: if I'm to have `gets` return a type of `Result`, I'd like to provide a second example that does _not_ use `.unwrap()` following `gets()`. Would I use a `match` statement?


