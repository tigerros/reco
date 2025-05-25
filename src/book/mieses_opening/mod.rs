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
use core::panic;pub static MIESES_OPENING: Variation = Variation {
        name: Mieses Opening,
        parent: None,
        variations: &[&myers_spike_attack,
&venezolana_variation,
&reversed_rat],
        lines: &[Line {
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<0>()
            },
            moves: &[
    Normal {
        role: Pawn,
        from: D2,
        capture: None,
        to: D3,
        promotion: None,
    },
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(71776119061804800),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18446462598732840960),
                        white: Bitboard(587775)
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