# cin

## Name

cin - works like cin in c++ std::cin to get "any" value from the cli.

## Installation


#### Use rust cargo


    cargo add cin


  #### Using from Github link:
 In the Cargo.toml file

    cin = {git = "https://github.com/2teez/cin"}


 use in your rust file like so

    use cin::cin::cin;

and/or use the macro version like so:

    #[macro_use]
    extern crate cin;


 ## Description

 cin makes it easier to get input from the keyboard when building CLI programs in Rust.

 Reading values from standard input in Rust can feel verbose and unintuitive, especially for beginners. Unlike in C++ or Python, where taking input is simple and introduced early (e.g., std::cin in C++ or input() in Python), Rust requires more boilerplate and familiarity with traits like FromStr and manual read line parsing.

 This crate aims to simplify that process by providing a familiar, beginner-friendly function: `cin::<T>()`, or/and `cin!()` macro inspired by C++'s [std::cin](https://en.cppreference.com/w/cpp/io/cin.html). With cin, you can read values of any supported type with a single line—making input handling in Rust as smooth as in other languages.


## Example


In cpp, you can have the following:
```
#include <iostream>
#include <string>
int main()
{
    std::string name;
    std::cout << "Enter your name: ";
    std::cin >> name;
    std::cout << "My name is " << name << std::endl;
}
```

Using, cin crate in rust you can also do like so:
```
use cin::cin::cin;

fn main() {
    let age: u32 = cin("Enter age: ");
    println!("{}", age);
    let name = cin::<String>("Enter your name: ");
    println!("{}", name);
}
```

You can specify the expected input type in two convenient ways:

  1. By annotating the variable: `let radius: f64 = cin("Enter radius: ");`

  2. specifying the type in the function call: `let radius = cin::<f64>("Enter radius: ");`

All the verbose boilerplate required to read and parse input is handled for you under the hood by the cin crate.

### Cin Methods


1. cin

    __*pub fn cin<T: InputSupport>(msg: &str) -> T*__

      `cin` is a generic function that takes a type T implementing the marker trait InputSupport.

      It supports all standard numeric types (integers and floating points) as well as String.

      The function displays a prompt message and reads input from standard input (stdin),

      parsing it into the specified type.

      ```
      println!("Enter two values: ");
      let first = cin::<u64>("Enter first number: ");
      let second = cin::<u64>("Enter second number: ");

      let max = |a: u64, b: u64| {
          if a > b { a } else { b }
      };
      let min = |a, b| {
          if a < b { a } else { b }
      };

      println!("{} <> {}", max(first, second), min(first, second));

      ```

2.  cin!

      The cin! macro makes it easy to get user input in Rust, just like cin in C++. You can show a message and read input from the user without writing much code.

      You can use it in different ways: cin!() shows a default prompt and reads a string, cin!(i32) reads a number, cin!("Name: ") shows a custom message, and cin!("Age: ", u8) shows a message and reads a number. It trims the input and stops with an error if the input is not valid.

      ```
      let age: u32 = cin!("Enter your age: ", u32);
      let name = cin!("Name: ");
      let nickname = cin!(); // "Prompt: "
      let score: f32 = cin!(f32); // "Prompt: "
      ```

## Versioning:
  version 0.1.0 - uses only `cin::<T>()` to receive input from the user.

  version 0.1.1/0.1.2 - corrected and added examples, as shown under the cin function
  documentation.

  versio 0.2.0 - added macro `cin!()`, which covered all practical use cases: `cin!(<Literal String>, <Type>)`, `cin!(<Type>)`, `cin!(<Literal String>)`,`cin!()`
