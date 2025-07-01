use crate::{Category, Volume, volume};
use core::cmp::Ordering;
use core::fmt::{Display, Formatter};
use core::str::FromStr;
use deranged::{RangedU8, RangedU16};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
/// The A00-E99 code of an opening.
pub struct Code {
    pub volume: Volume,
    pub category: Category,
}

static ALL: [Code; 500] = {
    let mut all = [Code {
        volume: Volume::A,
        category: Category(RangedU8::new_static::<0>()),
    }; 500];

    let mut i = 0u16;

    while i < 500 {
        #[expect(
            clippy::indexing_slicing,
            clippy::as_conversions,
            reason = "const expr"
        )]
        {
            all[i as usize] = Code {
                #[expect(
                    clippy::unreachable,
                    reason = "[0, 499] / 100 <= 4; it's also a const expr"
                )]
                volume: match i / 100 {
                    0 => Volume::A,
                    1 => Volume::B,
                    2 => Volume::C,
                    3 => Volume::D,
                    4 => Volume::E,
                    _ => unreachable!(),
                },
                #[expect(
                    clippy::as_conversions,
                    reason = "[0, 499] % 100 <= 99; it's also a const expr"
                )]
                category: Category(RangedU8::new((i % 100) as u8).unwrap()),
            };
        }

        i += 1;
    }

    all
};

impl Code {
    /// An array of all codes in ascending order.
    pub const fn all() -> &'static [Self; 500] {
        &ALL
    }

    #[expect(clippy::missing_panics_doc, reason = "doesn't actually panic")]
    /// Converts an ASCII slice to a [`Code`].
    ///
    /// # Examples
    /// ```rust
    /// use reco::{Category, Code, Volume, code, volume};
    ///
    /// assert_eq!(
    ///     Code::from_ascii(b"A00"),
    ///     Ok(Code {
    ///         volume: Volume::A,
    ///         category: Category::new_static::<0>()
    ///     })
    /// );
    ///
    /// assert_eq!(
    ///     Code::from_ascii(b"A0"),
    ///     Err(code::ParseError::InvalidLength)
    /// );
    ///
    /// assert_eq!(
    ///     Code::from_ascii(b"A 00"),
    ///     Err(code::ParseError::InvalidLength)
    /// );
    ///
    /// assert_eq!(
    ///     Code::from_ascii(b"F80"),
    ///     Err(code::ParseError::InvalidVolume(volume::ParseError))
    /// );
    ///
    /// assert_eq!(
    ///     Code::from_ascii(b""),
    ///     Err(code::ParseError::InvalidLength)
    /// );
    /// ```
    ///
    /// # Errors
    ///
    /// See [`ParseError`].
    pub const fn from_ascii(s: &[u8]) -> Result<Self, ParseError> {
        if s.len() != 3 {
            return Err(ParseError::InvalidLength);
        }

        #[expect(clippy::indexing_slicing, reason = "check length above")]
        let volume = match Volume::from_ascii(s[0]) {
            Ok(volume) => volume,
            Err(e) => return Err(ParseError::InvalidVolume(e)),
        };
        #[expect(
            clippy::indexing_slicing,
            clippy::as_conversions,
            reason = "check length above, u8 as char is safe"
        )]
        let Some(digit1) = (s[1] as char).to_digit(10) else {
            return Err(ParseError::InvalidCategory);
        };

        #[expect(
            clippy::cast_possible_truncation,
            clippy::as_conversions,
            reason = "base 10 digit always u8"
        )]
        let digit1 = digit1 as u8;

        #[expect(
            clippy::indexing_slicing,
            clippy::as_conversions,
            reason = "check length above, u8 as char is safe"
        )]
        let Some(digit2) = (s[2] as char).to_digit(10) else {
            return Err(ParseError::InvalidCategory);
        };

        #[expect(
            clippy::cast_possible_truncation,
            clippy::as_conversions,
            reason = "base 10 digit always u8"
        )]
        let digit2 = digit2 as u8;

        Ok(Self {
            volume,
            #[expect(
                clippy::arithmetic_side_effects,
                clippy::unwrap_used,
                reason = "both numbers are 0-9. They can't be larger than 99 in this calculation"
            )]
            category: Category(RangedU8::new(digit1 * 10 + digit2).unwrap()),
        })
    }

    /// Converts a [`Code`] into an ASCII array.
    pub const fn as_ascii(self) -> [u8; 3] {
        let mut ascii = [0u8; 3];

        ascii[0] = self.volume.as_ascii();
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "(0..=99) / 10 <= 10, +b'0' makes it a valid ASCII digit within u8 range"
        )]
        {
            ascii[1] = (self.category.0.get() / 10) + b'0';
        }
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "(0..=99) % 10 <= 10, +b'0' makes it a valid ASCII digit within u8 range"
        )]
        {
            ascii[2] = (self.category.0.get() % 10) + b'0';
        }

        ascii
    }
}

