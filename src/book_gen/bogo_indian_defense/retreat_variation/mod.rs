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
# use reco::book::bogo_indian_defense::RETREAT_VARIATION;
assert_eq!(RETREAT_VARIATION.original_name(), "Bogo-Indian Defense: Retreat Variation");
```"#
)]
pub static RETREAT_VARIATION: Variation = Variation {
    name: "Retreat Variation",
    parent: Some(&super::BOGO_INDIAN_DEFENSE),
    variations: &[],
    lines: &[
        Line {
            parent: &RETREAT_VARIATION,
            code: Code {
                volume: Volume::E,
                category: Category(RangedU8::new_static::<1>()),
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
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::B4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::D2,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B4,
                    capture: None,
                    to: Square::E7,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(67290111821280000),
                        knight: Bitboard(144150372450041858),
                        bishop: Bitboard(292733975779084320),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(11528986347649892352),
                        white: Bitboard(203488187),
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
            parent: &RETREAT_VARIATION,
            code: Code {
                volume: Volume::E,
                category: Category(RangedU8::new_static::<1>()),
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
                    role: Pawn,
                    from: Square::G2,
                    capture: None,
                    to: Square::G3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::B4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::D2,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B4,
                    capture: None,
                    to: Square::E7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::G2,
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
                    from: Square::G1,
                    capture: None,
                    to: Square::F3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(65038346371511040),
                        knight: Bitboard(144150372450041858),
                        bishop: Bitboard(292733975779100672),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(11526734582195945472),
                        white: Bitboard(207682459),
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
                halfmoves: 1,
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
            parent: &RETREAT_VARIATION,
            code: Code {
                volume: Volume::E,
                category: Category(RangedU8::new_static::<1>()),
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
                    role: Pawn,
                    from: Square::G2,
                    capture: None,
                    to: Square::G3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::B4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::D2,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B4,
                    capture: None,
                    to: Square::E7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::G2,
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
                    from: Square::G1,
                    capture: None,
                    to: Square::F3,
                    promotion: None,
                },
                Castle {
                    king: Square::E8,
                    rook: Square::H8,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(65038346371511040),
                        knight: Bitboard(144150372450041858),
                        bishop: Bitboard(292733975779100672),
                        rook: Bitboard(2377900603251622017),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(4611686018427387920),
                    },
                    ByColor {
                        black: Bitboard(8067970068375404544),
                        white: Bitboard(207682459),
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
                castling_rights: Bitboard(129),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 2,
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
            parent: &RETREAT_VARIATION,
            code: Code {
                volume: Volume::E,
                category: Category(RangedU8::new_static::<1>()),
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
                    role: Pawn,
                    from: Square::G2,
                    capture: None,
                    to: Square::G3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::B4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::D2,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B4,
                    capture: None,
                    to: Square::E7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::G2,
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
                    from: Square::G1,
                    capture: None,
                    to: Square::F3,
                    promotion: None,
                },
                Castle {
                    king: Square::E8,
                    rook: Square::H8,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(65038346371511040),
                        knight: Bitboard(144150372450041858),
                        bishop: Bitboard(292733975779100672),
                        rook: Bitboard(2377900603251621921),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(8067970068375404544),
                        white: Bitboard(207682411),
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
                halfmoves: 3,
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
            parent: &RETREAT_VARIATION,
            code: Code {
                volume: Volume::E,
                category: Category(RangedU8::new_static::<1>()),
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
                    role: Pawn,
                    from: Square::G2,
                    capture: None,
                    to: Square::G3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::B4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::D2,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B4,
                    capture: None,
                    to: Square::E7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::G2,
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
                    from: Square::G1,
                    capture: None,
                    to: Square::F3,
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
                Normal {
                    role: Pawn,
                    from: Square::C7,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(63916844511179520),
                        knight: Bitboard(144150372450041858),
                        bishop: Bitboard(292733975779100672),
                        rook: Bitboard(2377900603251621921),
                        queen: Bitboard(576460752303424512),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(8066848566515073024),
                        white: Bitboard(207683427),
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
                halfmoves: 1,
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
            parent: &RETREAT_VARIATION,
            code: Code {
                volume: Volume::E,
                category: Category(RangedU8::new_static::<1>()),
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
                    role: Pawn,
                    from: Square::G2,
                    capture: None,
                    to: Square::G3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::B4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::D2,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B4,
                    capture: None,
                    to: Square::E7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::G2,
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
                Normal {
                    role: Pawn,
                    from: Square::C7,
                    capture: None,
                    to: Square::C6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(63916844511179520),
                        knight: Bitboard(2286984187871234),
                        bishop: Bitboard(292733975779100672),
                        rook: Bitboard(2377900603251621921),
                        queen: Bitboard(576460752303424512),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(7924985178252902400),
                        white: Bitboard(207683427),
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
                castling_rights: Bitboard(0),
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
        Line {
            parent: &RETREAT_VARIATION,
            code: Code {
                volume: Volume::E,
                category: Category(RangedU8::new_static::<1>()),
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
                    role: Pawn,
                    from: Square::G2,
                    capture: None,
                    to: Square::G3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::B4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::D2,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B4,
                    capture: None,
                    to: Square::E7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::G2,
                    promotion: None,
                },
                Castle {
                    king: Square::E8,
                    rook: Square::H8,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
                Normal {
                    role: Pawn,
                    from: Square::C7,
                    capture: None,
                    to: Square::C6,
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
                    to: Square::D7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::D2,
                    capture: None,
                    to: Square::F4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::B7,
                    capture: None,
                    to: Square::B6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B1,
                    capture: None,
                    to: Square::D2,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(63356093581013760),
                        knight: Bitboard(2286984187873280),
                        bishop: Bitboard(292733976315969536),
                        rook: Bitboard(2377900603251621921),
                        queen: Bitboard(576460752303424512),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(7924424427322736640),
                        white: Bitboard(744554337),
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
                halfmoves: 1,
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
            parent: &RETREAT_VARIATION,
            code: Code {
                volume: Volume::E,
                category: Category(RangedU8::new_static::<1>()),
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
                    role: Pawn,
                    from: Square::G2,
                    capture: None,
                    to: Square::G3,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::B4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::D2,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::B4,
                    capture: None,
                    to: Square::E7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::F1,
                    capture: None,
                    to: Square::G2,
                    promotion: None,
                },
                Castle {
                    king: Square::E8,
                    rook: Square::H8,
                },
                Castle {
                    king: Square::E1,
                    rook: Square::H1,
                },
                Normal {
                    role: Pawn,
                    from: Square::C7,
                    capture: None,
                    to: Square::C6,
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
                    to: Square::D7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::D2,
                    capture: None,
                    to: Square::F4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::B7,
                    capture: None,
                    to: Square::B6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B1,
                    capture: None,
                    to: Square::D2,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C8,
                    capture: None,
                    to: Square::A6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(63356093581013760),
                        knight: Bitboard(2286984187873280),
                        bishop: Bitboard(4504699675885568),
                        rook: Bitboard(2377900603251621921),
                        queen: Bitboard(576460752303424512),
                        king: Bitboard(4611686018427387968),
                    },
                    ByColor {
                        black: Bitboard(7636195150682652672),
                        white: Bitboard(744554337),
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
