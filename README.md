# intname
`intname` is a tiny crate for generating integer names based on any integer type ranging from `u8` to `u128` and `i8` to `i128`.
The maximum scalar value supported for integers is `u128::MAX`.

```rust
use intname::integer_name;
assert_eq!(&integer_name(42), "forty-two")
```

Huge `signed` or `unsigned` integers can be parsed in nanoseconds.
```rust
use intname::integer_name;
assert_eq!(&integer_name(170141183460469231731687303715884105727i128), "one hundred seventy undecillion, one hundred forty-one decillion, one hundred eighty-three nonillion, four hundred sixty octillion, four hundred sixty-nine septillion, two hundred thirty-one sextillion, seven hundred thirty-one quintillion, six hundred eighty-seven quadrillion, three hundred three trillion, seven hundred fifteen billion, eight hundred eighty-four million, one hundred five thousand, seven hundred twenty-seven");
```