#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use crate::{Category, Code, Line, Variation, Volume};
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use core::num::NonZeroU32;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use core::unreachable;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use deranged::RangedU8;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Color::{Black, White};
#[allow(
    unused_imports,
    clippy::enum_glob_use,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Move::*;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Role::{Bishop, King, Knight, Pawn, Queen, Rook};
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Square;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::bitboard::Bitboard;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::board::Board;
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::{ByColor, ByRole, Setup};
#[cfg_attr(
    feature = "alloc",
    doc = r#"```rust
# use reco::book::ruy_lopez::OPEN;
assert_eq!(OPEN.original_name(), "Ruy Lopez: Open");
```"#
)]
pub static OPEN: Variation = Variation {
    name: "Open",
    parent: Some(&super::RUY_LOPEZ),
    variations: &[
        &BERGER_VARIATION,
        &BERLIN_VARIATION,
        &BERNSTEIN_VARIATION,
        &BRESLAU_VARIATION,
        &CLASSICAL_DEFENSE,
        &DILWORTH_VARIATION,
        &FRIESS_ATTACK,
        &HARKSEN_GAMBIT,
        &HOWELL_ATTACK,
        &ITALIAN_VARIATION,
        &KARPOV_GAMBIT,
        &KNORRE_VARIATION,
        &MAIN_LINE,
        &MALKIN_VARIATION,
        &MOTZKO_ATTACK,
        &RICHTER_VARIATION,
        &RIGA_VARIATION,
        &SCHLECHTER_DEFENSE,
        &SKIPWORTH_GAMBIT,
        &ST_PETERSBURG_VARIATION,
        &TARRASCH_TRAP,
        &ZUKERTORT_VARIATION,
    ],
    lines: &[
        Line {
            parent: &OPEN,
            code: Code {
                volume: Volume::C,
                category: Category(RangedU8::new_static::<8>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G1,
                    capture: None,
                    to: Square::F3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::B5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::A7,
                    capture: None,
                    to: Square::A6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B5,
                    capture: None,
                    to: Square::A4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
                Normal {
                    role: Knight,
                    from: Square::F6,
                    capture: Some(Pawn),
                    to: Square::E4,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(66992212688301824),
                        knight: Bitboard(4398317043714),
                        bishop: Bitboard(2594073385382182916),
                        rook: Bitboard(9295429630892703777),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606847040),
                    },
                    ByColor {
                        black: Bitboard(13685881884171567104),
                        white: Bitboard(18935663),
                    },
                ) {
                    board
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703744),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
                    fullmoves
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
            },
        },
        Line {
            parent: &OPEN,
            code: Code {
                volume: Volume::C,
                category: Category(RangedU8::new_static::<8>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G1,
                    capture: None,
                    to: Square::F3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::B5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::A7,
                    capture: None,
                    to: Square::A6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B5,
                    capture: None,
                    to: Square::A4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
                Normal {
                    role: Knight,
                    from: Square::F6,
                    capture: Some(Pawn),
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(66992212822517504),
                        knight: Bitboard(4398317043714),
                        bishop: Bitboard(2594073385382182916),
                        rook: Bitboard(9295429630892703777),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606847040),
                    },
                    ByColor {
                        black: Bitboard(13685881884171567104),
                        white: Bitboard(153151343),
                    },
                ) {
                    board
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703744),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(6) {
                    fullmoves
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
            },
        },
        Line {
            parent: &OPEN,
            code: Code {
                volume: Volume::C,
                category: Category(RangedU8::new_static::<8>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G1,
                    capture: None,
                    to: Square::F3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::B5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::A7,
                    capture: None,
                    to: Square::A6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B5,
                    capture: None,
                    to: Square::A4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
                Normal {
                    role: Knight,
                    from: Square::F6,
                    capture: Some(Pawn),
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::B7,
                    capture: None,
                    to: Square::B5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::A4,
                    capture: None,
                    to: Square::B3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(66429271459030784),
                        knight: Bitboard(4398317043714),
                        bishop: Bitboard(2594073385365536772),
                        rook: Bitboard(9295429630892703777),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606847040),
                    },
                    ByColor {
                        black: Bitboard(13685318942808080384),
                        white: Bitboard(136505199),
                    },
                ) {
                    board
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703744),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 1,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(7) {
                    fullmoves
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
            },
        },
        Line {
            parent: &OPEN,
            code: Code {
                volume: Volume::C,
                category: Category(RangedU8::new_static::<8>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G1,
                    capture: None,
                    to: Square::F3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::B5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::A7,
                    capture: None,
                    to: Square::A6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B5,
                    capture: None,
                    to: Square::A4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
                Normal {
                    role: Knight,
                    from: Square::F6,
                    capture: Some(Pawn),
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::B7,
                    capture: None,
                    to: Square::B5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::A4,
                    capture: None,
                    to: Square::B3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D7,
                    capture: None,
                    to: Square::D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D4,
                    capture: Some(Pawn),
                    to: Square::E5,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(64177505870866176),
                        knight: Bitboard(4398317043714),
                        bishop: Bitboard(2594073385365536772),
                        rook: Bitboard(9295429630892703777),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606847040),
                    },
                    ByColor {
                        black: Bitboard(13683067108634656768),
                        white: Bitboard(68721764207),
                    },
                ) {
                    board
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703744),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(8) {
                    fullmoves
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
            },
        },
        Line {
            parent: &OPEN,
            code: Code {
                volume: Volume::C,
                category: Category(RangedU8::new_static::<8>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G1,
                    capture: None,
                    to: Square::F3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::B5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::A7,
                    capture: None,
                    to: Square::A6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B5,
                    capture: None,
                    to: Square::A4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
                Normal {
                    role: Knight,
                    from: Square::F6,
                    capture: Some(Pawn),
                    to: Square::E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::B7,
                    capture: None,
                    to: Square::B5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::A4,
                    capture: None,
                    to: Square::B3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D7,
                    capture: None,
                    to: Square::D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D4,
                    capture: Some(Pawn),
                    to: Square::E5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C8,
                    capture: None,
                    to: Square::E6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C2,
                    capture: None,
                    to: Square::C3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(64177505871127296),
                        knight: Bitboard(4398317043714),
                        bishop: Bitboard(2305860601399869444),
                        rook: Bitboard(9295429630892703777),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606847040),
                    },
                    ByColor {
                        black: Bitboard(13394854324668989440),
                        white: Bitboard(68722025327),
                    },
                ) {
                    board
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703744),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(9) {
                    fullmoves
                } else {
                    #[expect(
                        clippy::unreachable,
                        reason = "intentional. It's in a const expression"
                    )]
                    {
                        unreachable!()
                    }
                },
            },
        },
    ],
};
pub mod berger_variation;
pub use berger_variation::BERGER_VARIATION;
pub mod berlin_variation;
pub use berlin_variation::BERLIN_VARIATION;
pub mod bernstein_variation;
pub use bernstein_variation::BERNSTEIN_VARIATION;
pub mod breslau_variation;
pub use breslau_variation::BRESLAU_VARIATION;
pub mod classical_defense;
pub use classical_defense::CLASSICAL_DEFENSE;
pub mod dilworth_variation;
pub use dilworth_variation::DILWORTH_VARIATION;
pub mod friess_attack;
pub use friess_attack::FRIESS_ATTACK;
pub mod harksen_gambit;
pub use harksen_gambit::HARKSEN_GAMBIT;
pub mod howell_attack;
pub use howell_attack::HOWELL_ATTACK;
pub mod italian_variation;
pub use italian_variation::ITALIAN_VARIATION;
pub mod karpov_gambit;
pub use karpov_gambit::KARPOV_GAMBIT;
pub mod knorre_variation;
pub use knorre_variation::KNORRE_VARIATION;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod malkin_variation;
pub use malkin_variation::MALKIN_VARIATION;
pub mod motzko_attack;
pub use motzko_attack::MOTZKO_ATTACK;
pub mod richter_variation;
pub use richter_variation::RICHTER_VARIATION;
pub mod riga_variation;
pub use riga_variation::RIGA_VARIATION;
pub mod schlechter_defense;
pub use schlechter_defense::SCHLECHTER_DEFENSE;
pub mod skipworth_gambit;
pub use skipworth_gambit::SKIPWORTH_GAMBIT;
pub mod st_petersburg_variation;
pub use st_petersburg_variation::ST_PETERSBURG_VARIATION;
pub mod tarrasch_trap;
pub use tarrasch_trap::TARRASCH_TRAP;
pub mod zukertort_variation;
pub use zukertort_variation::ZUKERTORT_VARIATION;
