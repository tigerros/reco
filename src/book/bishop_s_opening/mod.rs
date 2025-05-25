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
use core::panic;pub static BISHOP_S_OPENING: Variation = Variation {
        name: Bishop's Opening,
        parent: None,
        variations: &[&ponziani_gambit,
&horwitz_gambit,
&pratt_variation,
&stein_gambit,
&khan_gambit,
&philidor_counterattack,
&warsaw_gambit,
&boden_kieseritzky_gambit,
&vienna_hybrid,
&lopez_variation,
&mc_donnell_gambit,
&lewis_countergambit,
&calabrese_countergambit,
&lisitsyn_variation,
&anderssen_gambit,
&boi_variation,
&philidor_variation,
&thorold_gambit,
&del_rio_variation,
&lewis_gambit,
&berlin_defense,
&krejcik_gambit,
&kitchener_folly,
&pachman_gambit,
&lopez_gambit,
&urusov_gambit,
&four_pawns_gambit],
        lines: &[Line {
            code: Code {
                volume: Volume::C,
                category: Category::new_static::<23>()
            },
            moves: &[
    Normal {
        role: Pawn,
        from: E2,
        capture: None,
        to: E4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E7,
        capture: None,
        to: E5,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: F1,
        capture: None,
        to: C4,
        promotion: None,
    },
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(67272588421820160),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385432514564),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18441959067824947200),
                        white: Bitboard(335605727)
                    }
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 1,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) { fullmoves } else { unreachable!() }
            }
        }]
    }