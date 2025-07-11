#![allow(unused, dead_code)]

use std::io::{self, Write};

pub trait InputSupport {}

/// trait InputSpport for unsigned integer
impl InputSupport for u8 {}
impl InputSupport for u16 {}
impl InputSupport for u32 {}
impl InputSupport for u64 {}
impl InputSupport for u128 {}
impl InputSupport for usize {}

/// trait InputSpport for signed integer
impl InputSupport for i8 {}
impl InputSupport for i16 {}
impl InputSupport for i32 {}
impl InputSupport for i64 {}
impl InputSupport for i128 {}
impl InputSupport for isize {}

impl InputSupport for f32 {}
impl InputSupport for f64 {}
impl InputSupport for String {}

/// Get a number from cli
/// #Example
/// ```no_run
/// use cin::cin::cin;
///
/// #[test]
/// #[ignore]
/// fn magic_number() {
///     let expected = 42u8;
///     let got = cin::<u8>("Enter a number: ");
///     assert_eq!(got, expected);
/// }
/// ```
pub fn cin<T: InputSupport>(msg: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    loop {
        print!("{}", msg);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let trimmed = input.trim();
            match trimmed.parse::<T>() {
                Ok(value) => return value,
                Err(e) => eprintln!("Invalid input '{}': {:?}. Please try again.", trimmed, e),
            }
        } else {
            eprintln!("Failed to read input.");
        }
    }
}

#[cfg(test)]
mod tests;