impl PartialOrd for Code {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Code {
    /// Compares the volume first, then the category.
    ///
    /// # Examples
    /// ```rust
    /// use reco::{Code, Volume, Category};
    ///
    /// assert!(
    ///     Code {
    ///         volume: Volume::C,
    ///         category: Category::new_static::<50>()
    ///     } < Code {
    ///         volume: Volume::D,
    ///         category: Category::new_static::<10>()
    ///     },
    /// );
    ///
    /// assert!(
    ///     Code {
    ///         volume: Volume::D,
    ///         category: Category::new_static::<40>()
    ///     } > Code {
    ///         volume: Volume::D,
    ///         category: Category::new_static::<20>()
    ///     },
    /// );
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        let volume_cmp = self.volume.cmp(&other.volume);

        if volume_cmp != Ordering::Equal {
            return volume_cmp;
        }

        self.category.cmp(&other.category)
    }
}

#[expect(
    clippy::fallible_impl_from,
    reason = "not actually fallible, see below"
)]
impl From<Code> for RangedU16<0, 499> {
    /// Converts a [`Code`] to a `0..500` integer, by multiplying the volume by 100 and adding the category.
    ///
    /// # Examples
    /// ```rust
    /// use reco::{Code, Volume, Category};
    /// use deranged::RangedU16;
    ///
    /// assert_eq!(
    ///     RangedU16::from(Code {
    ///         volume: Volume::A,
    ///         category: Category::new_static::<37>()
    ///     }).get(),
    ///     37 // 0 * 100 + 37
    /// );
    ///
    /// assert_eq!(
    ///     RangedU16::from(Code {
    ///         volume: Volume::E,
    ///         category: Category::new_static::<6>()
    ///     }).get(),
    ///     406 // 4 * 100 + 6
    /// );
    /// ```
    fn from(code: Code) -> Self {
        #[expect(
            clippy::arithmetic_side_effects,
            clippy::unwrap_used,
            reason = "[0, 4] * 100 + [0, 99] <= 499"
        )]
        {
            Self::new(
                u16::from(RangedU8::from(code.volume).get()) * 100 + u16::from(code.category.get()),
            )
            .unwrap()
        }
    }
}

#[expect(
    clippy::fallible_impl_from,
    reason = "not actually fallible, see below"
)]
impl From<RangedU16<0, 499>> for Code {
    /// Converts a `0..500` integer to a [`Volume`] by dividing by 100 and a [`Category`] from the remainder.
    ///
    /// # Examples
    /// ```rust
    /// use reco::{Code, Volume, Category};
    /// use deranged::RangedU16;
    ///
    /// assert_eq!(
    ///     RangedU16::new_static::<127>(),
    ///     Code {
    ///         volume: Volume::B, // 127 / 100 = 1
    ///         category: Category::new_static::<27>() // 127 % 100 = 27
    ///     },
    /// );
    ///
    /// assert_eq!(
    ///     RangedU16::new_static::<0>(),
    ///     Code {
    ///         volume: Volume::A,
    ///         category: Category::new_static::<0>()
    ///     }
    /// );
    /// ```
    fn from(integer: RangedU16<0, 499>) -> Self {
        Self {
            #[expect(
                clippy::unwrap_used,
                clippy::as_conversions,
                clippy::cast_possible_truncation,
                reason = "[0, 499] / 100 <= 4"
            )]
            volume: Volume::from(RangedU8::new((integer.get() / 100) as u8).unwrap()),
            #[expect(
                clippy::unwrap_used,
                clippy::as_conversions,
                reason = "[0, 499] % 100 <= 99"
            )]
            category: RangedU8::new((integer.get() % 100) as u8).unwrap().into(),
        }
    }
}

impl FromStr for Code {
    type Err = ParseError;

    /// See [`Self::from_ascii`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_ascii(s.as_bytes())
    }
}

impl Display for Code {
    /// See [`Self::as_ascii`].
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        #[expect(clippy::unwrap_used, reason = "always valid ASCII")]
        f.write_str(str::from_utf8(&self.as_ascii()).unwrap())
    }
}

/// Parsing a [`Code`] failed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
pub enum ParseError {
    /// The byte string was shorter/longer than 3 bytes.
    InvalidLength,
    /// First character is not a [`Volume`].
    InvalidVolume(volume::ParseError),
    /// Second or third character is not a base 10 digit.
    InvalidCategory,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLength => f.write_str("invalid length; expected 3"),
            Self::InvalidVolume(e) => write!(f, "invalid volume: {e}"),
            Self::InvalidCategory => f.write_str("invalid category"),
        }
    }
}

impl core::error::Error for ParseError {}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "tests")]
mod tests {
    use super::*;
    use alloc::format;
    #[cfg(feature = "proptest")]
    use proptest::prelude::*;

