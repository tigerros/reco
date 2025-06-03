use core::cmp::Ordering;
use core::fmt::{Display, Formatter, Write};
use core::str::FromStr;
use deranged::RangedU8;

/// The A-E volume of an opening.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
#[repr(u8)]
pub enum Volume {
    A = 0,
    B,
    C,
    D,
    E,
}

impl PartialOrd for Volume {
    /// Uses <a href="#impl-Ord-for-Volume">`impl Ord for Volume`</a>, so it's guaranteed to be
    /// [`Some`].
    ///
    /// # Examples
    /// ```rust
    /// use reco::Volume;
    /// use core::cmp::Ordering;
    ///
    /// assert_eq!(Volume::D.partial_cmp(&Volume::D), Some(Ordering::Equal));
    /// assert_eq!(Volume::D.partial_cmp(&Volume::A), Some(Ordering::Greater));
    /// assert_eq!(Volume::D.partial_cmp(&Volume::E), Some(Ordering::Less));
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Volume {
    /// # Examples
    /// ```rust
    /// use reco::Volume;
    /// use core::cmp::Ordering;
    ///
    /// assert_eq!(Volume::B.cmp(&Volume::B), Ordering::Equal);
    /// assert_eq!(Volume::B.cmp(&Volume::A), Ordering::Greater);
    /// assert_eq!(Volume::B.cmp(&Volume::C), Ordering::Less);
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        u8::from(*self).cmp(&u8::from(*other))
    }
}

impl Volume {
    pub const ALL: [Self; 5] = [Self::A, Self::B, Self::C, Self::D, Self::E];
}

impl From<Volume> for u8 {
    /// Guaranteed to be `<= 4`.
    ///
    /// # Examples
    /// ```rust
    /// use reco::Volume;
    ///
    /// assert_eq!(u8::from(Volume::A), 0);
    /// assert_eq!(u8::from(Volume::B), 1);
    /// assert_eq!(u8::from(Volume::C), 2);
    /// assert_eq!(u8::from(Volume::D), 3);
    /// assert_eq!(u8::from(Volume::E), 4);
    /// ```
    fn from(volume: Volume) -> Self {
        #[expect(clippy::as_conversions, reason = "Volume is repr(u8)")]
        {
            volume as Self
        }
    }
}

#[expect(
    clippy::fallible_impl_from,
    reason = "not actually fallible, see below"
)]
impl From<Volume> for RangedU8<0, 4> {
    fn from(volume: Volume) -> Self {
        #[expect(
            clippy::unwrap_used,
            reason = "conversion is guaranteed to be in 0..=4 range by tests"
        )]
        Self::new(u8::from(volume)).unwrap()
    }
}

impl From<RangedU8<0, 4>> for Volume {
    /// Infallibly converts a [`RangedU8`] to a [`Volume`].
    ///
    /// # Examples
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

impl From<Volume> for char {
    /// # Examples
    /// ```rust
    /// use reco::Volume;
    ///
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
    /// use reco::{Volume, volume};
    /// use core::str::FromStr;
    ///
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

        /// Tests that [`Volume`] falls within `0..=4`.
        #[test]
        fn within_range(volume in any::<Volume>()) {
            assert!((0..=4).contains(&u8::from(volume)));
        }

        /// Similar to [`within_range`], but tests that [`From<Volume>`] for [`RangedU8`] doesn't panic.
        #[test]
        fn ranged_doesnt_panic(volume in any::<Volume>()) {
            let _ = RangedU8::<0, 4>::from(volume);
        }
    }
}
