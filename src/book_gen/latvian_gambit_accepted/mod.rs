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
# use reco::book::LATVIAN_GAMBIT_ACCEPTED;
assert_eq!(LATVIAN_GAMBIT_ACCEPTED.original_name(), "Latvian Gambit Accepted");
```"#
)]
pub static LATVIAN_GAMBIT_ACCEPTED: Variation = Variation {
    name: "Latvian Gambit Accepted",
    parent: None,
    variations: &[
        &BILGUER_VARIATION,
        &FOLTYS_LEONHARDT_VARIATION,
        &FOLTYS_VARIATION,
        &LEONHARDT_VARIATION,
        &MAIN_LINE,
        &NIMZOWITSCH_ATTACK,
        &BRONSTEIN_ATTACK,
        &BRONSTEIN_GAMBIT,
    ],
    lines: &[Line {
        parent: &LATVIAN_GAMBIT_ACCEPTED,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<40>(),
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
                role: Knight,
                from: G1,
                capture: None,
                to: F3,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: F7,
                capture: None,
                to: F5,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E4,
                capture: Some(Pawn),
                to: F5,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(58265526337597184),
                    knight: Bitboard(4755801206505340930),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18432951868570206208),
                    white: Bitboard(137441111999),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) {
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
pub mod bilguer_variation;
pub use bilguer_variation::BILGUER_VARIATION;
pub mod foltys_leonhardt_variation;
pub use foltys_leonhardt_variation::FOLTYS_LEONHARDT_VARIATION;
pub mod foltys_variation;
pub use foltys_variation::FOLTYS_VARIATION;
pub mod leonhardt_variation;
pub use leonhardt_variation::LEONHARDT_VARIATION;
pub mod main_line;
pub use main_line::MAIN_LINE;
pub mod nimzowitsch_attack;
pub use nimzowitsch_attack::NIMZOWITSCH_ATTACK;
pub mod bronstein_attack;
pub use bronstein_attack::BRONSTEIN_ATTACK;
pub mod bronstein_gambit;
pub use bronstein_gambit::BRONSTEIN_GAMBIT;
