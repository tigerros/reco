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
use deranged::RangedU8;
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
# use reco::book::ITALIAN_GAME;
assert_eq!(ITALIAN_GAME.original_name(), "Italian Game");
```"#
)]
pub static ITALIAN_GAME: Variation = Variation {
    name: "Italian Game",
    parent: None,
    variations: &[
        &ANTI_FRIED_LIVER_DEFENSE,
        &BIRDS_ATTACK,
        &BLACKBURNE_KOSTIC_GAMBIT,
        &CLASSICAL_VARIATION,
        &DEUTZ_GAMBIT,
        &EVANS_GAMBIT,
        &EVANS_GAMBIT_ACCEPTED,
        &EVANS_GAMBIT_DECLINED,
        &GIUOCO_PIANISSIMO,
        &GIUOCO_PIANO,
        &HUNGARIAN_DEFENSE,
        &JEROME_GAMBIT,
        &PARIS_DEFENSE,
        &ROSENTRETER_GAMBIT,
        &ROUSSEAU_GAMBIT,
        &SCOTCH_GAMBIT,
        &SCOTCH_INVITATION_DECLINED,
        &TWO_KNIGHTS_DEFENSE,
    ],
    lines: &[Line {
        parent: &ITALIAN_GAME,
        code: Code {
            volume: Volume::C,
            category: Category(RangedU8::new_static::<5>()),
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
        ],
        setup: Setup {
            board: if let Ok(board) = Board::try_from_bitboards(
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
pub mod anti_fried_liver_defense;
pub use anti_fried_liver_defense::ANTI_FRIED_LIVER_DEFENSE;
pub mod birds_attack;
pub use birds_attack::BIRDS_ATTACK;
pub mod blackburne_kostic_gambit;
pub use blackburne_kostic_gambit::BLACKBURNE_KOSTIC_GAMBIT;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod deutz_gambit;
pub use deutz_gambit::DEUTZ_GAMBIT;
pub mod evans_gambit;
pub use evans_gambit::EVANS_GAMBIT;
pub mod evans_gambit_accepted;
pub use evans_gambit_accepted::EVANS_GAMBIT_ACCEPTED;
pub mod evans_gambit_declined;
pub use evans_gambit_declined::EVANS_GAMBIT_DECLINED;
pub mod giuoco_pianissimo;
pub use giuoco_pianissimo::GIUOCO_PIANISSIMO;
pub mod giuoco_piano;
pub use giuoco_piano::GIUOCO_PIANO;
pub mod hungarian_defense;
pub use hungarian_defense::HUNGARIAN_DEFENSE;
pub mod jerome_gambit;
pub use jerome_gambit::JEROME_GAMBIT;
pub mod paris_defense;
pub use paris_defense::PARIS_DEFENSE;
pub mod rosentreter_gambit;
pub use rosentreter_gambit::ROSENTRETER_GAMBIT;
pub mod rousseau_gambit;
pub use rousseau_gambit::ROUSSEAU_GAMBIT;
pub mod scotch_gambit;
pub use scotch_gambit::SCOTCH_GAMBIT;
pub mod scotch_invitation_declined;
pub use scotch_invitation_declined::SCOTCH_INVITATION_DECLINED;
pub mod two_knights_defense;
pub use two_knights_defense::TWO_KNIGHTS_DEFENSE;
