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
# use reco::book::BENKO_GAMBIT;
assert_eq!(BENKO_GAMBIT.original_name(), "Benko Gambit");
```"#
)]
pub static BENKO_GAMBIT: Variation = Variation {
    name: "Benko Gambit",
    parent: None,
    variations: &[
        &FIANCHETTO_VARIATION,
        &MUTKIN_COUNTERGAMBIT,
        &ND2_VARIATION,
        &NESCAFE_FRAPPE_ATTACK,
        &ZAITSEV_SYSTEM,
        &ZAITSEV_VARIATION,
    ],
    lines: &[
        Line {
            parent: &BENKO_GAMBIT,
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<57>(),
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
                    to: C5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D4,
                    capture: None,
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: B7,
                    capture: None,
                    to: B5,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(70087329397666560),
                        knight: Bitboard(144150372447944770),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(13833122940587081728),
                        white: Bitboard(34426909695),
                    },
                ),
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
            parent: &BENKO_GAMBIT,
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<59>(),
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
                    to: C5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D4,
                    capture: None,
                    to: D5,
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
                    role: Pawn,
                    from: C4,
                    capture: Some(Pawn),
                    to: B5,
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
                    from: B5,
                    capture: Some(Pawn),
                    to: A6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: G7,
                    capture: None,
                    to: G6,
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
                    from: C8,
                    capture: Some(Pawn),
                    to: A6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E2,
                    capture: None,
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Bishop,
                    from: A6,
                    capture: Some(Bishop),
                    to: F1,
                    promotion: None,
                },
                Normal {
                    role: King,
                    from: E1,
                    capture: Some(Bishop),
                    to: F1,
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
                    from: G2,
                    capture: None,
                    to: G3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(49618812550554368),
                        knight: Bitboard(144150372448206912),
                        bishop: Bitboard(2305843009213693956),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606847008),
                    },
                    ByColor {
                        black: Bitboard(13524424047382757376),
                        white: Bitboard(34632672237),
                    },
                ),
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
pub mod fianchetto_variation;
pub use fianchetto_variation::FIANCHETTO_VARIATION;
pub mod mutkin_countergambit;
pub use mutkin_countergambit::MUTKIN_COUNTERGAMBIT;
pub mod nd2_variation;
pub use nd2_variation::ND2_VARIATION;
pub mod nescafe_frappe_attack;
pub use nescafe_frappe_attack::NESCAFE_FRAPPE_ATTACK;
pub mod zaitsev_system;
pub use zaitsev_system::ZAITSEV_SYSTEM;
pub mod zaitsev_variation;
pub use zaitsev_variation::ZAITSEV_VARIATION;
