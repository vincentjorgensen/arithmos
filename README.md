===============
       arithmos
===============

Rust library for converting Arabic numerals (1,2,3...) to Ancient Greek
numerals (α', β', γ' ...)

Example usage
=============
```rust
use arithmos::GreekNumeral;

fn main() {
   let num = GreekNumeral::new(616);
   println!("{}", num); // ΧΙϜ'
   assert_eq!("ΧΙϜ'".parse().unwrap(), num);
}
```

License
=======

GPL-3.0

Contribution
------------

Any contribution intentionally submitted for inclusion in this project shall be
licensed as above without additional terms or conditions.
