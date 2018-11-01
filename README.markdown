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

If you rather clone down the eyeoh library and use a local version, use:

```toml
[dependencies]
eyeoh = { path = "/path/to/directory/eyeoh" }

```
