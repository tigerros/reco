#![cfg_attr(not(test), no_std)]
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
    unsafe_code,
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

extern crate alloc;

pub mod code;
mod opening;
pub mod volume;
pub use code::Code;
pub use opening::Opening;
pub use volume::Volume;
#[cfg(feature = "book")]
#[cfg_attr(docsrs, doc(cfg(feature = "book")))]
pub mod book;
#[cfg(feature = "book")]
mod concat_slices;
#[cfg(feature = "book")]
pub(crate) use concat_slices::concat_slices;

/// The 0-99 category of an opening.
pub type Category = deranged::RangedU8<0, 99>;
