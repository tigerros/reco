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
# use reco::book::BENONI_DEFENSE;
assert_eq!(BENONI_DEFENSE.original_name(), "Benoni Defense");
```"#
)]
pub static BENONI_DEFENSE: Variation = Variation {
    name: "Benoni Defense",
    parent: None,
    variations: &[
        &ZILBERMINTS_BENONI_GAMBIT,
        &CZECH_BENONI_DEFENSE,
        &OLD_BENONI,
        &KINGS_INDIAN_SYSTEM,
        &HROMADKA_SYSTEM,
        &KINGS_PAWN_LINE,
        &MIKENAS_VARIATION,
        &PAWN_STORM_VARIATION,
        &WOOZLE,
        &UHLMANN_VARIATION,
        &BENONI_GAMBIT_ACCEPTED,
        &KNIGHTS_TOUR_VARIATION,
        &WEENINK_VARIATION,
        &BENONI_STAUNTON_GAMBIT,
        &BENONI_GAMBIT,
        &CORMORANT_GAMBIT,
        &FIANCHETTO_VARIATION,
        &BENONI_INDIAN_DEFENSE,
        &MODERN_VARIATION,
        &FRENCH_BENONI,
        &FRANCO_SICILIAN_HYBRID,
        &FOUR_PAWNS_ATTACK,
        &HAWK_VARIATION,
        &CLASSICAL_VARIATION,
        &SEMI_BENONI,
        &SNAIL_VARIATION,
        &TAIMANOV_VARIATION,
    ],
    lines: &[
        Line {
            parent: &BENONI_DEFENSE,
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<61>(),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: D2,
                    capture: None,
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: G8,
                    capture: None,
                    to: F6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C2,
                    capture: None,
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E7,
                    capture: None,
                    to: E6,
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
                    from: C7,
                    capture: None,
                    to: C5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D4,
                    capture: None,
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: E6,
                    capture: Some(Pawn),
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C4,
                    capture: Some(Pawn),
                    to: D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: D7,
                    capture: None,
                    to: D6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: B1,
                    capture: None,
                    to: C3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: G7,
                    capture: None,
                    to: G6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(45959637580706560),
                        knight: Bitboard(144150372450304000),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(13808995248837230592),
                        white: Bitboard(34362160061),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
                castling_rights: Bitboard(9295429630892703873),
                ep_square: None,
                remaining_checks: None,
                halfmoves: 0,
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(7) {
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
            parent: &BENONI_DEFENSE,
            code: Code {
                volume: Volume::A,
                category: Category::new_static::<56>(),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: D2,
                    capture: None,
                    to: D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: G8,
                    capture: None,
                    to: F6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C2,
                    capture: None,
                    to: C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: C7,
                    capture: None,
                    to: C5,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: Board::from_bitboards(
                    ByRole {
                        pawn: Bitboard(70650236535632640),
                        knight: Bitboard(144150372447944770),
                        bishop: Bitboard(2594073385365405732),
                        rook: Bitboard(9295429630892703873),
                        queen: Bitboard(576460752303423496),
                        king: Bitboard(1152921504606846992),
                    },
                    ByColor {
                        black: Bitboard(13833685881950568448),
                        white: Bitboard(201389055),
                    },
                ),
                promoted: Bitboard(0),
                pockets: None,
                turn: White,
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
        },
    ],
};
pub mod zilbermints_benoni_gambit;
pub use zilbermints_benoni_gambit::ZILBERMINTS_BENONI_GAMBIT;
pub mod czech_benoni_defense;
pub use czech_benoni_defense::CZECH_BENONI_DEFENSE;
pub mod old_benoni;
pub use old_benoni::OLD_BENONI;
pub mod kings_indian_system;
pub use kings_indian_system::KINGS_INDIAN_SYSTEM;
pub mod hromadka_system;
pub use hromadka_system::HROMADKA_SYSTEM;
pub mod kings_pawn_line;
pub use kings_pawn_line::KINGS_PAWN_LINE;
pub mod mikenas_variation;
pub use mikenas_variation::MIKENAS_VARIATION;
pub mod pawn_storm_variation;
pub use pawn_storm_variation::PAWN_STORM_VARIATION;
pub mod woozle;
pub use woozle::WOOZLE;
pub mod uhlmann_variation;
pub use uhlmann_variation::UHLMANN_VARIATION;
pub mod benoni_gambit_accepted;
pub use benoni_gambit_accepted::BENONI_GAMBIT_ACCEPTED;
pub mod knights_tour_variation;
pub use knights_tour_variation::KNIGHTS_TOUR_VARIATION;
pub mod weenink_variation;
pub use weenink_variation::WEENINK_VARIATION;
pub mod benoni_staunton_gambit;
pub use benoni_staunton_gambit::BENONI_STAUNTON_GAMBIT;
pub mod benoni_gambit;
pub use benoni_gambit::BENONI_GAMBIT;
pub mod cormorant_gambit;
pub use cormorant_gambit::CORMORANT_GAMBIT;
pub mod fianchetto_variation;
pub use fianchetto_variation::FIANCHETTO_VARIATION;
pub mod benoni_indian_defense;
pub use benoni_indian_defense::BENONI_INDIAN_DEFENSE;
pub mod modern_variation;
pub use modern_variation::MODERN_VARIATION;
pub mod french_benoni;
pub use french_benoni::FRENCH_BENONI;
pub mod franco_sicilian_hybrid;
pub use franco_sicilian_hybrid::FRANCO_SICILIAN_HYBRID;
pub mod four_pawns_attack;
pub use four_pawns_attack::FOUR_PAWNS_ATTACK;
pub mod hawk_variation;
pub use hawk_variation::HAWK_VARIATION;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod semi_benoni;
pub use semi_benoni::SEMI_BENONI;
pub mod snail_variation;
pub use snail_variation::SNAIL_VARIATION;
pub mod taimanov_variation;
pub use taimanov_variation::TAIMANOV_VARIATION;
