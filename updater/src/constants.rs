pub const OWNER: &str = "lichess-org";
pub const REPO: &str = "chess-openings";
pub const BRANCH: &str = "master";
pub const WORKFLOW: &str = "lint.yml";
pub const OPENING_FILE_INIT: &str = r#"#[expect(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Move::*;
#[expect(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Role::{Pawn, Knight, Bishop, Rook, Queen, King};
#[expect(clippy::enum_glob_use, reason = "there's 64 variants in this enum, importing them all is stupid")]
use shakmaty::Square::*;
#[expect(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Color::{Black, White};
use shakmaty::bitboard::Bitboard;
use shakmaty::board::Board;
use shakmaty::{ByRole, ByColor, Setup};
use core::num::NonZeroU32;
use crate::{Opening, Code, Volume};
use deranged::RangedU8;"#;
