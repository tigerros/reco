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
# use reco::book::sicilian_defense::najdorf_variation::POISONED_PAWN_ACCEPTED;
assert_eq!(POISONED_PAWN_ACCEPTED.original_name(), "Sicilian Defense: Najdorf Variation, Poisoned Pawn Accepted");
```"#
)]
pub static POISONED_PAWN_ACCEPTED: Variation = Variation {
    name: "Poisoned Pawn Accepted",
    parent: Some(&super::NAJDORF_VARIATION),
    variations: &[],
    lines: &[Line {
        parent: &POISONED_PAWN_ACCEPTED,
        code: Code {
            volume: Volume::B,
            category: Category::new_static::<97>(),
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
                from: C7,
                capture: None,
                to: C5,
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
                from: D7,
                capture: None,
                to: D6,
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
                role: Pawn,
                from: C5,
                capture: Some(Pawn),
                to: D4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F3,
                capture: Some(Pawn),
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
                role: Knight,
                from: B1,
                capture: None,
                to: C3,
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
                role: Bishop,
                from: C1,
                capture: None,
                to: G5,
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
                from: F2,
                capture: None,
                to: F4,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D8,
                capture: None,
                to: B6,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D1,
                capture: None,
                to: D2,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: B6,
                capture: Some(Pawn),
                to: B2,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: A1,
                capture: None,
                to: B1,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: B2,
                capture: None,
                to: A3,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(63640833332659456),
                    knight: Bitboard(144150372582424576),
                    bishop: Bitboard(2594073660243312672),
                    rook: Bitboard(9295429630892703874),
                    queen: Bitboard(67584),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(13250215725840269312),
                    white: Bitboard(275817745842),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703872),
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
    }],
};
