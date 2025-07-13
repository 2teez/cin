# cin

## Name
cin - works like cin in c++ std::cin to get "any" value from the cli.

### Installation

#### Use rust cargo

  ```
   cargo run cin
  ```

  #### Using from Github link:
 In the Cargo.toml file
 ```
  cin = {git = "https://github.com/2teez/cin"}
 ```

 use in your rust file like so
 ```
  use cin::cin::cin;
 ```

 ### Description
 `cin` - If you use rust on cli, getting values of any form from the stdin could be very danuting.
 Apart from writing a verbose code in rust just to read from the keyboard, you need to have some
 grabs of rust knowledge to do so.
 Meanwhile, using c++ or python, getting values using the keyboard is actually like a breeze. In your first or second leasson, you can start inputting values from the keyboard.
 To change, this narratives, comes in `cin`, which works similar to c++ [std::cin] from the [#include <iostream>].
