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
# use reco::book::kings_gambit_accepted::bishops_gambit::MC_DONNELL_ATTACK;
assert_eq!(MC_DONNELL_ATTACK.original_name(), "King's Gambit Accepted: Bishop's Gambit, McDonnell Attack");
```"#
)]
pub static MC_DONNELL_ATTACK: Variation = Variation {
    name: "McDonnell Attack",
    parent: Some(&super::BISHOPS_GAMBIT),
    variations: &[],
    lines: &[
        Line {
            parent: &MC_DONNELL_ATTACK,
            code: Code {
                volume: Volume::C,
                category: Category::new_static::<33>(),
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
                    from: E7,
                    capture: None,
                    to: E5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: F2,
                    capture: None,
                    to: F4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E5,
                    capture: Some(Pawn),
                    to: F4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: F1,
                    capture: None,
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D8,
                    capture: None,
                    to: H4,
                    promotion: None,
                },
                Normal {
                    role: King,
                    from: E1,
                    capture: None,
                    to: F1,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: G7,
                    capture: None,
                    to: G5,
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
                    role: Bishop,
                    from: F8,
                    capture: None,
                    to: G7,
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
                    role: Knight,
                    from: G8,
                    capture: None,
                    to: E7,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: G2,
                    capture: None,
                    to: G3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(49258396746024704),
                        knight: Bitboard(148618787703488576),
                        bishop: Bitboard(306244774728302596),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(2147483656),
                        king: Bitboard(1152921504606847008),
                    },
                    ByColor {
                        black: Bitboard(10952473096350597120),
                        white: Bitboard(474253293),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
                castling_rights: Bitboard(9295429630892703744),
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
            parent: &MC_DONNELL_ATTACK,
            code: Code {
                volume: Volume::C,
                category: Category::new_static::<33>(),
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
                    from: E7,
                    capture: None,
                    to: E5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: F2,
                    capture: None,
                    to: F4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E5,
                    capture: Some(Pawn),
                    to: F4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: F1,
                    capture: None,
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D8,
                    capture: None,
                    to: H4,
                    promotion: None,
                },
                Normal {
                    role: King,
                    from: E1,
                    capture: None,
                    to: F1,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: G7,
                    capture: None,
                    to: G5,
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
                    role: Bishop,
                    from: F8,
                    capture: None,
                    to: G7,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: G2,
                    capture: None,
                    to: G3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(49258396611809024),
                        knight: Bitboard(4755801206503505984),
                        bishop: Bitboard(306244774728302596),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(2147483656),
                        king: Bitboard(1152921504606847008),
                    },
                    ByColor {
                        black: Bitboard(15559655515150614528),
                        white: Bitboard(340037613),
                    },
                ),
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
    ],
};
