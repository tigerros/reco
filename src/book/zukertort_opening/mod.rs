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
use core::panic;pub static ZUKERTORT_OPENING: Variation = Variation {
        name: Zukertort Opening,
        parent: None,
        variations: &[&tennison_gambit,
&regina_nu_gambit,
&sicilian_invitation,
&queenside_fianchetto_variation,
&st_george_defense,
&arctic_defense,
&queen_s_gambit_invitation,
&slav_invitation,
&reversed_mexican_defense,
&polish_defense,
&old_indian_attack,
&herrstrom_gambit,
&pirc_invitation,
&pachman_gambit,
&vos_gambit,
&wade_defense,
&ampel_variation,
&reversed_grunfeld,
&black_mustang_defense,
&lemberger_gambit,
&dutch_variation,
&ross_gambit,
&quiet_system,
&lisitsyn_gambit_deferred,
&double_fianchetto_attack,
&kingside_fianchetto,
&myers_polish_attack,
&drunken_cavalry_variation,
&lisitsyn_gambit,
&nimzo_larsen_variation,
&the_potato,
&the_walrus,
&basman_defense,
&ware_defense,
&shabalov_gambit,
&speelsmet_gambit,
&santasiere_s_folly],
        lines: &[Line {
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<5>()
            },
            moves: &[
    Normal {
        role: Knight,
        from: G1,
        capture: None,
        to: F3,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G8,
        capture: None,
        to: F6,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: B1,
        capture: None,
        to: C3,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: B8,
        capture: None,
        to: C6,
        promotion: None,
    },
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(71776119061282560),
                        knight: Bitboard(39582420959232),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(13690700974648197120),
                        white: Bitboard(2424765)
                    }
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 4,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) { fullmoves } else { unreachable!() }
            }
        },
Line {
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<4>()
            },
            moves: &[
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
                        pawn: Bitboard(71776119061282560),
                        knight: Bitboard(4755801206505340930),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18446462598732840960),
                        white: Bitboard(2162623)
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
        },
Line {
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<5>()
            },
            moves: &[
    Normal {
        role: Knight,
        from: G1,
        capture: None,
        to: F3,
        promotion: None,
    },
    Normal {
        role: Knight,
        from: G8,
        capture: None,
        to: F6,
        promotion: None,
    },
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(71776119061282560),
                        knight: Bitboard(144150372450041858),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(13834811764677541888),
                        white: Bitboard(2162623)
                    }
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 2,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) { fullmoves } else { unreachable!() }
            }
        },
Line {
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<6>()
            },
            moves: &[
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
        to: D5,
        promotion: None,
    },
],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(69524353607335680),
                        knight: Bitboard(4755801206505340930),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992)
                    },
                    ByColor {
                        black: Bitboard(18444210833278894080),
                        white: Bitboard(2162623)
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