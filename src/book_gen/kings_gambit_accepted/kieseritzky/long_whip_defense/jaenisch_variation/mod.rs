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
# use reco::book::kings_gambit_accepted::kieseritzky::long_whip_defense::JAENISCH_VARIATION;
assert_eq!(JAENISCH_VARIATION.original_name(), "King's Gambit Accepted: Kieseritzky, Long Whip Defense, Jaenisch Variation");
```"#
)]
pub static JAENISCH_VARIATION: Variation = Variation {
    name: "Jaenisch Variation",
    parent: Some(&super::LONG_WHIP_DEFENSE),
    variations: &[],
    lines: &[Line {
        parent: &JAENISCH_VARIATION,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<39>(),
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
                role: Knight,
                from: G1,
                capture: None,
                to: F3,
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
                role: Pawn,
                from: H2,
                capture: None,
                to: H4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: G5,
                capture: None,
                to: G4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F3,
                capture: None,
                to: E5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: H7,
                capture: None,
                to: H5,
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
                role: Rook,
                from: H8,
                capture: None,
                to: H7,
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
                role: Bishop,
                from: F8,
                capture: None,
                to: H6,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: B1,
                capture: None,
                to: C3,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(13229877821982464),
                    knight: Bitboard(4755801275222982656),
                    bishop: Bitboard(288371113707175940),
                    rook: Bitboard(108086391056892033),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(6894870843382300672),
                    white: Bitboard(71337002909),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(72057594037928065),
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
    }],
};
