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
use core::panic;pub static VAN_GEET_OPENING: Variation = Variation {
        name: Van Geet Opening,
        parent: None,
        variations: &[&dusseldorf_gambit,
&gladbacher_gambit,
&jendrossek_gambit,
&melleby_gambit,
&novosibirsk_variation,
&sleipnir_gambit,
&grunfeld_defense,
&napoleon_attack,
&venezolana_variation,
&warsteiner_gambit,
&tubingen_gambit,
&laroche_gambit,
&battambang_variation,
&dunst_perrenet_gambit,
&nowokunski_gambit,
&sicilian_two_knights,
&liebig_gambit,
&hergert_gambit,
&berlin_gambit,
&myers_attack,
&pfeiffer_gambit,
&twyble_attack,
&hector_gambit,
&reversed_scandinavian,
&damhaug_gambit,
&kluever_gambit,
&reversed_nimzowitsch,
&billockus_johansen_gambit,
&hulsemann_gambit,
&dougherty_gambit],
        lines: &[Line {
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<0>()
            },
            moves: &[
    Normal {
        role: Knight,
        from: B1,
        capture: None,
        to: C3,
        promotion: None,
    },
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(71776119061282560),
                        knight: Bitboard(4755801206503505984),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18446462598732840960),
                        white: Bitboard(327677)
                    }
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 1,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(1) { fullmoves } else { unreachable!() }
            }
        }]
    }