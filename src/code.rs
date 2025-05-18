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
    MissingVolume,
    InvalidVolume(volume::Error),
    MissingCategory,
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
                reason = "both numbers are 0-9. They can't be larger than 99 in this calculation"
            )]
            category: unsafe { RangedU8::new_unchecked((digit_one * 10 + digit_two) as u8) },
        })
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.volume.fmt(f)?;

        // Maybe faster than write!(f, "{:02}", self.category.get())
        // but I haven't checked
        let category = self.category.get();
        let digit_one = category / 10;
        let digit_two = category % 10;

        digit_one.fmt(f)?;
        digit_two.fmt(f)
    }
}
