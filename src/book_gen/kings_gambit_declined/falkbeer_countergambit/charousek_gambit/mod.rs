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
# use reco::book::kings_gambit_declined::falkbeer_countergambit::CHAROUSEK_GAMBIT;
assert_eq!(CHAROUSEK_GAMBIT.original_name(), "King's Gambit Declined: Falkbeer Countergambit, Charousek Gambit");
```"#
)]
pub static CHAROUSEK_GAMBIT: Variation = Variation {
    name: "Charousek Gambit",
    parent: Some(&super::FALKBEER_COUNTERGAMBIT),
    variations: &[&KERES_VARIATION, &MAIN_LINE, &MORPHY_DEFENSE, &OLD_LINE],
    lines: &[
        Line {
            parent: &CHAROUSEK_GAMBIT,
            code: Code {
                volume: Volume::C,
                category: Category::new_static::<31>(),
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
                    from: D7,
                    capture: None,
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E4,
                    capture: Some(Pawn),
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E5,
                    capture: None,
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D2,
                    capture: None,
                    to: D3,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(65020754785781504),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18439707199560220672),
                        white: Bitboard(34897184767),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: Black,
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
            parent: &CHAROUSEK_GAMBIT,
            code: Code {
                volume: Volume::C,
                category: Category::new_static::<32>(),
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
                    from: D7,
                    capture: None,
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E4,
                    capture: Some(Pawn),
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E5,
                    capture: None,
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D2,
                    capture: None,
                    to: D3,
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
                    from: D3,
                    capture: Some(Pawn),
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: F6,
                    capture: Some(Pawn),
                    to: E4,
                    promotion: None,
                },
                Normal {
                    role: Queen,
                    from: D1,
                    capture: None,
                    to: E2,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(65020754516821760),
                        knight: Bitboard(144115188344291394),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303427584),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(13828021181132832768),
                        white: Bitboard(34896664567),
                    },
                ),
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
pub mod keres_variation;
pub use keres_variation::KERES_VARIATION;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod morphy_defense;
pub use morphy_defense::MORPHY_DEFENSE;
pub mod old_line;
pub use old_line::OLD_LINE;
