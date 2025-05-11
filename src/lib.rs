#![no_std]

mod A;
mod B;
mod C;
mod D;
mod E;
pub mod code;
mod opening;
pub mod volume;

pub use A::*;
pub use B::*;
pub use C::*;
pub use D::*;
pub use E::*;
pub use code::Code;
pub use opening::Opening;
pub use volume::Volume;

/// The 0-99 subcategory of an opening.
pub type Subcategory = deranged::RangedU8<0, 99>;
