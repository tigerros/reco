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
# use reco::book::BOGO_INDIAN_DEFENSE;
assert_eq!(BOGO_INDIAN_DEFENSE.original_name(), "Bogo-Indian Defense");
```"#
)]
pub static BOGO_INDIAN_DEFENSE: Variation = Variation {
    name: "Bogo-Indian Defense",
    parent: None,
    variations: &[
        &EXCHANGE_VARIATION,
        &GRUNFELD_VARIATION,
        &HAITI_VARIATION,
        &MONTICELLI_TRAP,
        &NEW_ENGLAND_VARIATION,
        &NIMZOWITSCH_VARIATION,
        &RETREAT_VARIATION,
        &VITOLINS_VARIATION,
        &WADE_SMYSLOV_VARIATION,
    ],
    lines: &[Line {
        parent: &BOGO_INDIAN_DEFENSE,
        code: Code {
            volume: Volume::E,
            category: Category::new_static::<1>(),
        },
        moves: &[
            Normal {
                role: Pawn,
                from: Square::D2,
                capture: None,
                to: Square::D4,
                promotion: None,
            },
            Normal {
                role: Knight,
                from: Square::G8,
                capture: None,
                to: Square::F6,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::C2,
                capture: None,
                to: Square::C4,
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
                role: Knight,
                from: Square::G1,
                capture: None,
                to: Square::F3,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::F8,
                capture: None,
                to: Square::B4,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(67290111821280000),
                    knight: Bitboard(144150372450041858),
                    bishop: Bitboard(288230376185266212),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(11524482748056076288),
                    white: Bitboard(203486143),
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
    }],
};
pub mod exchange_variation;
pub use exchange_variation::EXCHANGE_VARIATION;
pub mod grunfeld_variation;
pub use grunfeld_variation::GRUNFELD_VARIATION;
pub mod haiti_variation;
pub use haiti_variation::HAITI_VARIATION;
pub mod monticelli_trap;
pub use monticelli_trap::MONTICELLI_TRAP;
pub mod new_england_variation;
pub use new_england_variation::NEW_ENGLAND_VARIATION;
pub mod nimzowitsch_variation;
pub use nimzowitsch_variation::NIMZOWITSCH_VARIATION;
pub mod retreat_variation;
pub use retreat_variation::RETREAT_VARIATION;
pub mod vitolins_variation;
pub use vitolins_variation::VITOLINS_VARIATION;
pub mod wade_smyslov_variation;
pub use wade_smyslov_variation::WADE_SMYSLOV_VARIATION;
