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
use core::panic;pub static SICILIAN_DEFENSE: Variation = Variation {
        name: Sicilian Defense,
        parent: None,
        variations: &[&keres_variation,
&king_david_s_opening,
&kopec_system,
&brick_variation,
&smith_morra_gambit_deferred,
&mongoose_variation,
&open,
&four_knights_variation,
&katalimov_variation,
&drazic_variation,
&spielmann_variation,
&venice_attack,
&kan_variation,
&jalalabad_variation,
&nimzowitsch_variation,
&kveinis_variation,
&wing_gambit_deferred,
&boleslavsky_variation,
&lasker_pelikan_variation,
&kotov_gambit,
&coles_sicilian_gambit,
&alapin_variation,
&hyperaccelerated_dragon,
&classical_variation,
&staunton_cochrane_variation,
&acton_extension,
&snyder_variation,
&morphy_gambit,
&accelerated_dragon,
&wing_gambit,
&najdorf_variation,
&lowenthal_variation,
&moscow_variation,
&richter_rauzer_variation,
&nyezhmetdinov_rossolimo_attack,
&bucker_variation,
&franco_sicilian_variation,
&smith_morra_gambit_declined,
&double_dutch_gambit,
&brussels_gambit,
&frederico_variation,
&hyperaccelerated_fianchetto,
&polish_gambit,
&flohr_variation,
&euwe_attack,
&marshall_counterattack,
&pin_variation,
&prins_variation,
&magnus_smith_trap,
&sozin_attack,
&velimirovic_attack,
&gloria_variation,
&myers_attack,
&paulsen_basman_defense,
&halasz_gambit,
&smith_morra_gambit_accepted,
&mc_donnell_attack,
&nimzo_american_variation,
&kupreichik_variation,
&kalashnikov_variation,
&old_sicilian,
&gaw_paw_variation,
&quinteros_variation,
&portsmouth_gambit,
&scheveningen_variation,
&modern_variations,
&closed,
&o_kelly_variation,
&hyperaccelerated_pterodactyl,
&bowdler_attack,
&grob_variation,
&taimanov_variation,
&big_clamp_formation,
&mengarini_variation,
&french_variation,
&godiva_variation,
&lasker_dunne_attack,
&grand_prix_attack,
&kramnik_variation,
&yates_variation,
&amazon_attack,
&kronberger_variation,
&smith_morra_gambit,
&delayed_alapin_variation,
&dragon_variation,
&chekhover_variation,
&heidenfeld_variation],
        lines: &[Line {
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<50>()
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
        from: C7,
        capture: None,
        to: C5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G1,
        capture: None,
        to: F3,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: D7,
        capture: None,
        to: D6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: D2,
        capture: None,
        to: D4,
        promotion: None,
    },
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(68407233016293120),
                        knight: Bitboard(4755801206505340930),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18443093712285204480),
                        white: Bitboard(404809663)
                    }
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) { fullmoves } else { unreachable!() }
            }
        },
Line {
            code: Code {
                volume: Volume::B,
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
        from: C7,
        capture: None,
        to: C5,
        promotion: None,
    },
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(70650236602740480),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18445336716005867520),
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
        },
Line {
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<27>()
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
        from: C7,
        capture: None,
        to: C5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G1,
        capture: None,
        to: F3,
        promotion: None,
    },
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(70650236602740480),
                        knight: Bitboard(4755801206505340930),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18445336716005867520),
                        white: Bitboard(270593983)
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
        },
Line {
            code: Code {
                volume: Volume::B,
                category: Category::new_static::<50>()
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
        from: C7,
        capture: None,
        to: C5,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G1,
        capture: None,
        to: F3,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: D7,
        capture: None,
        to: D6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: D2,
        capture: None,
        to: D4,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: C5,
        capture: Some(
            Pawn,
        ),
        to: D4,
        promotion: None,
    },
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(68407215836423936),
                        knight: Bitboard(4755801206505340930),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18443093695239553024),
                        white: Bitboard(270591935)
                    }
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) { fullmoves } else { unreachable!() }
            }
        }]
    }