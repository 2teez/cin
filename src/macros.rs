//! cin is a library that is similar to c++ std::cin
//! in it usage. So, c++ programmer and others will
//! find it easiler to use.
//! It is very verbose to read from a cli in rust as compared
//! with several other popular programming langauge.
//! So, cin, had abstracted away the difficult of writing a verbose
//! code only because you want to get a value of most type from the cli.
//! This macros provides alternative for the function cin in this same crate

#[macro_export]
macro_rules! cin {
    // Custom prompt with type
    ($prompt:expr, $t:ty) => {{
        $crate::__cin_read!($prompt)
            .parse::<$t>()
            .unwrap_or_else(|_| panic!("Failed to parse input as {}", stringify!($t)))
    }};

    // Type only (default prompt)
    ($t:ty) => {{
        cin!("Prompt: ", $t)
    }};

    // Custom prompt (returns String)
    ($prompt:expr) => {{
        $crate::__cin_read!($prompt)
    }};

    // No arguments (returns String with default prompt)
    () => {{
        cin!("Prompt: ")
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __cin_read {
    ($prompt:expr) => {{
        use std::io::{self, Write};

        if !$prompt.is_empty() {
            print!("{}", $prompt);
            io::stdout().flush().expect("Failed to flush stdout");
        }

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        input.trim().to_string()
    }};
}
