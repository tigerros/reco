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
# use reco::book::semi_slav_defense::CHIGORIN_DEFENSE;
assert_eq!(CHIGORIN_DEFENSE.original_name(), "Semi-Slav Defense: Chigorin Defense");
```"#
)]
pub static CHIGORIN_DEFENSE: Variation = Variation {
    name: "Chigorin Defense",
    parent: Some(&super::SEMI_SLAV_DEFENSE),
    variations: &[],
    lines: &[
        Line {
            parent: &CHIGORIN_DEFENSE,
            code: Code {
                volume: Volume::D,
                category: Category(RangedU8::new_static::<4>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
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
                    from: Square::C2,
                    capture: None,
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C7,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B1,
                    capture: None,
                    to: Square::C3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E6,
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
                    to: Square::D7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::D3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::D6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(63916844508046080),
                        knight: Bitboard(2286984188133376),
                        bishop: Bitboard(288239172245258244),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(11379254888539095040),
                        white: Bitboard(205317021),
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
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 4,
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
            parent: &CHIGORIN_DEFENSE,
            code: Code {
                volume: Volume::D,
                category: Category(RangedU8::new_static::<4>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
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
                    from: Square::C2,
                    capture: None,
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C7,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B1,
                    capture: None,
                    to: Square::C3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E6,
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
                    to: Square::D7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::D3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::D6,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: Square::D1,
                    capture: None,
                    to: Square::C2,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(63916844508046080),
                        knight: Bitboard(2286984188133376),
                        bishop: Bitboard(288239172245258244),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303424512),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(11379254888539095040),
                        white: Bitboard(205318037),
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
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 5,
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
            parent: &CHIGORIN_DEFENSE,
            code: Code {
                volume: Volume::D,
                category: Category(RangedU8::new_static::<4>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
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
                    from: Square::C2,
                    capture: None,
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C7,
                    capture: None,
                    to: Square::C6,
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
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B1,
                    capture: None,
                    to: Square::C3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::D6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::D3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D5,
                    capture: Some(Pawn),
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::D3,
                    capture: Some(Pawn),
                    to: Square::C4,
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
                    from: Square::C4,
                    capture: None,
                    to: Square::D3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B8,
                    capture: None,
                    to: Square::D7,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
                Castle {
                    king: Square::E8,
                    rook: Square::H8,
                },
                Normal {
                    role: Queen,
                    from: Square::D1,
                    capture: None,
                    to: Square::C2,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(63353868717712128),
                        knight: Bitboard(2286984188133376),
                        bishop: Bitboard(288239172245258244),
                        rook: Bitboard(2377900603251621921),
                        queen: Bitboard(576460752303424512),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(7919927398995329024),
                        white: Bitboard(138209125),
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
                castling_rights: Bitboard(0),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 5,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(10) {
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
            parent: &CHIGORIN_DEFENSE,
            code: Code {
                volume: Volume::D,
                category: Category(RangedU8::new_static::<4>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
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
                    from: Square::C2,
                    capture: None,
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C7,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B1,
                    capture: None,
                    to: Square::C3,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E2,
                    capture: None,
                    to: Square::E3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E6,
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
                    to: Square::D7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::D3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D5,
                    capture: Some(Pawn),
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::D3,
                    capture: Some(Pawn),
                    to: Square::C4,
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
                    from: Square::C4,
                    capture: None,
                    to: Square::D3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::D6,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
                Normal {
                    role: Bishop,
                    from: Square::C8,
                    capture: None,
                    to: Square::B7,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::A2,
                    capture: None,
                    to: Square::A3,
                    promotion: None,
                },
                Castle {
                    king: Square::E8,
                    rook: Square::H8,
                },
                Normal {
                    role: Queen,
                    from: Square::D1,
                    capture: None,
                    to: Square::C2,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(63353868717777408),
                        knight: Bitboard(2286984188133376),
                        bishop: Bitboard(571746046967812),
                        rook: Bitboard(2377900603251621921),
                        queen: Bitboard(576460752303424512),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(7632259972797038592),
                        white: Bitboard(138274405),
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
                castling_rights: Bitboard(0),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 2,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(11) {
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
