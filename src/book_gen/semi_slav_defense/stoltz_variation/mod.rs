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
    clippy::enum_glob_use,
    reason = "there's 64 variants in this enum, importing them all is stupid"
)]
#[allow(
    unused_imports,
    reason = "because the code is generated, we don't know if it's going to be used"
)]
use shakmaty::Square::*;
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
# use reco::book::semi_slav_defense::STOLTZ_VARIATION;
assert_eq!(STOLTZ_VARIATION.original_name(), "Semi-Slav Defense: Stoltz Variation");
```"#
)]
pub static STOLTZ_VARIATION: Variation = Variation {
    name: "Stoltz Variation",
    parent: Some(&super::SEMI_SLAV_DEFENSE),
    variations: &[&CENTER_VARIATION, &SHABALOV_ATTACK],
    lines: &[
        Line {
            parent: &STOLTZ_VARIATION,
            code: Code {
                volume: Volume::D,
                category: Category::new_static::<45>(),
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
                    from: D7,
                    capture: None,
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C2,
                    capture: None,
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C7,
                    capture: None,
                    to: C6,
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
                    from: G8,
                    capture: None,
                    to: F6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E2,
                    capture: None,
                    to: E3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E7,
                    capture: None,
                    to: E6,
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
                    role: Knight,
                    from: B8,
                    capture: None,
                    to: D7,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D1,
                    capture: None,
                    to: C2,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63916844508046080),
                        knight: Bitboard(2286984188133376),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303424512),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(13685089101659766784),
                        white: Bitboard(204793781),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 3,
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
            parent: &STOLTZ_VARIATION,
            code: Code {
                volume: Volume::D,
                category: Category::new_static::<45>(),
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
                    from: D7,
                    capture: None,
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C2,
                    capture: None,
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C7,
                    capture: None,
                    to: C6,
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
                    from: G8,
                    capture: None,
                    to: F6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E2,
                    capture: None,
                    to: E3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E7,
                    capture: None,
                    to: E6,
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
                    role: Knight,
                    from: B8,
                    capture: None,
                    to: D7,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D1,
                    capture: None,
                    to: C2,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: B7,
                    capture: None,
                    to: B6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: B2,
                    capture: None,
                    to: B3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63356093578010880),
                        knight: Bitboard(2286984188133376),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303424512),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(13684528350729601024),
                        white: Bitboard(204924341),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
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
            parent: &STOLTZ_VARIATION,
            code: Code {
                volume: Volume::D,
                category: Category::new_static::<45>(),
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
                    from: D7,
                    capture: None,
                    to: D5,
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
                    role: Knight,
                    from: G8,
                    capture: None,
                    to: F6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C2,
                    capture: None,
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C7,
                    capture: None,
                    to: C6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E2,
                    capture: None,
                    to: E3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E7,
                    capture: None,
                    to: E6,
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
                    to: D7,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D1,
                    capture: None,
                    to: C2,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: F8,
                    capture: None,
                    to: D6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: B2,
                    capture: None,
                    to: B3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63916844508176640),
                        knight: Bitboard(2286984188133376),
                        bishop: Bitboard(288239172244733988),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303424512),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(11379254888539095040),
                        white: Bitboard(204924341),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
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
            parent: &STOLTZ_VARIATION,
            code: Code {
                volume: Volume::D,
                category: Category::new_static::<45>(),
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
                    from: D7,
                    capture: None,
                    to: D5,
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
                    role: Knight,
                    from: G8,
                    capture: None,
                    to: F6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C2,
                    capture: None,
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C7,
                    capture: None,
                    to: C6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E2,
                    capture: None,
                    to: E3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E7,
                    capture: None,
                    to: E6,
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
                    to: D7,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D1,
                    capture: None,
                    to: C2,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: F8,
                    capture: None,
                    to: D6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: B2,
                    capture: None,
                    to: B3,
                    promotion: None,
                },
                Castle { king: E8, rook: H8 },
                Normal {
                    role: Bishop,
                    from: C1,
                    capture: None,
                    to: B2,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63916844508176640),
                        knight: Bitboard(2286984188133376),
                        bishop: Bitboard(288239172244734496),
                        rook: Bitboard(2377900603251622017),
                        queen: Bitboard(576460752303424512),
                        king: Bitboard(4611686018427387920),
                    },
                    ByColor {
                        black: Bitboard(7920490374718554112),
                        white: Bitboard(204924849),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(129),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 2,
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
    ],
};
pub mod center_variation;
pub use center_variation::CENTER_VARIATION;
pub mod shabalov_attack;
pub use shabalov_attack::SHABALOV_ATTACK;
