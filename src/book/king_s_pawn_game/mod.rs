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
use core::panic;pub static KING_S_PAWN_GAME: Variation = Variation {
        name: King's Pawn Game,
        parent: None,
        variations: &[&clam_variation,
&napoleon_attack,
&busch_gass_gambit,
&damiano_defense,
&dresden_opening,
&tortoise_opening,
&mac_leod_attack,
&wayward_queen_attack,
&mc_connell_defense,
&schulze_muller_gambit,
&king_s_head_opening,
&pachman_wing_gambit,
&leonardis_variation,
&weber_gambit,
&la_bourdonnais_gambit,
&gunderam_defense,
&gunderam_gambit,
&beyer_gambit,
&alapin_opening,
&mengarini_s_opening,
&maroczy_defense,
&tayler_opening,
&bavarian_gambit,
&philidor_gambit],
        lines: &[Line {
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<0>()
            },
            moves: &[
    Normal {
        role: Pawn,
        from: E2,
        capture: None,
        to: E4,
        promotion: None,
    },
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(71776119329713920),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18446462598732840960),
                        white: Bitboard(268496895)
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
        },
Line {
            code: Code {
                volume: Volume::C,
                category: Category::new_static::<20>()
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
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(67272588421820160),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18441959067824947200),
                        white: Bitboard(268496895)
                    }
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) { fullmoves } else { unreachable!() }
            }
        }]
    }