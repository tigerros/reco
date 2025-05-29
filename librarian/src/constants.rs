/// Where the script should generate the opening files.
pub const GEN_DIR: &str = "src/book_gen";
/// Where the script should output the commit SHA of which the changes were sourced from.
pub const COMMIT_SOURCE_OUT: &str = "commit-source.txt";
/// The initial contents of each variation file.
pub const VARIATION_FILE_INIT: &str = r#"#[allow(unused_imports, clippy::enum_glob_use, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Move::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Role::{Pawn, Knight, Bishop, Rook, Queen, King};
#[allow(clippy::enum_glob_use, reason = "there's 64 variants in this enum, importing them all is stupid")]
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Square::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Color::{Black, White};
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::bitboard::Bitboard;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::board::Board;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::{ByRole, ByColor, Setup};
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use core::num::NonZeroU32;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use crate::{Variation, Line, Code, Volume, Category};"#;
/// The initial contents of `book/mod.rs`.
pub const BOOK_MOD_INIT: &str = r#"#![allow(
    clippy::allow_attributes,
    reason = "this module is generated, the allows don't know if they are going to be fulfilled"
)]
use crate::Variation;

#[cfg(test)]
mod tests {
    /// Asserts that each variation in [`super::ALL`] is valid.
    #[test]
    fn valid() {
        for variation in super::ALL {
            assert!(variation.validity().is_ok());
        }
    }
}"#;
