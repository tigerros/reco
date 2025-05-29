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
# use reco::book::sicilian_defense::WING_GAMBIT;
assert_eq!(WING_GAMBIT.original_name(), "Sicilian Defense: Wing Gambit");
```"#
)]
pub static WING_GAMBIT: Variation = Variation {
    name: "Wing Gambit",
    parent: Some(&super::SICILIAN_DEFENSE),
    variations: &[
        &SANTASIERE_VARIATION,
        &MARSHALL_VARIATION,
        &NANU_GAMBIT,
        &DEFERRED_VARIATION,
        &ROMANIAN_DEFENSE,
        &MARIENBAD_VARIATION,
        &CARLSBAD_VARIATION,
        &ABRAHAMS_VARIATION,
    ],
    lines: &[Line {
        parent: &WING_GAMBIT,
        code: Code {
            volume: Volume::B,
            category: Category::new_static::<20>(),
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
                role: Pawn,
                from: B2,
                capture: None,
                to: B4,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(70650236636294400),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18445336716005867520),
                    white: Bitboard(302050815),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
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
pub mod santasiere_variation;
pub use santasiere_variation::SANTASIERE_VARIATION;
pub mod marshall_variation;
pub use marshall_variation::MARSHALL_VARIATION;
pub mod nanu_gambit;
pub use nanu_gambit::NANU_GAMBIT;
pub mod deferred_variation;
pub use deferred_variation::DEFERRED_VARIATION;
pub mod romanian_defense;
pub use romanian_defense::ROMANIAN_DEFENSE;
pub mod marienbad_variation;
pub use marienbad_variation::MARIENBAD_VARIATION;
pub mod carlsbad_variation;
pub use carlsbad_variation::CARLSBAD_VARIATION;
pub mod abrahams_variation;
pub use abrahams_variation::ABRAHAMS_VARIATION;
