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
# use reco::book::semi_slav_defense::noteboom_variation::ABRAHAMS_VARIATION;
assert_eq!(ABRAHAMS_VARIATION.original_name(), "Semi-Slav Defense: Noteboom Variation, Abrahams Variation");
```"#
)]
pub static ABRAHAMS_VARIATION: Variation = Variation {
    name: "Abrahams Variation",
    parent: Some(&super::NOTEBOOM_VARIATION),
    variations: &[],
    lines: &[Line {
        parent: &ABRAHAMS_VARIATION,
        code: Code {
            volume: Volume::D,
            category: Category::new_static::<31>(),
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
                role: Pawn,
                from: D5,
                capture: Some(Pawn),
                to: C4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: A2,
                capture: None,
                to: A4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: F8,
                capture: None,
                to: B4,
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
                role: Bishop,
                from: B4,
                capture: Some(Knight),
                to: C3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: B2,
                capture: Some(Bishop),
                to: C3,
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
                from: A4,
                capture: Some(Pawn),
                to: B5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: C6,
                capture: Some(Pawn),
                to: B5,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(63349470738571264),
                    knight: Bitboard(4755801206505340928),
                    bishop: Bitboard(288230376151711780),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(16132192941060915200),
                    white: Bitboard(137683133),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(9295429630892703873),
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
    }],
};
