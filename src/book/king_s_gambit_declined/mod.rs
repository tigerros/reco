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
use core::panic;pub static KING_S_GAMBIT_DECLINED: Variation = Variation {
        name: King's Gambit Declined,
        parent: None,
        variations: &[&classical_variation,
&petrov_s_defense,
&keene_s_defense,
&panteldakis_countergambit,
&miles_defense,
&mafia_defense,
&falkbeer_countergambit,
&senechaud_countergambit,
&soller_zilbermints_gambit,
&classical,
&keene_defense,
&falkbeer_countergambit_accepted,
&norwalde_variation,
&hobbs_zilbermints_gambit,
&zilbermints_double_countergambit,
&zilbermints_double_gambit,
&queen_s_knight_defense],
        lines: &[]
    }