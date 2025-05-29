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
# use reco::book::ENGLISH_OPENING;
assert_eq!(ENGLISH_OPENING.original_name(), "English Opening");
```"#
)]
pub static ENGLISH_OPENING: Variation = Variation {
    name: "English Opening",
    parent: None,
    variations: &[
        &CLOSED,
        &MYERS_DEFENSE,
        &MIKENAS_CARLS_VARIATION,
        &KINGS_ENGLISH,
        &KINGS_ENGLISH_VARIATION,
        &SYMMETRICAL_VARIATION,
        &PORCUPINE_VARIATION,
        &CARO_KANN_DEFENSIVE_SYSTEM,
        &ANGLO_DUTCH_DEFENSE,
        &DRILL_VARIATION,
        &WADE_GAMBIT,
        &ANGLO_SCANDINAVIAN_DEFENSE,
        &AGINCOURT_DEFENSE,
        &NEO_CATALAN,
        &ZILBERMINTS_GAMBIT,
        &CARLS_BREMEN_SYSTEM,
        &FOUR_KNIGHTS_SYSTEM,
        &ANGLO_GRUNFELD_DEFENSE,
        &MYERS_GAMBIT,
        &ACHILLES_OMEGA_GAMBIT,
        &THE_WHALE,
        &GREAT_SNAKE_VARIATION,
        &JAENISCH_GAMBIT,
        &ANGLO_DUTCH_VARIATION,
        &NEO_CATALAN_DECLINED,
        &ANGLO_INDIAN_DEFENSE,
        &SYMMETRICAL,
        &ANGLO_LITHUANIAN_VARIATION,
        &WING_GAMBIT,
        &ADORJAN_DEFENSE,
        &MIKENAS_CARLS,
        &ROMANISHIN_GAMBIT,
    ],
    lines: &[Line {
        parent: &ENGLISH_OPENING,
        code: Code {
            volume: Volume::A,
            category: Category::new_static::<10>(),
        },
        moves: &[Normal {
            role: Pawn,
            from: C2,
            capture: None,
            to: C4,
            promotion: None,
        }],
        setup: Setup {
            board: Board::from_bitboards(
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
            ),
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
pub mod closed;
pub use closed::CLOSED;
pub mod myers_defense;
pub use myers_defense::MYERS_DEFENSE;
pub mod mikenas_carls_variation;
pub use mikenas_carls_variation::MIKENAS_CARLS_VARIATION;
pub mod kings_english;
pub use kings_english::KINGS_ENGLISH;
pub mod kings_english_variation;
pub use kings_english_variation::KINGS_ENGLISH_VARIATION;
pub mod symmetrical_variation;
pub use symmetrical_variation::SYMMETRICAL_VARIATION;
pub mod porcupine_variation;
pub use porcupine_variation::PORCUPINE_VARIATION;
pub mod caro_kann_defensive_system;
pub use caro_kann_defensive_system::CARO_KANN_DEFENSIVE_SYSTEM;
pub mod anglo_dutch_defense;
pub use anglo_dutch_defense::ANGLO_DUTCH_DEFENSE;
pub mod drill_variation;
pub use drill_variation::DRILL_VARIATION;
pub mod wade_gambit;
pub use wade_gambit::WADE_GAMBIT;
pub mod anglo_scandinavian_defense;
pub use anglo_scandinavian_defense::ANGLO_SCANDINAVIAN_DEFENSE;
pub mod agincourt_defense;
pub use agincourt_defense::AGINCOURT_DEFENSE;
pub mod neo_catalan;
pub use neo_catalan::NEO_CATALAN;
pub mod zilbermints_gambit;
pub use zilbermints_gambit::ZILBERMINTS_GAMBIT;
pub mod carls_bremen_system;
pub use carls_bremen_system::CARLS_BREMEN_SYSTEM;
pub mod four_knights_system;
pub use four_knights_system::FOUR_KNIGHTS_SYSTEM;
pub mod anglo_grunfeld_defense;
pub use anglo_grunfeld_defense::ANGLO_GRUNFELD_DEFENSE;
pub mod myers_gambit;
pub use myers_gambit::MYERS_GAMBIT;
pub mod achilles_omega_gambit;
pub use achilles_omega_gambit::ACHILLES_OMEGA_GAMBIT;
pub mod the_whale;
pub use the_whale::THE_WHALE;
pub mod great_snake_variation;
pub use great_snake_variation::GREAT_SNAKE_VARIATION;
pub mod jaenisch_gambit;
pub use jaenisch_gambit::JAENISCH_GAMBIT;
pub mod anglo_dutch_variation;
pub use anglo_dutch_variation::ANGLO_DUTCH_VARIATION;
pub mod neo_catalan_declined;
pub use neo_catalan_declined::NEO_CATALAN_DECLINED;
pub mod anglo_indian_defense;
pub use anglo_indian_defense::ANGLO_INDIAN_DEFENSE;
pub mod symmetrical;
pub use symmetrical::SYMMETRICAL;
pub mod anglo_lithuanian_variation;
pub use anglo_lithuanian_variation::ANGLO_LITHUANIAN_VARIATION;
pub mod wing_gambit;
pub use wing_gambit::WING_GAMBIT;
pub mod adorjan_defense;
pub use adorjan_defense::ADORJAN_DEFENSE;
pub mod mikenas_carls;
pub use mikenas_carls::MIKENAS_CARLS;
pub mod romanishin_gambit;
pub use romanishin_gambit::ROMANISHIN_GAMBIT;
