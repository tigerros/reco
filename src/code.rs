use crate::{Category, Volume, volume};
use core::cmp::Ordering;
use core::fmt::{Display, Formatter};
use core::str::FromStr;
use deranged::{RangedU8, RangedU16};
#[cfg(feature = "proptest")]
use proptest::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The A00-E99 code of an opening.
pub struct Code {
    pub volume: Volume,
    pub category: Category,
}

impl Code {
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
        #[expect(clippy::indexing_slicing, reason = "check length above")]
        let digit1 = s[1].saturating_sub(b'0');

        if digit1 > 9 {
            return Err(ParseError::InvalidCategory);
        }

        #[expect(clippy::indexing_slicing, reason = "check length above")]
        let digit2 = s[2].saturating_sub(b'0');

        if digit2 > 9 {
            return Err(ParseError::InvalidCategory);
        }

        Ok(Self {
            volume,
            #[expect(
                clippy::arithmetic_side_effects,
                clippy::unwrap_used,
                reason = "both numbers are 0-9. They can't be larger than 99 in this calculation"
            )]
            category: RangedU8::new(digit1 * 10 + digit2).unwrap(),
        })
    }

    /// Converts a [`Code`] into an ASCII array.
    pub const fn as_ascii(self) -> [u8; 3] {
        let mut bytes = [0u8; 3];

        bytes[0] = self.volume.as_ascii();
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "(0..=99) / 10 <= 10, +b'0' makes it a valid ASCII digit within u8 range"
        )]
        {
            bytes[1] = (self.category.get() / 10) + b'0';
        }
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "(0..=99) % 10 <= 10, +b'0' makes it a valid ASCII digit within u8 range"
        )]
        {
            bytes[2] = (self.category.get() % 10) + b'0';
        }

        bytes
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
                clippy::arithmetic_side_effects,
                clippy::unwrap_used,
                reason = "[0, 499] / 100 <= 4"
            )]
            volume: Volume::from(RangedU8::new((integer.get() / 100) as u8).unwrap()),
            #[expect(
                clippy::arithmetic_side_effects,
                clippy::unwrap_used,
                reason = "[0, 499] % 100 <= 99"
            )]
            category: Category::new((integer.get() % 100) as u8).unwrap(),
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

#[cfg(feature = "proptest")]
impl Arbitrary for Code {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        (
            Volume::arbitrary(),
            #[expect(clippy::unwrap_used, reason = "Category range is 0-99")]
            (0u8..=99).prop_map(|n| Category::new(n).unwrap()),
        )
            .prop_map(|(volume, category)| Self { volume, category })
            .boxed()
    }
}

/// Parsing a [`Code`] failed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

#[cfg(feature = "proptest")]
impl Arbitrary for ParseError {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(Self::InvalidLength),
            Just(Self::InvalidVolume(volume::ParseError)),
            Just(Self::InvalidCategory),
        ]
        .boxed()
    }
}

#[cfg(test)]
#[cfg(feature = "proptest")]
mod tests {
    use super::*;
    use alloc::format;

    proptest! {
        /// Tests that it doesn't panic.
        #[test]
        fn parse_random_string(s in "\\PC*") {
            let _ = Code::from_str(&s);
        }

        /// Tests that it succeeds.
        #[test]
        fn parse_every_valid_string(s in "[A-E][0-9][0-9]") {
            assert!(Code::from_str(&s).is_ok());
        }

        /// Tests that it errors.
        #[test]
        fn parse_every_invalid_string(s in ".*[^A-E][^0-9][^0-9].*") {
            assert!(Code::from_str(&s).is_err());
        }

        /// Tests that the [`Display`] implementation is correct.
        #[test]
        fn display(volume in any::<Volume>(), category in 0u8..=99) {
            let category = Category::new(category).unwrap();

            assert_eq!(Code { volume, category }.to_string(), format!("{volume}{:02}", category.get()));
        }

        #[test]
        fn ord(code1 in any::<Code>(), code2 in any::<Code>()) {
            assert_eq!(code1.cmp(&code2), RangedU16::from(code1).cmp(&RangedU16::from(code2)));
        }

        /// Tests that a [`Code`] can be converted to a string and back.
        #[test]
        fn to_str_from_str(code in any::<Code>()) {
            Code::from_str(&code.to_string()).unwrap();
        }

        #[test]
        fn invalid_volume(s in "[^A-E][0-9][0-9]") {
            assert_eq!(Code::from_str(&s), Err(ParseError::InvalidVolume(volume::ParseError)));
        }

        #[test]
        fn missing_category(s in "[A-E]") {
            assert_eq!(Code::from_str(&s), Err(ParseError::InvalidLength));
        }

        #[test]
        fn missing_category2(s in "[A-E][0-9]") {
            assert_eq!(Code::from_str(&s), Err(ParseError::InvalidLength));
        }

        #[test]
        fn invalid_category(s in "[A-E][^0-9][0-9]") {
            assert_eq!(Code::from_str(&s), Err(ParseError::InvalidCategory));
        }

        #[test]
        fn invalid_category2(s in "[A-E][0-9][^0-9]") {
            assert_eq!(Code::from_str(&s), Err(ParseError::InvalidCategory));
        }

        #[test]
        fn from_str_eq_from_bytes(s in "\\PC*") {
            assert_eq!(Code::from_str(&s), Code::from_ascii(s.as_bytes()));
        }

         #[test]
        fn from_str_eq_from_bytes_valid(code in any::<Code>()) {
            assert_eq!(Code::from_str(&code.to_string()), Code::from_ascii(&code.as_ascii()));
        }
    }
}
