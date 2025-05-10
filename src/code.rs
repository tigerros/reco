use core::fmt::{Display, Formatter};
use core::str::FromStr;
use deranged::RangedU8;
use crate::{volume, Subcategory, Volume};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The A00-E99 code of an opening.
pub struct Code {
    pub volume: Volume,
    pub subcategory: Subcategory,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    MissingVolume,
    InvalidVolume(volume::Error),
    MissingSubcategory,
    InvalidSubcategory,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingVolume => f.write_str("missing volume"),
            Self::InvalidVolume(e) => write!(f, "invalid volume: {e}"),
            Self::MissingSubcategory => f.write_str("missing subcategory"),
            Self::InvalidSubcategory => f.write_str("invalid subcategory"),
        }
    }
}

impl core::error::Error for Error {}

impl FromStr for Code {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let volume = Volume::try_from(
            chars.next().ok_or(Error::MissingVolume)?
        )
            .map_err(Error::InvalidVolume)?;

        let digit_one = chars.next()
            .ok_or(Error::MissingSubcategory)?
            .to_digit(10)
            .ok_or(Error::InvalidSubcategory)?;

        let digit_two = chars.next()
            .ok_or(Error::MissingSubcategory)?
            .to_digit(10)
            .ok_or(Error::InvalidSubcategory)?;

        Ok(Self {
            volume,
            // CLIPPY: Both numbers are 0-9. They can't be larger than 99 in this calculation.
            subcategory: RangedU8::new((digit_one * 10 + digit_two) as u8).unwrap()
        })
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.volume.fmt(f)?;
        
        // Maybe faster than write!(f, "{:02}", self.subcategory.get())
        // but I haven't checked
        let subcategory = self.subcategory.get();
        let digit_one = subcategory / 10;
        let digit_two = subcategory % 10;

        digit_one.fmt(f)?;
        digit_two.fmt(f)
    }
}