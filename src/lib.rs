//! # arithmos
//!
//! A library for converting Arabic numerals to Classical Attic numerals
//! Library formatting inspired by:
//! https://github.com/AA-Turner/roman-numerals/blob/master/rust/src/lib.rs
//!
//! ## License
//!
//! GNU GPL 3

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[cfg(not(feature = "std"))]
extern crate alloc;

use core::fmt;

/// The value of the smallest Greek numeral
pub const MIN: u32 = 0;
/// The value of the largest Greek numeral
pub const MAX: u32 = 9_999;

/// Returned as an error if a numeral is constructed with an invalid input
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
pub struct OutOfRangeError;

impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number out of range (must be between 1 and 9,999).")
    }
}

/// A Greek numeral
///
/// Values from 0 to 9999 are currently supported
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GreekNumeral(u32);

impl GreekNumeral {
    /// Creates a ``GreekNumeral`` for any value in range.
    /// Requires ``value`` to be less than 10,000. 0 (ZERO) is acceptable.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    //     let answer: GreekNumeral = GreekNumeral::new(42).unwrap();
    //     assert_eq!("ΜΒ'", answer.to_uppercase());
    ///
    pub const fn new(value: u32) -> Result<Self, OutOfRangeError> {
        if value < 10_000 {
            // SAFETY: 0 <= value <= 9,999
            Ok(Self(value))
        } else {
            Err(OutOfRangeError)
        }
    }

    /// Return the value of this ``GreekNumeral`` as a ``u32``.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let answer: GreekNumeral = GreekNumeral::new(42)?;
    ///    assert_eq!(answer.as_u32(), 42_u32);
    ///
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    /// Converts a ``GreekNumeral`` to an uppercase string.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let answer: GreekNumeral = GreekNumeral::new(42)?;
    ///    assert_eq!("ΜΒ'", answer.to_uppercase());
    ///
    #[must_use]
    #[cfg(feature = "std")]
    pub fn to_uppercase(self) -> String {
        let mut out = String::new();
        let mut n = self.0;
        if n == 0 {
            out.push_str(&"𐆊".to_string());
        } else {
            for (_, arithmos) in ARITHMOI.iter().enumerate() {
                while n >= arithmos.arabic {
                    n -= arithmos.arabic;
                    out.push_str(arithmos.u_attic);
                }
            }
        }
        out.push_str(&"'".to_string());
        out
    }

    /// Converts a ``GreekNumeral`` to a lowercase string.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let answer: GreekNumeral = GreekNumeral::new(42)?;
    ///    assert_eq!("μβ'", answer.to_lowercase());
    ///
    #[must_use]
    #[cfg(feature = "std")]
    pub fn to_lowercase(self) -> String {
        let mut out = String::new();
        let mut n = self.0;
        if n == 0 {
            out.push_str(&"𐆊".to_string());
        } else {
            for (_, arithmos) in ARITHMOI.iter().enumerate() {
                while n >= arithmos.arabic {
                    n -= arithmos.arabic;
                    out.push_str(arithmos.l_attic);
                }
            }
        }
        out.push_str(&"'".to_string());
        out
    }
}

#[cfg(feature = "std")]
impl fmt::Display for GreekNumeral {
    /// Converts a ``GreekNumeral`` to an uppercase string.
    ///
    /// Example
    /// -------
    ///
    /// .. code-block:: rust
    ///
    ///    let answer: GreekNumeral = GreekNumeral::new(42)?;
    ///    assert_eq!("ΜΒ'", answer.to_string());
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_uppercase())
    }
}

#[cfg(feature = "std")]
struct Arabic2GreekStruct<'a> {
    arabic: u32,
    u_attic: &'a str,
    l_attic: &'a str,
}

