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
# use reco::book::sicilian_defense::CLOSED;
assert_eq!(CLOSED.original_name(), "Sicilian Defense: Closed");
```"#
)]
pub static CLOSED: Variation = Variation {
    name: "Closed",
    parent: Some(&super::SICILIAN_DEFENSE),
    variations: &[
        &ANTI_SVESHNIKOV_VARIATION,
        &BOTVINNIK_DEFENSE,
        &CHAMELEON_VARIATION,
        &FIANCHETTO_VARIATION,
        &GROB_ATTACK,
        &KORCHNOI_DEFENSE,
        &SMYSLOV_VARIATION,
        &TRADITIONAL,
    ],
    lines: &[
        Line {
            parent: &CLOSED,
            code: Code {
                volume: Volume::B,
                category: Category(RangedU8::new_static::<2>()),
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
                    from: Square::C7,
                    capture: None,
                    to: Square::C5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B1,
                    capture: None,
                    to: Square::C3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(70650236602740480),
                        knight: Bitboard(4755801206503505984),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18445336716005867520),
                        white: Bitboard(268759037),
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
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
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
            parent: &CLOSED,
            code: Code {
                volume: Volume::B,
                category: Category(RangedU8::new_static::<2>()),
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
                    from: Square::C7,
                    capture: None,
                    to: Square::C5,
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
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(66164229161414400),
                        knight: Bitboard(4755801206503505984),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18440850708564541440),
                        white: Bitboard(268759037),
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
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) {
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
            parent: &CLOSED,
            code: Code {
                volume: Volume::B,
                category: Category(RangedU8::new_static::<2>()),
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
                    from: Square::C7,
                    capture: None,
                    to: Square::C5,
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
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(66164229165592320),
                        knight: Bitboard(4755801206503505984),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18440850708564541440),
                        white: Bitboard(272936957),
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
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) {
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
            parent: &CLOSED,
            code: Code {
                volume: Volume::B,
                category: Category(RangedU8::new_static::<2>()),
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
                    from: Square::C7,
                    capture: None,
                    to: Square::C5,
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
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
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
                    role: Pawn,
                    from: Square::G7,
                    capture: None,
                    to: Square::G6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(52706206841614080),
                        knight: Bitboard(4611690416474161216),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18283281896211218432),
                        white: Bitboard(272936957),
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
                halfmoves: 0,
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
            parent: &CLOSED,
            code: Code {
                volume: Volume::B,
                category: Category(RangedU8::new_static::<2>()),
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
                    from: Square::C7,
                    capture: None,
                    to: Square::C5,
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
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
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
                    role: Pawn,
                    from: Square::G7,
                    capture: None,
                    to: Square::G6,
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
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::G7,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(52706206841614080),
                        knight: Bitboard(4611690416474161216),
                        bishop: Bitboard(306244774661210116),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(15995453285507006464),
                        white: Bitboard(272953309),
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
                halfmoves: 2,
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
            parent: &CLOSED,
            code: Code {
                volume: Volume::B,
                category: Category(RangedU8::new_static::<2>()),
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
                    from: Square::C7,
                    capture: None,
                    to: Square::C5,
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
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
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
                    role: Pawn,
                    from: Square::G7,
                    capture: None,
                    to: Square::G6,
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
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::G7,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D7,
                    capture: None,
                    to: Square::D6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(50463203121473280),
                        knight: Bitboard(4611690416474161216),
                        bishop: Bitboard(306244774661210116),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(15993210281786343424),
                        white: Bitboard(273475549),
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
            parent: &CLOSED,
            code: Code {
                volume: Volume::B,
                category: Category(RangedU8::new_static::<2>()),
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
                    from: Square::C7,
                    capture: None,
                    to: Square::C5,
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
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
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
                    role: Pawn,
                    from: Square::G7,
                    capture: None,
                    to: Square::G6,
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
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::G7,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D7,
                    capture: None,
                    to: Square::D6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::F2,
                    capture: None,
                    to: Square::F4,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(50463203658336000),
                        knight: Bitboard(4611690416474161216),
                        bishop: Bitboard(306244774661210116),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(15993210281786343424),
                        white: Bitboard(810338269),
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
            parent: &CLOSED,
            code: Code {
                volume: Volume::B,
                category: Category(RangedU8::new_static::<2>()),
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
                    from: Square::C7,
                    capture: None,
                    to: Square::C5,
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
                    from: Square::B8,
                    capture: None,
                    to: Square::C6,
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
                    role: Pawn,
                    from: Square::G7,
                    capture: None,
                    to: Square::G6,
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
                    role: Bishop,
                    from: Square::F8,
                    capture: None,
                    to: Square::G7,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D7,
                    capture: None,
                    to: Square::D6,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: Square::C1,
                    capture: None,
                    to: Square::E3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(50463203121473280),
                        knight: Bitboard(4611690416474161216),
                        bishop: Bitboard(306244774662258688),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(15993210281786343424),
                        white: Bitboard(274524121),
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
    ],
};
pub mod anti_sveshnikov_variation;
pub use anti_sveshnikov_variation::ANTI_SVESHNIKOV_VARIATION;
pub mod botvinnik_defense;
pub use botvinnik_defense::BOTVINNIK_DEFENSE;
pub mod chameleon_variation;
pub use chameleon_variation::CHAMELEON_VARIATION;
pub mod fianchetto_variation;
pub use fianchetto_variation::FIANCHETTO_VARIATION;
pub mod grob_attack;
pub use grob_attack::GROB_ATTACK;
pub mod korchnoi_defense;
pub use korchnoi_defense::KORCHNOI_DEFENSE;
pub mod smyslov_variation;
pub use smyslov_variation::SMYSLOV_VARIATION;
pub mod traditional;
pub use traditional::TRADITIONAL;
