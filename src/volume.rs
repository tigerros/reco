use core::fmt::{Display, Formatter, Write};
use core::str::FromStr;

/// The A-E volume of an opening.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[repr(u8)]
pub enum Volume {
    A,
    B,
    C,
    D,
    E,
}

impl Volume {
    pub const ALL: [Self; 5] = [Self::A, Self::B, Self::C, Self::D, Self::E];
}

impl From<Volume> for char {
    /// # Examples
    /// ```rust
    /// use reco::Volume;
    /// assert_eq!(char::from(Volume::A), 'A');
    /// assert_eq!(char::from(Volume::B), 'B');
    /// assert_eq!(char::from(Volume::C), 'C');
    /// assert_eq!(char::from(Volume::D), 'D');
    /// assert_eq!(char::from(Volume::E), 'E');
    /// ```
    fn from(volume: Volume) -> Self {
        match volume {
            Volume::A => 'A',
            Volume::B => 'B',
            Volume::C => 'C',
            Volume::D => 'D',
            Volume::E => 'E',
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("invalid volume, expected A-E")
    }
}

impl core::error::Error for Error {}

impl TryFrom<char> for Volume {
    type Error = Error;

    /// The char must match a volume exactly.
    ///
    /// # Examples
    /// See [`Volume::from_str`].
    fn try_from(c: char) -> Result<Self, Self::Error> {
        let volume = match c {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            'E' => Self::E,
            _ => return Err(Error),
        };

        Ok(volume)
    }
}

impl FromStr for Volume {
    type Err = Error;

    /// The string must match a volume exactly.
    ///
    /// # Examples
    /// ```rust
    /// use reco::Volume;
    /// use reco::volume;
    /// use core::str::FromStr;
    /// assert_eq!(Volume::from_str("A"), Ok(Volume::A));
    /// assert_eq!(Volume::from_str("B"), Ok(Volume::B));
    /// assert_eq!(Volume::from_str("C"), Ok(Volume::C));
    /// assert_eq!(Volume::from_str("D"), Ok(Volume::D));
    /// assert_eq!(Volume::from_str("E"), Ok(Volume::E));
    ///
    /// assert_eq!(Volume::from_str(" A"), Err(volume::Error));
    /// assert_eq!(Volume::from_str("A "), Err(volume::Error));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let volume = match s {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "E" => Self::E,
            _ => return Err(Error),
        };

        Ok(volume)
    }
}

impl Display for Volume {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_char((*self).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Tests that it doesn't panic.
        #[test]
        fn parse_random_string(s in "\\PC*") {
            let _ = Volume::from_str(&s);
        }

        /// Tests that it succeeds.
        #[test]
        fn parse_every_valid_string(s in "[A-E]") {
            assert!(Volume::from_str(&s).is_ok());
        }

        /// Tests that it errors.
        #[test]
        fn parse_every_invalid_string(s in ".*[^A-E].*") {
            assert!(Volume::from_str(&s).is_err());
        }

        /// Tests that the [`Display`] implementation is equivalent to converting to a char.
        #[test]
        fn display_eq_to_char(volume in any::<Volume>()) {
            assert_eq!(volume.to_string(), char::from(volume).to_string());
        }
    }
}
