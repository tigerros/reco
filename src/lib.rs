#![no_std]
#![forbid(unsafe_code)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::arithmetic_side_effects,
    clippy::unchecked_duration_subtraction,
    clippy::as_conversions,
    clippy::large_futures,
    clippy::large_stack_arrays,
    clippy::large_stack_frames,
    clippy::modulo_one,
    clippy::iterator_step_by_zero,
    clippy::invalid_regex,
    clippy::print_stdout,
    clippy::print_stderr,
    missing_debug_implementations,
    missing_copy_implementations,
    clippy::missing_const_for_fn,
    clippy::pedantic,
    clippy::allow_attributes,
    clippy::alloc_instead_of_core,
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::cfg_not_test,
    clippy::clone_on_ref_ptr,
    clippy::string_add,
    clippy::unnecessary_safety_comment,
    clippy::unnecessary_safety_doc,
    clippy::tests_outside_test_module
)]
#![deny(
    clippy::unwrap_used,
    clippy::panic,
    clippy::exit,
    clippy::todo,
    clippy::unreachable,
    clippy::panic_in_result_fn,
    clippy::indexing_slicing,
    clippy::string_slice
)]
#![allow(
    clippy::must_use_candidate,
    reason = "triggers too much, kind of pointless"
)]
#![allow(
    clippy::unreadable_literal,
    reason = "triggers only on the generated bitboards"
)]

// #[expect(
//     non_snake_case,
//     reason = "ECO volumes are capitalized, and by being a single letter it would be confusing"
// )]
// mod A;
// #[expect(
//     non_snake_case,
//     reason = "ECO volumes are capitalized, and by being a single letter it would be confusing"
// )]
// mod B;
// #[expect(
//     non_snake_case,
//     reason = "ECO volumes are capitalized, and by being a single letter it would be confusing"
// )]
// mod C;
// #[expect(
//     non_snake_case,
//     reason = "ECO volumes are capitalized, and by being a single letter it would be confusing"
// )]
// mod D;
// #[expect(
//     non_snake_case,
//     reason = "ECO volumes are capitalized, and by being a single letter it would be confusing"
// )]
// mod E;
// pub use A::*;
// pub use B::*;
// pub use C::*;
// pub use D::*;
// pub use E::*;
pub mod code;
mod opening;
pub mod volume;
pub use code::Code;
pub use opening::{Opening, OpeningOwned};
pub use volume::Volume;

/// The 0-99 subcategory of an opening.
pub type Subcategory = deranged::RangedU8<0, 99>;