#[cfg(feature = "std")]
static ARITHMOI: [Arabic2GreekStruct; 36] = [
    Arabic2GreekStruct {
        arabic: 9000,
        u_attic: "͵Θ",
        l_attic: "͵θ",
    },
    Arabic2GreekStruct {
        arabic: 8000,
        u_attic: "͵Η",
        l_attic: "͵η",
    },
    Arabic2GreekStruct {
        arabic: 7000,
        u_attic: "͵Ζ",
        l_attic: "͵ζ",
    },
    Arabic2GreekStruct {
        arabic: 6000,
        u_attic: "͵Ε",
        l_attic: "͵ε",
    },
    Arabic2GreekStruct {
        arabic: 5000,
        u_attic: "͵Ε",
        l_attic: "͵ε",
    },
    Arabic2GreekStruct {
        arabic: 4000,
        u_attic: "͵Δ",
        l_attic: "͵δ",
    },
    Arabic2GreekStruct {
        arabic: 3000,
        u_attic: "͵Γ",
        l_attic: "͵γ",
    },
    Arabic2GreekStruct {
        arabic: 2000,
        u_attic: "͵Β",
        l_attic: "͵β",
    },
    Arabic2GreekStruct {
        arabic: 1000,
        u_attic: "͵Α",
        l_attic: "͵α",
    },
    Arabic2GreekStruct {
        arabic: 900,
        u_attic: "Ϡ",
        l_attic: "ϡ",
    },
    Arabic2GreekStruct {
        arabic: 800,
        u_attic: "Ω",
        l_attic: "ω",
    },
    Arabic2GreekStruct {
        arabic: 700,
        u_attic: "Ψ",
        l_attic: "ψ",
    },
    Arabic2GreekStruct {
        arabic: 600,
        u_attic: "Χ",
        l_attic: "χ",
    },
    Arabic2GreekStruct {
        arabic: 500,
        u_attic: "Φ",
        l_attic: "φ",
    },
    Arabic2GreekStruct {
        arabic: 400,
        u_attic: "Ρ",
        l_attic: "ρ",
    },
    Arabic2GreekStruct {
        arabic: 300,
        u_attic: "Ρ",
        l_attic: "ρ",
    },
    Arabic2GreekStruct {
        arabic: 200,
        u_attic: "Σ",
        l_attic: "σ",
    },
    Arabic2GreekStruct {
        arabic: 100,
        u_attic: "Ρ",
        l_attic: "ρ",
    },
    Arabic2GreekStruct {
        arabic: 90,
        u_attic: "Ϙ",
        l_attic: "ϙ",
    },
    Arabic2GreekStruct {
        arabic: 80,
        u_attic: "Π",
        l_attic: "π",
    },
    Arabic2GreekStruct {
        arabic: 70,
        u_attic: "Ο",
        l_attic: "ο",
    },
    Arabic2GreekStruct {
        arabic: 60,
        u_attic: "Ξ",
        l_attic: "ξ",
    },
    Arabic2GreekStruct {
        arabic: 50,
        u_attic: "Ν",
        l_attic: "ν",
    },
    Arabic2GreekStruct {
        arabic: 40,
        u_attic: "Μ",
        l_attic: "μ",
    },
    Arabic2GreekStruct {
        arabic: 30,
        u_attic: "Λ",
        l_attic: "λ",
    },
    Arabic2GreekStruct {
        arabic: 20,
        u_attic: "Κ",
        l_attic: "κ",
    },
    Arabic2GreekStruct {
        arabic: 10,
        u_attic: "Ι",
        l_attic: "ι",
    },
    Arabic2GreekStruct {
        arabic: 9,
        u_attic: "Θ",
        l_attic: "θ",
    },
    Arabic2GreekStruct {
        arabic: 8,
        u_attic: "Η",
        l_attic: "η",
    },
    Arabic2GreekStruct {
        arabic: 7,
        u_attic: "Ζ",
        l_attic: "ζ",
    },
    Arabic2GreekStruct {
        arabic: 6,
        u_attic: "Ϝ",
        l_attic: "ϝ",
    },
    Arabic2GreekStruct {
        arabic: 5,
        u_attic: "Ε",
        l_attic: "ε",
    },
    Arabic2GreekStruct {
        arabic: 4,
        u_attic: "Δ",
        l_attic: "δ",
    },
    Arabic2GreekStruct {
        arabic: 3,
        u_attic: "Γ",
        l_attic: "γ",
    },
    Arabic2GreekStruct {
        arabic: 2,
        u_attic: "Β",
        l_attic: "β",
    },
    Arabic2GreekStruct {
        arabic: 1,
        u_attic: "Α",
        l_attic: "α",
    },
];

impl TryFrom<u8> for GreekNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``GreekNumeral`` from an ``u8``.
    ///
    /// Returns ``GreekNumeral`` or ``OutOfRangeError``.
    fn try_from(value: u8) -> Result<Self, OutOfRangeError> {
        Self::new(u32::from(value))
    }
}

impl TryFrom<u16> for GreekNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``GreekNumeral`` from an ``u16``.
    ///
    /// Returns ``GreekNumeral`` or ``OutOfRangeError``.
    fn try_from(value: u16) -> Result<Self, OutOfRangeError> {
        Self::new(u32::from(value))
    }
}

