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
# use reco::book::semi_slav_defense::meran_variation::SOZIN_VARIATION;
assert_eq!(SOZIN_VARIATION.original_name(), "Semi-Slav Defense: Meran Variation, Sozin Variation");
```"#
)]
pub static SOZIN_VARIATION: Variation = Variation {
    name: "Sozin Variation",
    parent: Some(&super::MERAN_VARIATION),
    variations: &[],
    lines: &[
        Line {
            parent: &SOZIN_VARIATION,
            code: Code {
                volume: Volume::D,
                category: Category::new_static::<49>(),
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
                    role: Pawn,
                    from: E7,
                    capture: None,
                    to: E6,
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
                    role: Knight,
                    from: B8,
                    capture: None,
                    to: D7,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: F1,
                    capture: None,
                    to: D3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D5,
                    capture: Some(Pawn),
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: D3,
                    capture: Some(Pawn),
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: B7,
                    capture: None,
                    to: B5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: C4,
                    capture: None,
                    to: D3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: A7,
                    capture: None,
                    to: A6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E3,
                    capture: None,
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C6,
                    capture: None,
                    to: C5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E4,
                    capture: None,
                    to: E5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C5,
                    capture: Some(Pawn),
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: C3,
                    capture: Some(Pawn),
                    to: B5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: D7,
                    capture: Some(Pawn),
                    to: E5,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63069086615134976),
                        knight: Bitboard(35261683597312),
                        bishop: Bitboard(2594073385365929988),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(13681989612875022336),
                        white: Bitboard(8592614301),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(12) {
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
            parent: &SOZIN_VARIATION,
            code: Code {
                volume: Volume::D,
                category: Category::new_static::<49>(),
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
                    role: Knight,
                    from: B1,
                    capture: None,
                    to: C3,
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
                    role: Pawn,
                    from: E2,
                    capture: None,
                    to: E3,
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
                    role: Bishop,
                    from: F1,
                    capture: None,
                    to: D3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D5,
                    capture: Some(Pawn),
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: D3,
                    capture: Some(Pawn),
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: B7,
                    capture: None,
                    to: B5,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: C4,
                    capture: None,
                    to: D3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: A7,
                    capture: None,
                    to: A6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E3,
                    capture: None,
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C6,
                    capture: None,
                    to: C5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E4,
                    capture: None,
                    to: E5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C5,
                    capture: Some(Pawn),
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: C3,
                    capture: Some(Pawn),
                    to: B5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: D7,
                    capture: Some(Pawn),
                    to: E5,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: F3,
                    capture: Some(Knight),
                    to: E5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: A6,
                    capture: Some(Knight),
                    to: B5,
                    promotion: None,
                },
                Castle { king: E1, rook: H1 },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(63067995693441792),
                        knight: Bitboard(35253091565568),
                        bishop: Bitboard(2594073385365929988),
                        rook: Bitboard(9295429630892703777),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606847040),
                    },
                    ByColor {
                        black: Bitboard(13681988453233852416),
                        white: Bitboard(68720059245),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703744),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 1,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(13) {
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
