use core::fmt::{Display, Formatter, Write};
use core::str::FromStr;
use deranged::RangedU8;

/// The A-E volume of an opening.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
#[repr(u8)]
pub enum Volume {
    A = 0,
    B,
    C,
    D,
    E,
}

impl Volume {
    /// An array of all possible volumes in ascending order.
    pub const ALL: [Self; 5] = [Self::A, Self::B, Self::C, Self::D, Self::E];

    /// Converts an ASCII byte to a [`Volume`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use reco::{Volume, volume};
    /// use core::str::FromStr;
    ///
    /// assert_eq!(Volume::from_ascii(b'A'), Ok(Volume::A));
    /// assert_eq!(Volume::from_ascii(b'B'), Ok(Volume::B));
    /// assert_eq!(Volume::from_ascii(b'C'), Ok(Volume::C));
    /// assert_eq!(Volume::from_ascii(b'D'), Ok(Volume::D));
    /// assert_eq!(Volume::from_ascii(b'E'), Ok(Volume::E));
    /// assert_eq!(Volume::from_ascii(b'F'), Err(volume::ParseError));
    /// ```
    ///
    /// # Errors
    ///
    /// See [`ParseError`].
    pub const fn from_ascii(s: u8) -> Result<Self, ParseError> {
        match s {
            b'A' => Ok(Self::A),
            b'B' => Ok(Self::B),
            b'C' => Ok(Self::C),
            b'D' => Ok(Self::D),
            b'E' => Ok(Self::E),
            _ => Err(ParseError),
        }
    }

    /// Converts a [`Volume`] into an ASCII byte.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use reco::Volume;
    ///
    /// assert_eq!(Volume::A.as_ascii(), b'A');
    /// assert_eq!(Volume::B.as_ascii(), b'B');
    /// assert_eq!(Volume::C.as_ascii(), b'C');
    /// assert_eq!(Volume::D.as_ascii(), b'D');
    /// assert_eq!(Volume::E.as_ascii(), b'E');
    /// ```
    pub const fn as_ascii(self) -> u8 {
        match self {
            Self::A => b'A',
            Self::B => b'B',
            Self::C => b'C',
            Self::D => b'D',
            Self::E => b'E',
        }
    }
}

impl From<RangedU8<0, 4>> for Volume {
    /// # Examples
    ///
    /// ```rust
    /// use reco::Volume;
    /// use deranged::RangedU8;
    ///
    /// assert_eq!(Volume::from(RangedU8::new_static::<0>()), Volume::A);
    /// assert_eq!(Volume::from(RangedU8::new_static::<1>()), Volume::B);
    /// assert_eq!(Volume::from(RangedU8::new_static::<2>()), Volume::C);
    /// assert_eq!(Volume::from(RangedU8::new_static::<3>()), Volume::D);
    /// assert_eq!(Volume::from(RangedU8::new_static::<4>()), Volume::E);
    /// ```
    fn from(value: RangedU8<0, 4>) -> Self {
        match value.get() {
            0 => Self::A,
            1 => Self::B,
            2 => Self::C,
            3 => Self::D,
            4 => Self::E,
            #[expect(clippy::unreachable, reason = "guaranteed to be in 0..=4 range")]
            _ => unreachable!(),
        }
    }
}

impl From<Volume> for RangedU8<0, 4> {
    /// # Examples
    ///
    /// ```rust
    /// use reco::Volume;
    /// use deranged::RangedU8;
    ///
    /// assert_eq!(RangedU8::from(Volume::A), RangedU8::new_static::<0>());
    /// assert_eq!(RangedU8::from(Volume::B), RangedU8::new_static::<1>());
    /// assert_eq!(RangedU8::from(Volume::C), RangedU8::new_static::<2>());
    /// assert_eq!(RangedU8::from(Volume::D), RangedU8::new_static::<3>());
    /// assert_eq!(RangedU8::from(Volume::E), RangedU8::new_static::<4>());
    /// ```
    fn from(volume: Volume) -> Self {
        match volume {
            Volume::A => Self::new_static::<0>(),
            Volume::B => Self::new_static::<1>(),
            Volume::C => Self::new_static::<2>(),
            Volume::D => Self::new_static::<3>(),
            Volume::E => Self::new_static::<4>(),
        }
    }
}

impl TryFrom<char> for Volume {
    type Error = ParseError;

    /// See [`Self::from_ascii`].
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Self::from_ascii(u8::try_from(c).map_err(|_| ParseError)?)
    }
}

impl From<Volume> for char {
    /// See [`Self::as_ascii`].
    fn from(volume: Volume) -> Self {
        Self::from(volume.as_ascii())
    }
}

impl FromStr for Volume {
    type Err = ParseError;

