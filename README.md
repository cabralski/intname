# intname
 `intname` is a tiny crate for generating integer names based on any integer type ranging from `u8` to `u128` and `i8` to `i128`.
 The maximum value supported for integers is `u128`.

 ```rust
 use intname::integer_name;
 assert_eq!(&integer_name(42), "forty-two")
 ```