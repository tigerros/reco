#![no_std]

pub mod a;
pub mod b;
pub mod c;
pub mod d;
pub mod e;
pub mod volume;
pub mod code;
mod opening;

pub use volume::Volume;
pub use code::Code;

/// The 0-99 subcategory of an opening.
pub type Subcategory = deranged::RangedU8<0, 99>;