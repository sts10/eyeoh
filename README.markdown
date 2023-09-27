# eyeoh

This Rust library aims to make getting command line input easier for simple Rust command line tools. 

If you're looking for something more advanced, check out [getops](https://docs.rs/getopts/0.2.18/getopts/) or [clap](https://github.com/clap-rs/clap).

I'm new to Rust so bare with me! I'm slowly adding to [a blog post about this project](https://sts10.github.io/2018/11/02/eyeoh-rust-library.html) if you want to read more.

## Example 1: `gets`

This library provides a `gets` function that is similar to [Ruby's `gets` function](https://ruby-doc.org/core-2.3.0/IO.html#method-i-gets) (short for "get string").

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

Note that `gets` does trim off the newline character at the end of the user's input (and thus is technically more similar to Ruby's `gets.chomp` chain of methods).

## Example 2: `ensure`

`ensure` is a bit more advanced than `gets`. `ensure` attempts to `parse` the user's input into the type that you're calling to (in the example below, that `f64`, a float). If the user's input can't be parsed into the assigned type, the function will loop and ask the user to try again, displaying the string (`str`) that is passed to `ensure`.


```rust
extern crate eyeoh;
use eyeoh::ensure;

fn main(){
  println!("Enter a number");
  // declare a new variable and specify a type of f64
  let num: f64 = ensure("Please try again. Enter a number (a float)").unwrap();

  // we're now out of the `ensure` loop, so we can be 
  // reasonably sure that `num` was parsed into a float Type
  println!(
      "Great, you entered {}, which I'm reasonably sure is a number",
      num
  );
}
```

A user running the above example program might have a back-and-forth like this (if they got it wrong the first two times):

```
Enter a number
four
Please try again. Enter a float
four.four
Please try again. Enter a float
4.4
Great, you entered 4.4, which I'm reasonably sure is a number
```

Note that, since it uses `gets`, `ensure` also removes the trailing new line.

## Example 3: `read_by_line`

eyeoh also provides a function that attempts to simplify the process of reading in a file, say a .txt text file, line-by-line.

```rust
extern crate eyeoh;
use eyeoh::read_by_line;

fn main() {
  println!("Enter file path");
  let file_path = gets().unwrap();
  let vec: Vec<String> = read_by_line(&file_path).unwrap();
  // as an example, print out each line of the file
  for line in vec {
      println!("{}", line);
  }

  println!("Enter file path with numbers in it");
  let file_path = gets().unwrap();
  // read_by_line will attempt to parse the lines of the file into 
  // the type of the elements of the Vector
  let nums_vec: Vec<u64> = read_by_line(&file_path).unwrap();
  for line in &nums_vec {
      println!("line is {}", line);
  }
  // ... allowing us to easily perform numerical functions
  let sum: u64 = nums_vec.iter().sum();
  println!("The sum of all the numbers in that file is {}", sum);
}
```

Note: `read_by_line` trims off any spaces or single quotes at the beginning or end of the user-inputted file name. This is because various operating systems add these characters automatically when users "drag and drop" a file into their terminal.

## Adding this library to your project

Since I haven't yet published this to [Cargo's main crate repository](https://crates.io/), to use this library, add the following to your `Cargo.toml`:

```toml
[dependencies]
eyeoh = { git = "https://github.com/sts10/eyeoh" }
```


If you'd rather clone down the library and use a local version, use:

```toml
[dependencies]
eyeoh = { path = "/path/to/directory/eyeoh" }

```

(Here's a [reference](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories) for this Cargo functionality, if you want to learn more)

## I have a suggestion/idea

Please submit an issue or pull request! 

## Some things I'm still figuring out:

  - Should the `gets` method return a type of `Result` or something else?  Maybe just `String`? `&str`?
  - Is there any way to _safely_ have `ensure` just return the Type `T`, as opposed to a `io::Result<T>`?
  - How should I test this?
  <!-- - Also: if I'm to have `gets` return a type of `Result`, I'd like to provide a second example that does _not_ use `.unwrap()` following `gets()`. Would I use a `match` statement? -->

## Thanks / Acknowledgments

Huge thank you to [Sergey Bugaev](https://mastodon.technology/@bugaevc) for helping me make this library and its functions safer and more efficient!
