use crate::{Category, Volume, volume};
use core::fmt::{Display, Formatter};
use core::str::FromStr;
use deranged::RangedU8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The A00-E99 code of an opening.
pub struct Code {
    pub volume: Volume,
    pub category: Category,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    /// There's no first character.
    MissingVolume,
    /// First character is present, but not a [`Volume`].
    InvalidVolume(volume::Error),
    /// Second or third character is missing.
    MissingCategory,
    /// Second or third character present, but not a base 10 digit.
    InvalidCategory,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingVolume => f.write_str("missing volume"),
            Self::InvalidVolume(e) => write!(f, "invalid volume: {e}"),
            Self::MissingCategory => f.write_str("missing category"),
            Self::InvalidCategory => f.write_str("invalid category"),
        }
    }
}

impl core::error::Error for Error {}

impl FromStr for Code {
    type Err = Error;

    /// Converts a string to a [`Code`].
    ///
    /// The string must be in this form: `{volume}{category:02}`,
    /// where `{:02}` specifies that `category` is always two digits.
    ///
    /// # Examples
    /// ```rust
    /// use reco::{Category, Code, Volume, code, volume};
    /// use core::str::FromStr;
    ///
    /// assert_eq!(
    ///     Code::from_str("A00"),
    ///     Ok(Code {
    ///         volume: Volume::A,
    ///         category: Category::new_static::<0>()
    ///     })
    /// );
    ///
    /// assert_eq!(
    ///     Code::from_str("A0"),
    ///     Err(code::Error::MissingCategory)
    /// );
    ///
    /// assert_eq!(
    ///     Code::from_str("A 00"),
    ///     Err(code::Error::InvalidCategory)
    /// );
    ///
    /// assert_eq!(
    ///     Code::from_str("F80"),
    ///     Err(code::Error::InvalidVolume(volume::Error))
    /// );
    ///
    /// assert_eq!(
    ///     Code::from_str(""),
    ///     Err(code::Error::MissingVolume)
    /// );
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let volume = Volume::try_from(chars.next().ok_or(Error::MissingVolume)?)
            .map_err(Error::InvalidVolume)?;

        let digit_one = chars
            .next()
            .ok_or(Error::MissingCategory)?
            .to_digit(10)
            .ok_or(Error::InvalidCategory)?;

        let digit_two = chars
            .next()
            .ok_or(Error::MissingCategory)?
            .to_digit(10)
            .ok_or(Error::InvalidCategory)?;

        Ok(Self {
            volume,
            #[expect(
                unsafe_code,
                clippy::arithmetic_side_effects,
                clippy::cast_possible_truncation,
                clippy::as_conversions,
                clippy::unwrap_used,
                reason = "both numbers are 0-9. They can't be larger than 99 in this calculation"
            )]
            category: if cfg!(debug_assertions) {
                RangedU8::new((digit_one * 10 + digit_two) as u8).unwrap()
            } else {
                unsafe { RangedU8::new_unchecked((digit_one * 10 + digit_two) as u8) }
            },
        })
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.volume.fmt(f)?;

        // Probably faster than write!(f, "{:02}", self.category.get())
        // but I haven't checked
        let category = self.category.get();
        let digit_one = category / 10;
        let digit_two = category % 10;

        digit_one.fmt(f)?;
        digit_two.fmt(f)
    }
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "it's fine and helpful in tests")]
mod tests {
    use super::*;
    use alloc::format;
    use proptest::prelude::*;

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
        fn display(volume in any::<Volume>(), category in 0u8..99) {
            let category = Category::new(category).unwrap();

            assert_eq!(Code { volume, category }.to_string(), format!("{volume}{:02}", category.get()));
        }
    }
}
