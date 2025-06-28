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
# use reco::book::italian_game::GIUOCO_PIANO;
assert_eq!(GIUOCO_PIANO.original_name(), "Italian Game: Giuoco Piano");
```"#
)]
pub static GIUOCO_PIANO: Variation = Variation {
    name: "Giuoco Piano",
    parent: Some(&super::ITALIAN_GAME),
    variations: &[
        &AITKEN_VARIATION,
        &BERNSTEIN_VARIATION,
        &CRACOW_VARIATION,
        &GRECOS_ATTACK,
        &HOLZHAUSEN_ATTACK,
        &KRAUSE_VARIATION,
        &ROSENTRETER_VARIATION,
        &STEINITZ_VARIATION,
        &THERKATZ_HERZOG_VARIATION,
    ],
    lines: &[Line {
        parent: &GIUOCO_PIANO,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<5>(),
        },
        moves: &[
            Normal {
                role: Pawn,
                from: Square::E2,
                capture: None,
                to: Square::E4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: Square::E7,
                capture: None,
                to: Square::E5,
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
                role: Knight,
                from: Square::B8,
                capture: None,
                to: Square::C6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::F1,
                capture: None,
                to: Square::C4,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: Square::F8,
                capture: None,
                to: Square::C5,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(67272588421820160),
                    knight: Bitboard(4611690416475996162),
                    bishop: Bitboard(288230393398689796),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(15992005285761777664),
                    white: Bitboard(337702815),
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
            halfmoves: 4,
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
pub mod aitken_variation;
pub use aitken_variation::AITKEN_VARIATION;
pub mod bernstein_variation;
pub use bernstein_variation::BERNSTEIN_VARIATION;
pub mod cracow_variation;
pub use cracow_variation::CRACOW_VARIATION;
pub mod grecos_attack;
pub use grecos_attack::GRECOS_ATTACK;
pub mod holzhausen_attack;
pub use holzhausen_attack::HOLZHAUSEN_ATTACK;
pub mod krause_variation;
pub use krause_variation::KRAUSE_VARIATION;
pub mod rosentreter_variation;
pub use rosentreter_variation::ROSENTRETER_VARIATION;
pub mod steinitz_variation;
pub use steinitz_variation::STEINITZ_VARIATION;
pub mod therkatz_herzog_variation;
pub use therkatz_herzog_variation::THERKATZ_HERZOG_VARIATION;
