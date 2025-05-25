#[allow(unused_imports, clippy::enum_glob_use, reason = "because the code is generated, we don't know if it's going to be used")]
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
use crate::{Variation, Line, Code, Volume, Category};
use core::panic;pub static BLACKMAR_DIEMER_GAMBIT_DECLINED: Variation = Variation {
        name: Blackmar-Diemer Gambit Declined,
        parent: None,
        variations: &[&gedult_defense,
&langeheinicke_defense,
&brombacher_countergambit,
&elbert_countergambit,
&weinsbach_defense,
&vienna_defense,
&o_kelly_defense,
&lamb_defense],
        lines: &[]
    }