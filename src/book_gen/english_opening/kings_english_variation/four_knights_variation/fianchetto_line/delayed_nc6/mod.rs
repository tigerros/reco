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
# use reco::book::english_opening::kings_english_variation::four_knights_variation::fianchetto_line::DELAYED_NC6;
assert_eq!(DELAYED_NC6.original_name(), "English Opening: King's English Variation, Four Knights Variation, Fianchetto Line, Delayed .. Nc6");
```"#
)]
pub static DELAYED_NC6: Variation = Variation {
    name: "Delayed .. Nc6",
    parent: Some(&super::FIANCHETTO_LINE),
    variations: &[],
    lines: &[Line {
        parent: &DELAYED_NC6,
        code: Code {
            volume: Volume::A,
            category: Category::new_static::<29>(),
        },
        moves: &[
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
                to: E5,
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
                from: G2,
                capture: None,
                to: G3,
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
                role: Bishop,
                from: F1,
                capture: None,
                to: G2,
                promotion: None,
            },
            Castle { king: E8, rook: H8 },
            Normal {
                role: Knight,
                from: G1,
                capture: None,
                to: F3,
                promotion: None,
            },
            Normal {
                role: Rook,
                from: F8,
                capture: None,
                to: E8,
                promotion: None,
            },
            Castle { king: E1, rook: H1 },
            Normal {
                role: Pawn,
                from: E5,
                capture: None,
                to: E4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: F3,
                capture: None,
                to: D4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: B8,
                capture: None,
                to: C6,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67272519773633280),
                    knight: Bitboard(39582553079808),
                    bishop: Bitboard(288230376185282564),
                    rook: Bitboard(1224979098644774945),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(4611686018427387968),
                },
                ByColor {
                    black: Bitboard(6768668347681734656),
                    white: Bitboard(205847405),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: White,
            castling_rights: Bitboard(0),
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
