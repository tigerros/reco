#![no_std]

pub mod volume;
pub mod code;
mod opening;

pub use volume::Volume;
pub use code::Code;
pub use opening::Opening;

/// The 0-99 subcategory of an opening.
pub type Subcategory = deranged::RangedU8<0, 99>;