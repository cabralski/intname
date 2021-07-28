//! `intname` is a light and tiny crate for generating integer names based on any integer type ranging from `u8` to `u64` and `i8` to `i128`.
//! The maximum value supported for integers is `[i128::MIN + 1, i128::MAX]`.
//!
//! ```rust
//! use intname::integer_name;
//! assert_eq!(&integer_name(42), "forty-two");
//! ```
//!
//! Works seamless with `signed` integers.
//! ```rust
//! use intname::integer_name;
//! assert_eq!(&integer_name(i32::MAX), "two billion, one hundred forty-seven million, four hundred eighty-three thousand, six hundred forty-seven");
//! ```
//!
//! Huge `signed` or `unsigned` integers can be parsed in nanoseconds.
//! ```rust
//! use intname::integer_name;
//! assert_eq!(&integer_name(i128::MAX), "one hundred seventy undecillion, one hundred forty-one decillion, one hundred eighty-three nonillion, four hundred sixty octillion, four hundred sixty-nine septillion, two hundred thirty-one sextillion, seven hundred thirty-one quintillion, six hundred eighty-seven quadrillion, three hundred three trillion, seven hundred fifteen billion, eight hundred eighty-four million, one hundred five thousand, seven hundred twenty-seven");
//! ```

/// Contains all numbers from `one` to `nineteen`.
const NUMBERS: [&str; 20] = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];

/// Contains all multiples of 10 for building numbers between `20` and `90`.
const MULTIPLES_10: [&str; 10] = ["", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

/// Contains all decimal classes that are below `i128::MAX`.
/// The maximum range of `i128` is `170 undecillion`.
const DECIMAL_CLASSES: [&str; 13] = ["", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion", "sextillion", "septillion", "octillion", "nonillion", "decillion", "undecillion"];

extern crate num_traits;                      // this library uses num_traits for traits for generics.
use num_traits::{PrimInt, ToPrimitive, zero}; // integer trait for generic implementation.
use std::convert::TryInto;                    // good ol' TryInto for some unwrapping.

pub(crate) fn tiny_integer_name(int: u16) -> String {

    if int <= 19 {
        return NUMBERS[int as usize].to_string();
    }

    if (int <= 99) && (int % 10 == 0) {
        return MULTIPLES_10[(int / 10) as usize].to_string();
    }

    if (int <= 99) && (int % 10 != 0) {
        return [
            MULTIPLES_10[(int / 10) as usize],
            "-",
            NUMBERS[(int % 10) as usize]
        ].join("").to_string();
    }

    if int % 100 == 0 {
        return [
            NUMBERS[(int / 100) as usize],
            "hundred"
        ].join(" ").to_string();
    }
    
    return [
        NUMBERS[(int / 100) as usize],
        "hundred",
        &tiny_integer_name(int % 100)
    ].join(" ").trim().to_string();

}

/// Computes the integer name for a given `signed` or `unsigned` integer.
pub fn integer_name<T: PrimInt + ToPrimitive>(int: T) -> String {

    if int.is_zero() {
        return "zero".to_string();
    }
    
    let mut name: String = String::new();

    if int < zero() {
        name.push_str("negative ");
    }

    let mut int: i128 = match int.to_i128() {
        Some(n) => n.abs(),
        None => panic!("The maximum value supported for integers is [i128::MIN + 1, i128::MAX].")
    };

    let mut hundreds: Vec<u16> = Vec::new();

    while int > 0 {
        hundreds.push((int % 1000).try_into().unwrap());
        int /= 1000;
    }

    for (i, n) in hundreds.iter().enumerate().rev() {

        name.push_str([
            &tiny_integer_name(*n),
            DECIMAL_CLASSES[i]
        ].join(" ").trim());

        if i != 0 {
            name.push_str(", ")
        }

    }

    return name.trim().to_string();

}

#[cfg(test)]
mod tests {
    use crate::{integer_name, tiny_integer_name};

    #[test]
    fn tiny_numbers() {
        assert_eq!(&tiny_integer_name(900), "nine hundred");
        assert_eq!(&tiny_integer_name(909), "nine hundred nine");
        assert_eq!(&tiny_integer_name(999), "nine hundred ninety-nine");
    }

    #[test]
    fn small_numbers() {
        assert_eq!(&integer_name(650320), "six hundred fifty thousand, three hundred twenty");
        assert_eq!(&integer_name(123456), "one hundred twenty-three thousand, four hundred fifty-six");
        assert_eq!(&integer_name(293847), "two hundred ninety-three thousand, eight hundred forty-seven");
    }

    #[test]
    fn big_numbers() {
        assert_eq!(&integer_name(-170141183460469231731687303715884105727i128), "negative one hundred seventy undecillion, one hundred forty-one decillion, one hundred eighty-three nonillion, four hundred sixty octillion, four hundred sixty-nine septillion, two hundred thirty-one sextillion, seven hundred thirty-one quintillion, six hundred eighty-seven quadrillion, three hundred three trillion, seven hundred fifteen billion, eight hundred eighty-four million, one hundred five thousand, seven hundred twenty-seven");
        assert_eq!(&integer_name(170141183460469231731687303715884105727i128), "one hundred seventy undecillion, one hundred forty-one decillion, one hundred eighty-three nonillion, four hundred sixty octillion, four hundred sixty-nine septillion, two hundred thirty-one sextillion, seven hundred thirty-one quintillion, six hundred eighty-seven quadrillion, three hundred three trillion, seven hundred fifteen billion, eight hundred eighty-four million, one hundred five thousand, seven hundred twenty-seven");
    }

}