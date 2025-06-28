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
# use reco::book::ENGLISH_OPENING;
assert_eq!(ENGLISH_OPENING.original_name(), "English Opening");
```"#
)]
pub static ENGLISH_OPENING: Variation = Variation {
    name: "English Opening",
    parent: None,
    variations: &[
        &ACHILLES_OMEGA_GAMBIT,
        &ADORJAN_DEFENSE,
        &AGINCOURT_DEFENSE,
        &ANGLO_DUTCH_DEFENSE,
        &ANGLO_DUTCH_VARIATION,
        &ANGLO_GRUNFELD_DEFENSE,
        &ANGLO_INDIAN_DEFENSE,
        &ANGLO_LITHUANIAN_VARIATION,
        &ANGLO_SCANDINAVIAN_DEFENSE,
        &CARLS_BREMEN_SYSTEM,
        &CARO_KANN_DEFENSIVE_SYSTEM,
        &CLOSED,
        &DRILL_VARIATION,
        &FOUR_KNIGHTS_SYSTEM,
        &GREAT_SNAKE_VARIATION,
        &JAENISCH_GAMBIT,
        &KINGS_ENGLISH,
        &KINGS_ENGLISH_VARIATION,
        &MIKENAS_CARLS,
        &MIKENAS_CARLS_VARIATION,
        &MYERS_DEFENSE,
        &MYERS_GAMBIT,
        &NEO_CATALAN,
        &NEO_CATALAN_DECLINED,
        &PORCUPINE_VARIATION,
        &ROMANISHIN_GAMBIT,
        &SYMMETRICAL,
        &SYMMETRICAL_VARIATION,
        &THE_WHALE,
        &WADE_GAMBIT,
        &WING_GAMBIT,
        &ZILBERMINTS_GAMBIT,
    ],
    lines: &[Line {
        parent: &ENGLISH_OPENING,
        code: Code {
            volume: Volume::A,
            category: Category::new_static::<1>(),
        },
        moves: &[Normal {
            role: Pawn,
            from: Square::C2,
            capture: None,
            to: Square::C4,
            promotion: None,
        }],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
                ByRole {
                    pawn: Bitboard(71776119128390400),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18446462598732840960),
                    white: Bitboard(67173375),
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
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(1) {
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
pub mod achilles_omega_gambit;
pub use achilles_omega_gambit::ACHILLES_OMEGA_GAMBIT;
pub mod adorjan_defense;
pub use adorjan_defense::ADORJAN_DEFENSE;
pub mod agincourt_defense;
pub use agincourt_defense::AGINCOURT_DEFENSE;
pub mod anglo_dutch_defense;
pub use anglo_dutch_defense::ANGLO_DUTCH_DEFENSE;
pub mod anglo_dutch_variation;
pub use anglo_dutch_variation::ANGLO_DUTCH_VARIATION;
pub mod anglo_grunfeld_defense;
pub use anglo_grunfeld_defense::ANGLO_GRUNFELD_DEFENSE;
pub mod anglo_indian_defense;
pub use anglo_indian_defense::ANGLO_INDIAN_DEFENSE;
pub mod anglo_lithuanian_variation;
pub use anglo_lithuanian_variation::ANGLO_LITHUANIAN_VARIATION;
pub mod anglo_scandinavian_defense;
pub use anglo_scandinavian_defense::ANGLO_SCANDINAVIAN_DEFENSE;
pub mod carls_bremen_system;
pub use carls_bremen_system::CARLS_BREMEN_SYSTEM;
pub mod caro_kann_defensive_system;
pub use caro_kann_defensive_system::CARO_KANN_DEFENSIVE_SYSTEM;
pub mod closed;
pub use closed::CLOSED;
pub mod drill_variation;
pub use drill_variation::DRILL_VARIATION;
pub mod four_knights_system;
pub use four_knights_system::FOUR_KNIGHTS_SYSTEM;
pub mod great_snake_variation;
pub use great_snake_variation::GREAT_SNAKE_VARIATION;
pub mod jaenisch_gambit;
pub use jaenisch_gambit::JAENISCH_GAMBIT;
pub mod kings_english;
pub use kings_english::KINGS_ENGLISH;
pub mod kings_english_variation;
pub use kings_english_variation::KINGS_ENGLISH_VARIATION;
pub mod mikenas_carls;
pub use mikenas_carls::MIKENAS_CARLS;
pub mod mikenas_carls_variation;
pub use mikenas_carls_variation::MIKENAS_CARLS_VARIATION;
pub mod myers_defense;
pub use myers_defense::MYERS_DEFENSE;
pub mod myers_gambit;
pub use myers_gambit::MYERS_GAMBIT;
pub mod neo_catalan;
pub use neo_catalan::NEO_CATALAN;
pub mod neo_catalan_declined;
pub use neo_catalan_declined::NEO_CATALAN_DECLINED;
pub mod porcupine_variation;
pub use porcupine_variation::PORCUPINE_VARIATION;
pub mod romanishin_gambit;
pub use romanishin_gambit::ROMANISHIN_GAMBIT;
pub mod symmetrical;
pub use symmetrical::SYMMETRICAL;
pub mod symmetrical_variation;
pub use symmetrical_variation::SYMMETRICAL_VARIATION;
pub mod the_whale;
pub use the_whale::THE_WHALE;
pub mod wade_gambit;
pub use wade_gambit::WADE_GAMBIT;
pub mod wing_gambit;
pub use wing_gambit::WING_GAMBIT;
pub mod zilbermints_gambit;
pub use zilbermints_gambit::ZILBERMINTS_GAMBIT;
