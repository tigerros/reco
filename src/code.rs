use crate::{Category, Volume, volume};
use core::cmp::Ordering;
use core::fmt::{Display, Formatter};
use core::str::FromStr;
use deranged::RangedU8;
#[cfg(feature = "proptest")]
use proptest::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The A00-E99 code of an opening.
pub struct Code {
    pub volume: Volume,
    pub category: Category,
}

impl From<Code> for u16 {
    /// Gets a 0-499 value representing the code, by multiplying the volume by 100 and adding the category.
    ///
    /// # Examples
    /// ```rust
    /// use reco::{Code, Volume, Category};
    ///
    /// assert_eq!(
    ///     u16::from(Code {
    ///         volume: Volume::A,
    ///         category: Category::new_static::<37>()
    ///     }),
    ///     37 // 0 * 100 + 37
    /// );
    ///
    /// assert_eq!(
    ///     u16::from(Code {
    ///         volume: Volume::E,
    ///         category: Category::new_static::<6>()
    ///     }),
    ///     406 // 4 * 100 + 6
    /// );
    /// ```
    fn from(code: Code) -> Self {
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "u8 * 100 + u8 is not even close to u16::MAX"
        )]
        {
            Self::from(u8::from(code.volume)) * 100 + Self::from(code.category.get())
        }
    }
}

impl PartialOrd for Code {
    /// Uses <a href="#impl-Ord-for-Code">`impl Ord for Code`</a>, so it's guaranteed to be
    /// [`Some`].
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

#[cfg(feature = "proptest")]
impl Arbitrary for Error {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(Self::MissingVolume),
            Just(Self::InvalidVolume(volume::Error)),
            Just(Self::MissingCategory),
            Just(Self::InvalidCategory),
        ]
        .boxed()
    }
}

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
                clippy::arithmetic_side_effects,
                clippy::cast_possible_truncation,
                clippy::as_conversions,
                clippy::unwrap_used,
                reason = "both numbers are 0-9. They can't be larger than 99 in this calculation"
            )]
            category: RangedU8::new((digit_one * 10 + digit_two) as u8).unwrap(),
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
mod tests {
    use super::*;
    #[cfg(feature = "proptest")]
    use alloc::format;

    #[test]
    fn missing_volume() {
        assert_eq!(Code::from_str(""), Err(Error::MissingVolume));
    }

    #[cfg(feature = "proptest")]
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
        fn ord(volume1 in any::<Volume>(), category1 in 0u8..=99, volume2 in any::<Volume>(), category2 in 0u8..=99) {
            let category1 = Category::new(category1).unwrap();
            let category2 = Category::new(category2).unwrap();
            let code1 = Code { volume: volume1, category: category1 };
            let code2 = Code { volume: volume2, category: category2 };

            assert_eq!(code1.cmp(&code2), u16::from(code1).cmp(&u16::from(code2)));
        }

        /// Tests that a [`Code`] can be converted to a string and back.
        #[test]
        fn to_str_from_str(code in any::<Code>()) {
            Code::from_str(&code.to_string()).unwrap();
        }

        #[test]
        fn invalid_volume(s in "[^A-E][0-9][0-9]") {
            assert_eq!(Code::from_str(&s), Err(Error::InvalidVolume(volume::Error)));
        }

        #[test]
        fn missing_category(s in "[A-E]") {
            assert_eq!(Code::from_str(&s), Err(Error::MissingCategory));
        }

        #[test]
        fn missing_category2(s in "[A-E][0-9]") {
            assert_eq!(Code::from_str(&s), Err(Error::MissingCategory));
        }

        #[test]
        fn invalid_category(s in "[A-E][^0-9][0-9]") {
            assert_eq!(Code::from_str(&s), Err(Error::InvalidCategory));
        }

        #[test]
        fn invalid_category2(s in "[A-E][0-9][^0-9]") {
            assert_eq!(Code::from_str(&s), Err(Error::InvalidCategory));
        }
    }
}
