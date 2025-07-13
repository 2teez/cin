# cin

## Name
--
cin - works like cin in c++ std::cin to get "any" value from the cli.

## Installation
--

#### Use rust cargo


    cargo run cin


  #### Using from Github link:
 In the Cargo.toml file

    cin = {git = "https://github.com/2teez/cin"}


 use in your rust file like so

    use cin::cin::cin;


 ## Description
 --
 cin makes it easier to receive input from the keyboard when building CLI programs in Rust.

 Reading values from standard input in Rust can feel verbose and unintuitive, especially for beginners. Unlike in C++ or Python, where taking input is simple and introduced early (e.g., std::cin in C++ or input() in Python), Rust requires more boilerplate and familiarity with traits like FromStr and manual read_line parsing.

 This crate aims to simplify that process by providing a familiar, beginner-friendly function: cin::<T>(), inspired by C++'s [std::cin](https://en.cppreference.com/w/cpp/io/cin.html). With cin, you can read values of any supported type with a single line—making input handling in Rust as smooth as in other languages.


## Example
--
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

  1. By annotating the variable: let radius: f64 = cin("Enter radius: ");

  2. specifying the type in the function call: let radius = cin::<f64>("Enter radius: ");

All the verbose boilerplate required to read and parse input is handled for you under the hood by the cin crate.
