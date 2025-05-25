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
use core::panic;pub static BIRD_OPENING: Variation = Variation {
        name: Bird Opening,
        parent: None,
        variations: &[&batavo_polish_attack,
&sturm_gambit,
&schlechter_gambit,
&double_duck_formation,
&myers_defense,
&hobbs_zilbermints_gambit,
&horsefly_defense,
&williams_gambit,
&mujannah,
&from_s_gambit,
&hobbs_gambit,
&thomas_gambit,
&williams_zilbermints_gambit,
&swiss_gambit,
&lasker_variation,
&wagner_zwitersch_gambit,
&dutch_variation,
&lasker_gambit,
&siegener_gambit,
&platz_gambit],
        lines: &[Line {
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<2>()
            },
            moves: &[
    Normal {
        role: Pawn,
        from: F2,
        capture: None,
        to: F4,
        promotion: None,
    },
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(71776119598145280),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18446462598732840960),
                        white: Bitboard(536928255)
                    }
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(1) { fullmoves } else { unreachable!() }
            }
        }]
    }