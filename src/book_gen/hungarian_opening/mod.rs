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
# use reco::book::HUNGARIAN_OPENING;
assert_eq!(HUNGARIAN_OPENING.original_name(), "Hungarian Opening");
```"#
)]
pub static HUNGARIAN_OPENING: Variation = Variation {
    name: "Hungarian Opening",
    parent: None,
    variations: &[
        &ASTEN_GAMBIT,
        &BURK_GAMBIT,
        &BUCKER_GAMBIT,
        &CATALAN_FORMATION,
        &DUTCH_DEFENSE,
        &INDIAN_DEFENSE,
        &MYERS_DEFENSE,
        &PACHMAN_GAMBIT,
        &PARIS_GAMBIT,
        &REVERSED_ALEKHINE,
        &REVERSED_BROOKLYN_DEFENSE,
        &REVERSED_MODERN_DEFENSE,
        &REVERSED_NORWEGIAN_DEFENSE,
        &SICILIAN_INVITATION,
        &SLAV_FORMATION,
        &SYMMETRICAL_VARIATION,
        &VAN_KUIJK_GAMBIT,
        &WIEDENHAGEN_BETA_GAMBIT,
        &WINTERBERG_GAMBIT,
    ],
    lines: &[Line {
        parent: &HUNGARIAN_OPENING,
        code: Code {
            volume: Volume::A,
            category: Category::new_static::<0>(),
        },
        moves: &[Normal {
            role: Pawn,
            from: G2,
            capture: None,
            to: G3,
            promotion: None,
        }],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(71776119065460480),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18446462598732840960),
                    white: Bitboard(4243455),
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
pub mod asten_gambit;
pub use asten_gambit::ASTEN_GAMBIT;
pub mod burk_gambit;
pub use burk_gambit::BURK_GAMBIT;
pub mod bucker_gambit;
pub use bucker_gambit::BUCKER_GAMBIT;
pub mod catalan_formation;
pub use catalan_formation::CATALAN_FORMATION;
pub mod dutch_defense;
pub use dutch_defense::DUTCH_DEFENSE;
pub mod indian_defense;
pub use indian_defense::INDIAN_DEFENSE;
pub mod myers_defense;
pub use myers_defense::MYERS_DEFENSE;
pub mod pachman_gambit;
pub use pachman_gambit::PACHMAN_GAMBIT;
pub mod paris_gambit;
pub use paris_gambit::PARIS_GAMBIT;
pub mod reversed_alekhine;
pub use reversed_alekhine::REVERSED_ALEKHINE;
pub mod reversed_brooklyn_defense;
pub use reversed_brooklyn_defense::REVERSED_BROOKLYN_DEFENSE;
pub mod reversed_modern_defense;
pub use reversed_modern_defense::REVERSED_MODERN_DEFENSE;
pub mod reversed_norwegian_defense;
pub use reversed_norwegian_defense::REVERSED_NORWEGIAN_DEFENSE;
pub mod sicilian_invitation;
pub use sicilian_invitation::SICILIAN_INVITATION;
pub mod slav_formation;
pub use slav_formation::SLAV_FORMATION;
pub mod symmetrical_variation;
pub use symmetrical_variation::SYMMETRICAL_VARIATION;
pub mod van_kuijk_gambit;
pub use van_kuijk_gambit::VAN_KUIJK_GAMBIT;
pub mod wiedenhagen_beta_gambit;
pub use wiedenhagen_beta_gambit::WIEDENHAGEN_BETA_GAMBIT;
pub mod winterberg_gambit;
pub use winterberg_gambit::WINTERBERG_GAMBIT;