    /// Asserts that [`Ord`] reflects the integer representation of [`Code`].
    #[test]
    fn ord() {
        // low tech fuzzing/property testing
        // seems kind of funky but it's much better! exhaustive, simple, fast
        // assuming you have a teeny tiny input space
        for code1 in Code::all() {
            for code2 in Code::all() {
                assert_eq!(
                    code1.cmp(code2),
                    RangedU16::from(*code1).cmp(&RangedU16::from(*code2))
                );
            }
        }
    }

    /// Asserts that [`PartialOrd`] is the same as [`Ord`].
    #[test]
    fn partial_ord_eq_ord() {
        for code1 in Code::all() {
            for code2 in Code::all() {
                assert_eq!(code1.partial_cmp(code2), Some(code1.cmp(code2)));
            }
        }
    }

    /// Asserts that [`Display`] is correct.
    #[test]
    fn display() {
        for code in Code::all() {
            assert_eq!(
                code.to_string(),
                format!("{}{:02}", code.volume, code.category.get())
            );
        }
    }

    /// Asserts that [`Code`] can be converted to an ASCII string and back to the same value.
    #[test]
    fn ascii_roundtrip() {
        for code in Code::all() {
            assert_eq!(Code::from_ascii(&code.as_ascii()), Ok(*code));
        }
    }

    /// Asserts that [`Code`] can be converted to a string and back to the same value.
    #[test]
    fn str_roundtrip() {
        for code in Code::all() {
            assert_eq!(Code::from_str(&code.to_string()), Ok(*code));
        }
    }

    /// Asserts that [`Code::as_ascii`] returns valid ASCII.
    #[test]
    fn is_ascii() {
        for code in Code::all() {
            let ascii = code.as_ascii();
            let str = str::from_utf8(&ascii);
            assert!(str.is_ok());
            assert!(str.unwrap().is_ascii());
        }
    }

    /// Asserts that [`Code::as_ascii`], [`Code::to_string`] and [`Code::from_ascii`], [`Code::from_str`] are equivalent.
    #[test]
    fn ascii_eq_str() {
        for code in Code::all() {
            let ascii = code.as_ascii();
            let string = code.to_string();

            assert_eq!(&ascii, string.as_bytes());
        }
    }

    #[cfg(feature = "proptest")]
    proptest! {
        /// Asserts that it doesn't panic.
        #[test]
        fn parse_random_string(s in "\\PC*") {
            let _ = Code::from_str(&s);
        }

        /// Asserts that it succeeds.
        #[test]
        fn parse_every_valid_string(s in "[A-E][0-9][0-9]") {
            assert!(Code::from_str(&s).is_ok());
        }

        /// Asserts that it errors.
        #[test]
        fn parse_every_invalid_string(s in ".*[^A-E][^0-9][^0-9].*") {
            assert!(Code::from_str(&s).is_err());
        }

        /// Asserts that it errors.
        ///
        /// `\x00-\x40\x46-\x7F` is the ASCII set minus `A-E`. This is necessary otherwise we
        /// would get either [`ParseError::InvalidLength`] errors as well.
        #[test]
        fn invalid_volume(s in "[\x00-\x40\x46-\x7F][0-9][0-9]") {
            assert_eq!(Code::from_str(&s), Err(ParseError::InvalidVolume(volume::ParseError)));
        }

        /// Asserts that it errors.
        #[test]
        fn missing_category(s in "[A-E]") {
            assert_eq!(Code::from_str(&s), Err(ParseError::InvalidLength));
        }

        /// Asserts that it errors.
        #[test]
        fn missing_category2(s in "[A-E][0-9]") {
            assert_eq!(Code::from_str(&s), Err(ParseError::InvalidLength));
        }

        /// Asserts that it errors.
        ///
        /// `\x00-/:-\x7F` is the ASCII set minus `0-9`.  This is necessary otherwise we
        /// would get either [`ParseError::InvalidLength`] errors as well.
        #[test]
        fn invalid_category(s in "[A-E][\x00-/:-\x7F][0-9]") {
            assert_eq!(Code::from_str(&s), Err(ParseError::InvalidCategory));
        }

        /// Asserts that it errors.
        ///
        /// `\x00-/:-\x7F` is the ASCII set minus `0-9`.  This is necessary otherwise we
        /// would get either [`ParseError::InvalidLength`] errors as well.
        #[test]
        fn invalid_category2(s in "[A-E][0-9][\x00-/:-\x7F]") {
            assert_eq!(Code::from_str(&s), Err(ParseError::InvalidCategory));
        }

        /// Asserts that [`Code::from_str`] and [`Code::from_ascii`] return the same value
        /// (even if it is an error).
        #[test]
        fn from_str_eq_from_ascii(s in "\\PC*") {
            assert_eq!(Code::from_str(&s), Code::from_ascii(s.as_bytes()));
        }
    }
}
