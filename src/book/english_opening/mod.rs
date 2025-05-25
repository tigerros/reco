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
use core::panic;pub static ENGLISH_OPENING: Variation = Variation {
        name: English Opening,
        parent: None,
        variations: &[&myers_gambit,
&wing_gambit,
&anglo_scandinavian_defense,
&myers_defense,
&drill_variation,
&caro_kann_defensive_system,
&adorjan_defense,
&anglo_lithuanian_variation,
&neo_catalan,
&mikenas_carls,
&closed,
&zilbermints_gambit,
&anglo_indian_defense,
&four_knights_system,
&anglo_dutch_defense,
&jaenisch_gambit,
&neo_catalan_declined,
&achilles_omega_gambit,
&anglo_dutch_variation,
&king_s_english_variation,
&romanishin_gambit,
&symmetrical_variation,
&mikenas_carls_variation,
&carls_bremen_system,
&symmetrical,
&king_s_english,
&anglo_grunfeld_defense,
&the_whale,
&porcupine_variation,
&great_snake_variation,
&agincourt_defense,
&wade_gambit],
        lines: &[Line {
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<10>()
            },
            moves: &[
    Normal {
        role: Pawn,
        from: C2,
        capture: None,
        to: C4,
        promotion: None,
    },
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(71776119128390400),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18446462598732840960),
                        white: Bitboard(67173375)
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