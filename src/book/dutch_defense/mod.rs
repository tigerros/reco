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
use core::panic;pub static DUTCH_DEFENSE: Variation = Variation {
        name: Dutch Defense,
        parent: None,
        variations: &[&staunton_gambit_accepted,
&manhattan_gambit,
&hort_antoshin_system,
&leningrad,
&semi_leningrad_variation,
&bellon_gambit,
&stonewall_variation,
&rubinstein_variation,
&fianchetto_attack,
&janzen_korchnoi_gambit,
&alekhine_variation,
&normal_variation,
&spielmann_gambit,
&leningrad_variation,
&blackmar_s_second_gambit,
&queen_s_knight_variation,
&hevendehl_gambit,
&korchnoi_attack,
&hopton_attack,
&classical_variation,
&kingfisher_gambit,
&krause_variation,
&senechaud_gambit,
&omega_isis_gambit,
&raphael_variation,
&nimzo_dutch_variation,
&bladel_variation,
&alapin_variation,
&krejcik_gambit,
&staunton_gambit,
&blackburne_variation,
&fianchetto_variation],
        lines: &[Line {
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<80>()
            },
            moves: &[
    Normal {
        role: Pawn,
        from: D2,
        capture: None,
        to: D4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: F7,
        capture: None,
        to: F5,
        promotion: None,
    },
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(62769057379710720),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18437455536917053440),
                        white: Bitboard(134281215)
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
        },
Line {
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<84>()
            },
            moves: &[
    Normal {
        role: Pawn,
        from: D2,
        capture: None,
        to: D4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: F7,
        capture: None,
        to: F5,
        promotion: None,
    },
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
                        pawn: Bitboard(62769057446818560),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18437455536917053440),
                        white: Bitboard(201389055)
                    }
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) { fullmoves } else { unreachable!() }
            }
        }]
    }