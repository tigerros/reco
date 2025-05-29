#[allow(unused_imports, clippy::enum_glob_use, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Move::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Role::{Pawn, Knight, Bishop, Rook, Queen, King};
#[allow(clippy::enum_glob_use, reason = "there's 64 variants in this enum, importing them all is stupid")]
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Square::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Color::{Black, White};
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::bitboard::Bitboard;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::board::Board;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::{ByRole, ByColor, Setup};
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use core::num::NonZeroU32;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use crate::{Variation, Line, Code, Volume, Category};#[cfg_attr(feature = "alloc", doc = r#"```rust
# use reco::book::HUNGARIAN_OPENING;
assert_eq!(HUNGARIAN_OPENING.original_name(), "Hungarian Opening");
```"#)]
pub static HUNGARIAN_OPENING: Variation = Variation {
    name: "Hungarian Opening",
    variations: &[&PACHMAN_GAMBIT,
&REVERSED_ALEKHINE,
&MYERS_DEFENSE,
&REVERSED_NORWEGIAN_DEFENSE,
&SICILIAN_INVITATION,
&INDIAN_DEFENSE,
&REVERSED_BROOKLYN_DEFENSE,
&WIEDENHAGEN_BETA_GAMBIT,
&PARIS_GAMBIT,
&ASTEN_GAMBIT,
&DUTCH_DEFENSE,
&CATALAN_FORMATION,
&BURK_GAMBIT,
&REVERSED_MODERN_DEFENSE,
&WINTERBERG_GAMBIT,
&SYMMETRICAL_VARIATION,
&BUCKER_GAMBIT,
&SLAV_FORMATION,
&VAN_KUIJK_GAMBIT],
    parent: None,
    lines: &[Line {
    code: Code {
        volume: Volume::A,
        category: Category::new_static::<0>()
    },
    moves: &[
    Normal {
        role: Pawn,
        from: G2,
        capture: None,
        to: G3,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(71776119065460480),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18446462598732840960),
                white: Bitboard(4243455)
            }
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
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod pachman_gambit;
pub use pachman_gambit::PACHMAN_GAMBIT;
pub mod reversed_alekhine;
pub use reversed_alekhine::REVERSED_ALEKHINE;
pub mod myers_defense;
pub use myers_defense::MYERS_DEFENSE;
pub mod reversed_norwegian_defense;
pub use reversed_norwegian_defense::REVERSED_NORWEGIAN_DEFENSE;
pub mod sicilian_invitation;
pub use sicilian_invitation::SICILIAN_INVITATION;
pub mod indian_defense;
pub use indian_defense::INDIAN_DEFENSE;
pub mod reversed_brooklyn_defense;
pub use reversed_brooklyn_defense::REVERSED_BROOKLYN_DEFENSE;
pub mod wiedenhagen_beta_gambit;
pub use wiedenhagen_beta_gambit::WIEDENHAGEN_BETA_GAMBIT;
pub mod paris_gambit;
pub use paris_gambit::PARIS_GAMBIT;
pub mod asten_gambit;
pub use asten_gambit::ASTEN_GAMBIT;
pub mod dutch_defense;
pub use dutch_defense::DUTCH_DEFENSE;
pub mod catalan_formation;
pub use catalan_formation::CATALAN_FORMATION;
pub mod burk_gambit;
pub use burk_gambit::BURK_GAMBIT;
pub mod reversed_modern_defense;
pub use reversed_modern_defense::REVERSED_MODERN_DEFENSE;
pub mod winterberg_gambit;
pub use winterberg_gambit::WINTERBERG_GAMBIT;
pub mod symmetrical_variation;
pub use symmetrical_variation::SYMMETRICAL_VARIATION;
pub mod bucker_gambit;
pub use bucker_gambit::BUCKER_GAMBIT;
pub mod slav_formation;
pub use slav_formation::SLAV_FORMATION;
pub mod van_kuijk_gambit;
pub use van_kuijk_gambit::VAN_KUIJK_GAMBIT;
