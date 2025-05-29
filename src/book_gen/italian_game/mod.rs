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
# use reco::book::ITALIAN_GAME;
assert_eq!(ITALIAN_GAME.original_name(), "Italian Game");
```"#
)]
pub static ITALIAN_GAME: Variation = Variation {
    name: "Italian Game",
    parent: None,
    variations: &[
        &ROUSSEAU_GAMBIT,
        &BIRDS_ATTACK,
        &TWO_KNIGHTS_DEFENSE,
        &BLACKBURNE_KOSTIC_GAMBIT,
        &PARIS_DEFENSE,
        &JEROME_GAMBIT,
        &CLASSICAL_VARIATION,
        &DEUTZ_GAMBIT,
        &GIUOCO_PIANISSIMO,
        &ANTI_FRIED_LIVER_DEFENSE,
        &EVANS_GAMBIT,
        &ROSENTRETER_GAMBIT,
        &SCOTCH_INVITATION_DECLINED,
        &HUNGARIAN_DEFENSE,
        &EVANS_GAMBIT_ACCEPTED,
        &SCOTCH_GAMBIT,
        &GIUOCO_PIANO,
        &EVANS_GAMBIT_DECLINED,
    ],
    lines: &[Line {
        parent: &ITALIAN_GAME,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<50>(),
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
                role: Knight,
                from: B8,
                capture: None,
                to: C6,
                promotion: None,
            },
            Normal {
                role: Bishop,
                from: F1,
                capture: None,
                to: C4,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67272588421820160),
                    knight: Bitboard(4611690416475996162),
                    bishop: Bitboard(2594073385432514564),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18297848277795602432),
                    white: Bitboard(337702815),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 3,
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
pub mod rousseau_gambit;
pub use rousseau_gambit::ROUSSEAU_GAMBIT;
pub mod birds_attack;
pub use birds_attack::BIRDS_ATTACK;
pub mod two_knights_defense;
pub use two_knights_defense::TWO_KNIGHTS_DEFENSE;
pub mod blackburne_kostic_gambit;
pub use blackburne_kostic_gambit::BLACKBURNE_KOSTIC_GAMBIT;
pub mod paris_defense;
pub use paris_defense::PARIS_DEFENSE;
pub mod jerome_gambit;
pub use jerome_gambit::JEROME_GAMBIT;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod deutz_gambit;
pub use deutz_gambit::DEUTZ_GAMBIT;
pub mod giuoco_pianissimo;
pub use giuoco_pianissimo::GIUOCO_PIANISSIMO;
pub mod anti_fried_liver_defense;
pub use anti_fried_liver_defense::ANTI_FRIED_LIVER_DEFENSE;
pub mod evans_gambit;
pub use evans_gambit::EVANS_GAMBIT;
pub mod rosentreter_gambit;
pub use rosentreter_gambit::ROSENTRETER_GAMBIT;
pub mod scotch_invitation_declined;
pub use scotch_invitation_declined::SCOTCH_INVITATION_DECLINED;
pub mod hungarian_defense;
pub use hungarian_defense::HUNGARIAN_DEFENSE;
pub mod evans_gambit_accepted;
pub use evans_gambit_accepted::EVANS_GAMBIT_ACCEPTED;
pub mod scotch_gambit;
pub use scotch_gambit::SCOTCH_GAMBIT;
pub mod giuoco_piano;
pub use giuoco_piano::GIUOCO_PIANO;
pub mod evans_gambit_declined;
pub use evans_gambit_declined::EVANS_GAMBIT_DECLINED;
