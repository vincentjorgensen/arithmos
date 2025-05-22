# arithmos

A Rust library for converting Arabic numerals (1,2,3...) to Ancient Greek
numerals (α', β', γ' ...)

Integers between 0 and 999,999 (inclusive) are currently supported. Any number
beyond this range will return an ``OutOfRangeError``.

Both uppercase and lowercase Greek formatting are supported. 

## Example usage

### Create Greek numerals

```rust
use arithmos::GreekNumeral;

let num = GreekNumeral::new(616)?;
assert_eq!(num.to_string(), "ΧΙϜ'");

let num: RomanNumeral = 49_999.try_into().unwrap();
println!("{}", num);  // ͵Μ͵ΘϠϘΘ'
```

## License

GPL-3.0

