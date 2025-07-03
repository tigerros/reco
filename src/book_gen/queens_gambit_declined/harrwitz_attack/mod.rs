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
# use reco::book::queens_gambit_declined::HARRWITZ_ATTACK;
assert_eq!(HARRWITZ_ATTACK.original_name(), "Queen's Gambit Declined: Harrwitz Attack");
```"#
)]
pub static HARRWITZ_ATTACK: Variation = Variation {
    name: "Harrwitz Attack",
    parent: Some(&super::QUEENS_GAMBIT_DECLINED),
    variations: &[
        &FIANCHETTO_DEFENSE,
        &MAIN_LINE,
        &ORTHODOX_DEFENSE,
        &TWO_KNIGHTS_DEFENSE,
    ],
    lines: &[
        Line {
            parent: &HARRWITZ_ATTACK,
            code: Code {
                volume: Volume::D,
                category: Category(RangedU8::new_static::<3>()),
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
                    from: Square::E7,
                    capture: None,
                    to: Square::E6,
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
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::F4,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(65038346367333120),
                        knight: Bitboard(144150372448206912),
                        bishop: Bitboard(2594073385902276640),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(13828073991782268928),
                        white: Bitboard(738522105),
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
                halfmoves: 3,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(4) {
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
            parent: &HARRWITZ_ATTACK,
            code: Code {
                volume: Volume::D,
                category: Category(RangedU8::new_static::<3>()),
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
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
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
                    role: Pawn,
                    from: Square::D7,
                    capture: None,
                    to: Square::D5,
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
                    to: Square::E7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::F4,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(65038346367333120),
                        knight: Bitboard(144150372450304000),
                        bishop: Bitboard(292733976315953184),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(11526734582195945472),
                        white: Bitboard(740619193),
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
                halfmoves: 3,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) {
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
            parent: &HARRWITZ_ATTACK,
            code: Code {
                volume: Volume::D,
                category: Category(RangedU8::new_static::<3>()),
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
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
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
                    role: Pawn,
                    from: Square::D7,
                    capture: None,
                    to: Square::D5,
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
                    to: Square::E7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::F4,
                    promotion: None,
                },
                Castle {
                    king: Square::E8,
                    rook: Square::H8,
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
                    from: Square::C7,
                    capture: None,
                    to: Square::C5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D4,
                    capture: Some(Pawn),
                    to: Square::C5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::E7,
                    capture: Some(Pawn),
                    to: Square::C5,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: Square::D1,
                    capture: None,
                    to: Square::C2,
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
                    role: Rook,
                    from: Square::A1,
                    capture: None,
                    to: Square::D1,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: Square::D8,
                    capture: None,
                    to: Square::A5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::A2,
                    capture: None,
                    to: Square::A3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(63912446327382528),
                        knight: Bitboard(39582420959232),
                        bishop: Bitboard(288230393868451872),
                        rook: Bitboard(2377900603251622024),
                        queen: Bitboard(4294968320),
                        king: Bitboard(4611686018427387920),
                    },
                    ByColor {
                        black: Bitboard(7341769047983259648),
                        white: Bitboard(607512248),
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
                castling_rights: Bitboard(128),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
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
            parent: &HARRWITZ_ATTACK,
            code: Code {
                volume: Volume::D,
                category: Category(RangedU8::new_static::<3>()),
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
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
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
                    role: Pawn,
                    from: Square::D7,
                    capture: None,
                    to: Square::D5,
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
                    to: Square::E7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::F4,
                    promotion: None,
                },
                Castle {
                    king: Square::E8,
                    rook: Square::H8,
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
                    from: Square::C7,
                    capture: None,
                    to: Square::C5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D4,
                    capture: Some(Pawn),
                    to: Square::C5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::E7,
                    capture: Some(Pawn),
                    to: Square::C5,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: Square::D1,
                    capture: None,
                    to: Square::C2,
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
                    role: Pawn,
                    from: Square::A2,
                    capture: None,
                    to: Square::A3,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: Square::D8,
                    capture: None,
                    to: Square::A5,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::A1,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(63912446327382528),
                        knight: Bitboard(39582420959232),
                        bishop: Bitboard(288230393868451872),
                        rook: Bitboard(2377900603251622024),
                        queen: Bitboard(4294968320),
                        king: Bitboard(4611686018427387908),
                    },
                    ByColor {
                        black: Bitboard(7341769047983259648),
                        white: Bitboard(607512236),
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
    ],
};
pub mod fianchetto_defense;
pub use fianchetto_defense::FIANCHETTO_DEFENSE;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod orthodox_defense;
pub use orthodox_defense::ORTHODOX_DEFENSE;
pub mod two_knights_defense;
pub use two_knights_defense::TWO_KNIGHTS_DEFENSE;
