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
# use reco::book::VIENNA_GAME;
assert_eq!(VIENNA_GAME.original_name(), "Vienna Game");
```"#
)]
pub static VIENNA_GAME: Variation = Variation {
    name: "Vienna Game",
    parent: None,
    variations: &[
        &ADAMS_GAMBIT,
        &ANDERSSEN_DEFENSE,
        &FALKBEER_VARIATION,
        &FRANKENSTEIN_DRACULA_VARIATION,
        &FYFE_GAMBIT,
        &GIRAFFE_ATTACK,
        &HAMPPE_MEITNER_VARIATION,
        &HAMPPE_MUZIO,
        &HAMPPE_MUZIO_GAMBIT,
        &HEYDE_VARIATION,
        &MAX_LANGE_DEFENSE,
        &MENGARINI_VARIATION,
        &MIESES_VARIATION,
        &OMAHA_GAMBIT,
        &PAULSEN_VARIATION,
        &PHILIDOR_COUNTERGAMBIT,
        &PIERCE_GAMBIT,
        &STANLEY_VARIATION,
        &VIENNA_GAMBIT,
        &ZHURAVLEV_COUNTERGAMBIT,
    ],
    lines: &[Line {
        parent: &VIENNA_GAME,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<2>(),
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
                from: Square::B1,
                capture: None,
                to: Square::C3,
                promotion: None,
            },
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(67272588421820160),
                    knight: Bitboard(4755801206503505984),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18441959067824947200),
                    white: Bitboard(268759037),
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
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
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
pub mod adams_gambit;
pub use adams_gambit::ADAMS_GAMBIT;
pub mod anderssen_defense;
pub use anderssen_defense::ANDERSSEN_DEFENSE;
pub mod falkbeer_variation;
pub use falkbeer_variation::FALKBEER_VARIATION;
pub mod frankenstein_dracula_variation;
pub use frankenstein_dracula_variation::FRANKENSTEIN_DRACULA_VARIATION;
pub mod fyfe_gambit;
pub use fyfe_gambit::FYFE_GAMBIT;
pub mod giraffe_attack;
pub use giraffe_attack::GIRAFFE_ATTACK;
pub mod hamppe_meitner_variation;
pub use hamppe_meitner_variation::HAMPPE_MEITNER_VARIATION;
pub mod hamppe_muzio;
pub use hamppe_muzio::HAMPPE_MUZIO;
pub mod hamppe_muzio_gambit;
pub use hamppe_muzio_gambit::HAMPPE_MUZIO_GAMBIT;
pub mod heyde_variation;
pub use heyde_variation::HEYDE_VARIATION;
pub mod max_lange_defense;
pub use max_lange_defense::MAX_LANGE_DEFENSE;
pub mod mengarini_variation;
pub use mengarini_variation::MENGARINI_VARIATION;
pub mod mieses_variation;
pub use mieses_variation::MIESES_VARIATION;
pub mod omaha_gambit;
pub use omaha_gambit::OMAHA_GAMBIT;
pub mod paulsen_variation;
pub use paulsen_variation::PAULSEN_VARIATION;
pub mod philidor_countergambit;
pub use philidor_countergambit::PHILIDOR_COUNTERGAMBIT;
pub mod pierce_gambit;
pub use pierce_gambit::PIERCE_GAMBIT;
pub mod stanley_variation;
pub use stanley_variation::STANLEY_VARIATION;
pub mod vienna_gambit;
pub use vienna_gambit::VIENNA_GAMBIT;
pub mod zhuravlev_countergambit;
pub use zhuravlev_countergambit::ZHURAVLEV_COUNTERGAMBIT;