    /// See [`Self::from_ascii`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseError);
        }

        #[expect(clippy::indexing_slicing, reason = "s.len() == 1")]
        Self::from_ascii(s.as_bytes()[0])
    }
}

impl Display for Volume {
    /// See [`Self::as_ascii`].
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_char(char::from(self.as_ascii()))
    }
}

/// Parsing a [`Volume`] failed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "proptest", derive(proptest_derive::Arbitrary))]
pub struct ParseError;

impl ParseError {
    /// # Examples
    ///
    /// ```rust
    /// use reco::volume;
    ///
    /// assert_eq!(volume::ParseError.as_str(), "invalid volume; expected A-E");
    /// assert_eq!(volume::ParseError.to_string(), volume::ParseError.as_str());
    /// ```
    pub const fn as_str(self) -> &'static str {
        "invalid volume; expected A-E"
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl core::error::Error for ParseError {}

#[cfg(test)]
#[expect(clippy::unwrap_used, clippy::indexing_slicing, reason = "tests")]
mod tests {
    use super::*;
    #[cfg(feature = "proptest")]
    use proptest::prelude::*;

    /// Asserts that [`Ord`] reflects the integer representation of [`Volume`].
    #[test]
    fn ord() {
        for volume1 in Volume::ALL {
            for volume2 in Volume::ALL {
                assert_eq!(
                    volume1.cmp(&volume2),
                    RangedU8::from(volume1).cmp(&RangedU8::from(volume2))
                );
            }
        }
    }

    /// Asserts that [`PartialOrd`] is the same as [`Ord`].
    #[test]
    fn partial_ord_eq_ord() {
        for volume1 in Volume::ALL {
            for volume2 in Volume::ALL {
                assert_eq!(volume1.partial_cmp(&volume2), Some(volume1.cmp(&volume2)));
            }
        }
    }

    /// Asserts that turning a [`Volume`] into a [`RangedU8`] doesn't panic.
    #[test]
    fn to_integer_does_not_panic() {
        for volume in Volume::ALL {
            let _ = RangedU8::<0, 4>::from(volume);
        }
    }

    /// Asserts that turning a [`RangedU8`] into a [`Volume`] doesn't panic.
    #[test]
    fn from_integer_does_not_panic() {
        for i in 0..=4 {
            let _ = Volume::from(RangedU8::new(i).unwrap());
        }
    }

    /// Asserts that [`Volume`] can be converted to an ASCII byte and back to the same value.
    #[test]
    fn ascii_roundtrip() {
        for volume in Volume::ALL {
            assert_eq!(Volume::from_ascii(volume.as_ascii()), Ok(volume));
        }
    }

    /// Asserts that [`Volume`] can be converted to a string and back to the same value.
    #[test]
    fn str_roundtrip() {
        for volume in Volume::ALL {
            assert_eq!(Volume::from_str(&volume.to_string()), Ok(volume));
        }
    }

    /// Asserts that [`Volume::as_ascii`] returns valid ASCII.
    #[test]
    fn is_ascii() {
        for volume in Volume::ALL {
            let ascii = [volume.as_ascii()];
            let str = str::from_utf8(&ascii);
            assert!(str.is_ok());
            assert!(str.unwrap().is_ascii());
        }
    }

    /// Asserts that [`Volume::as_ascii`], [`Volume::to_string`] and [`Volume::from_ascii`], [`Volume::from_str`] are equivalent.
    #[test]
    fn ascii_eq_str() {
        for volume in Volume::ALL {
            let ascii = volume.as_ascii();
            let string = volume.to_string();

            assert_eq!(&[ascii], string.as_bytes());
        }
    }

    #[cfg(feature = "proptest")]
    proptest! {
        /// Asserts that it doesn't panic.
        #[test]
        fn parse_random_string(s in "\\PC*") {
            let _ = Volume::from_str(&s);
        }

        /// Asserts that it succeeds.
        #[test]
        fn parse_every_valid_string(s in "[A-E]") {
            assert!(Volume::from_str(&s).is_ok());
        }

        /// Asserts that it errors.
        ///
        /// `\x00-\x40\x46-\x7F` is the ASCII set minus `A-E`. This is necessary otherwise we
        /// would get either [`ParseError::InvalidLength`] errors as well.
        #[test]
        fn parse_every_invalid_string(s in ".*[\x00-\x40\x46-\x7F].*") {
            assert!(Volume::from_str(&s).is_err());
        }

        /// Asserts that [`Volume::from_str`] and [`Volume::from_ascii`] return the same value
        /// (even if it is an error).
        #[test]
        fn from_str_eq_from_ascii(s in "\\PC*") {
            if s.len() == 1 {
                assert_eq!(Volume::from_str(&s), Volume::from_ascii(s.as_bytes()[0]));
            }
        }
    }
}
