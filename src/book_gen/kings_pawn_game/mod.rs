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
# use reco::book::KINGS_PAWN_GAME;
assert_eq!(KINGS_PAWN_GAME.original_name(), "King's Pawn Game");
```"#
)]
pub static KINGS_PAWN_GAME: Variation = Variation {
    name: "King's Pawn Game",
    parent: None,
    variations: &[
        &ALAPIN_OPENING,
        &BAVARIAN_GAMBIT,
        &BEYER_GAMBIT,
        &BUSCH_GASS_GAMBIT,
        &CLAM_VARIATION,
        &DAMIANO_DEFENSE,
        &DRESDEN_OPENING,
        &GUNDERAM_DEFENSE,
        &GUNDERAM_GAMBIT,
        &KINGS_HEAD_OPENING,
        &LA_BOURDONNAIS_GAMBIT,
        &LEONARDIS_VARIATION,
        &MAC_LEOD_ATTACK,
        &MAROCZY_DEFENSE,
        &MC_CONNELL_DEFENSE,
        &MENGARINIS_OPENING,
        &NAPOLEON_ATTACK,
        &PACHMAN_WING_GAMBIT,
        &PHILIDOR_GAMBIT,
        &SCHULZE_MULLER_GAMBIT,
        &TAYLER_OPENING,
        &TORTOISE_OPENING,
        &WAYWARD_QUEEN_ATTACK,
        &WEBER_GAMBIT,
    ],
    lines: &[
        Line {
            parent: &KINGS_PAWN_GAME,
            code: Code {
                volume: Volume::B,
                category: Category(RangedU8::new_static::<0>()),
            },
            moves: &[Normal {
                role: Pawn,
                from: Square::E2,
                capture: None,
                to: Square::E4,
                promotion: None,
            }],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(71776119329713920),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18446462598732840960),
                        white: Bitboard(268496895),
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
        },
        Line {
            parent: &KINGS_PAWN_GAME,
            code: Code {
                volume: Volume::C,
                category: Category(RangedU8::new_static::<2>()),
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
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
                    ByRole {
                        pawn: Bitboard(67272588421820160),
                        knight: Bitboard(4755801206503243842),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(18441959067824947200),
                        white: Bitboard(268496895),
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
        },
    ],
};
pub mod alapin_opening;
pub use alapin_opening::ALAPIN_OPENING;
pub mod bavarian_gambit;
pub use bavarian_gambit::BAVARIAN_GAMBIT;
pub mod beyer_gambit;
pub use beyer_gambit::BEYER_GAMBIT;
pub mod busch_gass_gambit;
pub use busch_gass_gambit::BUSCH_GASS_GAMBIT;
pub mod clam_variation;
pub use clam_variation::CLAM_VARIATION;
pub mod damiano_defense;
pub use damiano_defense::DAMIANO_DEFENSE;
pub mod dresden_opening;
pub use dresden_opening::DRESDEN_OPENING;
pub mod gunderam_defense;
pub use gunderam_defense::GUNDERAM_DEFENSE;
pub mod gunderam_gambit;
pub use gunderam_gambit::GUNDERAM_GAMBIT;
pub mod kings_head_opening;
pub use kings_head_opening::KINGS_HEAD_OPENING;
pub mod la_bourdonnais_gambit;
pub use la_bourdonnais_gambit::LA_BOURDONNAIS_GAMBIT;
pub mod leonardis_variation;
pub use leonardis_variation::LEONARDIS_VARIATION;
pub mod mac_leod_attack;
pub use mac_leod_attack::MAC_LEOD_ATTACK;
pub mod maroczy_defense;
pub use maroczy_defense::MAROCZY_DEFENSE;
pub mod mc_connell_defense;
pub use mc_connell_defense::MC_CONNELL_DEFENSE;
pub mod mengarinis_opening;
pub use mengarinis_opening::MENGARINIS_OPENING;
pub mod napoleon_attack;
pub use napoleon_attack::NAPOLEON_ATTACK;
pub mod pachman_wing_gambit;
pub use pachman_wing_gambit::PACHMAN_WING_GAMBIT;
pub mod philidor_gambit;
pub use philidor_gambit::PHILIDOR_GAMBIT;
pub mod schulze_muller_gambit;
pub use schulze_muller_gambit::SCHULZE_MULLER_GAMBIT;
pub mod tayler_opening;
pub use tayler_opening::TAYLER_OPENING;
pub mod tortoise_opening;
pub use tortoise_opening::TORTOISE_OPENING;
pub mod wayward_queen_attack;
pub use wayward_queen_attack::WAYWARD_QUEEN_ATTACK;
pub mod weber_gambit;
pub use weber_gambit::WEBER_GAMBIT;
