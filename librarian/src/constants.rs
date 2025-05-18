/// Where the script should generate the opening files.
pub const GEN_DIR: &str = "src/book";
pub const COMMIT_SOURCE_OUT: &str = "commit-source.txt";
pub const OWNER: &str = "lichess-org";
pub const REPO: &str = "chess-openings";
pub const BRANCH: &str = "master";
pub const WORKFLOW: &str = "lint.yml";
pub const OPENING_FILE_INIT: &str = r#"#[allow(unused_imports, clippy::enum_glob_use, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Move::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Role::{Pawn, Knight, Bishop, Rook, Queen, King};
#[allow(clippy::enum_glob_use, reason = "there's 64 variants in this enum, importing them all is stupid")]
use shakmaty::Square::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Color::{Black, White};
use shakmaty::bitboard::Bitboard;
use shakmaty::board::Board;
use shakmaty::{ByRole, ByColor, Setup};
use core::num::NonZeroU32;
use crate::{Opening, Code, Volume};
use alloc::borrow::Cow;
use deranged::RangedU8;
use core::panic;"#;
pub const BOOK_MOD_INIT: &str = r#"
//! Each entry is an array of openings, because a lot of distinct entries have the same name.
#![allow(
    clippy::allow_attributes,
    reason = "this module is generated, the allows don't know if they are going to be fulfilled"
)]
use crate::Opening;"#;