impl TryFrom<u32> for GreekNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``GreekNumeral`` from an ``u32``.
    ///
    /// Returns ``GreekNumeral`` or ``OutOfRangeError``.
    fn try_from(value: u32) -> Result<Self, OutOfRangeError> {
        Self::new(value)
    }
}

impl TryFrom<u64> for GreekNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``GreekNumeral`` from an ``u64``.
    ///
    /// Returns ``GreekNumeral`` or ``OutOfRangeError``.
    fn try_from(value: u64) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<u128> for GreekNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``GreekNumeral`` from an ``u128``.
    ///
    /// Returns ``GreekNumeral`` or ``OutOfRangeError``.
    fn try_from(value: u128) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<usize> for GreekNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``GreekNumeral`` from an ``usize``.
    ///
    /// Returns ``GreekNumeral`` or ``OutOfRangeError``.
    fn try_from(value: usize) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<i8> for GreekNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``GreekNumeral`` from an ``i8``.
    ///
    /// Returns ``GreekNumeral`` or ``OutOfRangeError``.
    fn try_from(value: i8) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<i16> for GreekNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``GreekNumeral`` from an ``i16``.
    ///
    /// Returns ``GreekNumeral`` or ``OutOfRangeError``.
    fn try_from(value: i16) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<i32> for GreekNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``GreekNumeral`` from an ``i32``.
    ///
    /// Returns ``GreekNumeral`` or ``OutOfRangeError``.
    fn try_from(value: i32) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<i64> for GreekNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``GreekNumeral`` from an ``i64``.
    ///
    /// Returns ``GreekNumeral`` or ``OutOfRangeError``.
    fn try_from(value: i64) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

impl TryFrom<i128> for GreekNumeral {
    type Error = OutOfRangeError;

    /// Creates a ``GreekNumeral`` from an ``i128``.
    ///
    /// Returns ``GreekNumeral`` or ``OutOfRangeError``.
    fn try_from(value: i128) -> Result<Self, OutOfRangeError> {
        u32::try_from(value).map_or(Err(OutOfRangeError), Self::new)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_greek_numeral_new() {
        assert_eq!(GreekNumeral::new(0), Ok(GreekNumeral(0_u32)));
        assert_eq!(GreekNumeral::new(1), Ok(GreekNumeral(1_u32)));
        assert_eq!(GreekNumeral::new(1_u8.into()), Ok(GreekNumeral(1_u32)));
        assert_eq!(GreekNumeral::new(1_u32), Ok(GreekNumeral(1_u32)));
        assert_eq!(GreekNumeral::new(42), Ok(GreekNumeral(42_u32)));
        assert_eq!(GreekNumeral::new(9_999), Ok(GreekNumeral(9_999_u32)));
        assert_eq!(GreekNumeral::new(MAX), Ok(GreekNumeral(9_999_u32)));
        assert!(matches!(GreekNumeral::new(10_000), Err(OutOfRangeError)));
        assert!(matches!(GreekNumeral::new(u32::MAX), Err(OutOfRangeError)));
    }

    #[test]
    fn test_try_from_one() {
        assert_eq!(GreekNumeral::try_from(1_u8), Ok(GreekNumeral(1_u32)));
        assert_eq!(GreekNumeral::try_from(1_u16), Ok(GreekNumeral(1_u32)));
        assert_eq!(GreekNumeral::try_from(1_u32), Ok(GreekNumeral(1_u32)));
        assert_eq!(GreekNumeral::try_from(1_u64), Ok(GreekNumeral(1_u32)));
        assert_eq!(GreekNumeral::try_from(1_u128), Ok(GreekNumeral(1_u32)));
        assert_eq!(GreekNumeral::try_from(1_usize), Ok(GreekNumeral(1_u32)));
        assert_eq!(GreekNumeral::try_from(1_i8), Ok(GreekNumeral(1_u32)));
        assert_eq!(GreekNumeral::try_from(1_i16), Ok(GreekNumeral(1_u32)));
        assert_eq!(GreekNumeral::try_from(1_i32), Ok(GreekNumeral(1_u32)));
        assert_eq!(GreekNumeral::try_from(1_i64), Ok(GreekNumeral(1_u32)));
        assert_eq!(GreekNumeral::try_from(1_i128), Ok(GreekNumeral(1_u32)));
    }

    //    #[test]
    //    #[cfg(feature = "std")]
    //    fn test_roman_numeral_round_trip() {
    //        for i in 1..=9_999 {
    //            let r = GreekNumeral::new(i).unwrap().to_string();
    //            let parsed: GreekNumeral = r.parse().unwrap();
    //            let val = parsed.0;
    //            assert_eq!(val, i);
    //        }
    //    }
}
