/// Where the script should generate the opening files.
pub const GEN_DIR: &str = "crates/reco-book/src/generated";
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
use reco_core::{Opening, Code, Volume};
use deranged::RangedU8;
use core::panic;"#;
