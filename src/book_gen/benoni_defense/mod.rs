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
use core::unreachable;
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
# use reco::book::BENONI_DEFENSE;
assert_eq!(BENONI_DEFENSE.original_name(), "Benoni Defense");
```"#
)]
pub static BENONI_DEFENSE: Variation = Variation {
    name: "Benoni Defense",
    parent: None,
    variations: &[
        &BENONI_GAMBIT,
        &BENONI_GAMBIT_ACCEPTED,
        &BENONI_INDIAN_DEFENSE,
        &BENONI_STAUNTON_GAMBIT,
        &CLASSICAL_VARIATION,
        &CORMORANT_GAMBIT,
        &CZECH_BENONI_DEFENSE,
        &FIANCHETTO_VARIATION,
        &FOUR_PAWNS_ATTACK,
        &FRANCO_SICILIAN_HYBRID,
        &FRENCH_BENONI,
        &HAWK_VARIATION,
        &HROMADKA_SYSTEM,
        &KINGS_INDIAN_SYSTEM,
        &KINGS_PAWN_LINE,
        &KNIGHTS_TOUR_VARIATION,
        &MIKENAS_VARIATION,
        &MODERN_VARIATION,
        &OLD_BENONI,
        &PAWN_STORM_VARIATION,
        &SEMI_BENONI,
        &SNAIL_VARIATION,
        &TAIMANOV_VARIATION,
        &UHLMANN_VARIATION,
        &WEENINK_VARIATION,
        &WOOZLE,
        &ZILBERMINTS_BENONI_GAMBIT,
    ],
    lines: &[
        Line {
            parent: &BENONI_DEFENSE,
            code: Code {
                volume: Volume::A,
                category: Category(RangedU8::new_static::<5>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C2,
                    capture: None,
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C7,
                    capture: None,
                    to: Square::C5,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
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
        Line {
            parent: &BENONI_DEFENSE,
            code: Code {
                volume: Volume::A,
                category: Category(RangedU8::new_static::<6>()),
            },
            moves: &[
                Normal {
                    role: Pawn,
                    from: Square::D2,
                    capture: None,
                    to: Square::D4,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::G8,
                    capture: None,
                    to: Square::F6,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C2,
                    capture: None,
                    to: Square::C4,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E7,
                    capture: None,
                    to: Square::E6,
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
                    role: Pawn,
                    from: Square::C7,
                    capture: None,
                    to: Square::C5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D4,
                    capture: None,
                    to: Square::D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::E6,
                    capture: Some(Pawn),
                    to: Square::D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::C4,
                    capture: Some(Pawn),
                    to: Square::D5,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::D7,
                    capture: None,
                    to: Square::D6,
                    promotion: None,
                },
                Normal {
                    role: Knight,
                    from: Square::B1,
                    capture: None,
                    to: Square::C3,
                    promotion: None,
                },
                Normal {
                    role: Pawn,
                    from: Square::G7,
                    capture: None,
                    to: Square::G6,
                    promotion: None,
                },
            ],
            setup: Setup {
                board: if let Ok(board) = Board::try_from_bitboards(
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
    ],
};
pub mod benoni_gambit;
pub use benoni_gambit::BENONI_GAMBIT;
pub mod benoni_gambit_accepted;
pub use benoni_gambit_accepted::BENONI_GAMBIT_ACCEPTED;
pub mod benoni_indian_defense;
pub use benoni_indian_defense::BENONI_INDIAN_DEFENSE;
pub mod benoni_staunton_gambit;
pub use benoni_staunton_gambit::BENONI_STAUNTON_GAMBIT;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod cormorant_gambit;
pub use cormorant_gambit::CORMORANT_GAMBIT;
pub mod czech_benoni_defense;
pub use czech_benoni_defense::CZECH_BENONI_DEFENSE;
pub mod fianchetto_variation;
pub use fianchetto_variation::FIANCHETTO_VARIATION;
pub mod four_pawns_attack;
pub use four_pawns_attack::FOUR_PAWNS_ATTACK;
pub mod franco_sicilian_hybrid;
pub use franco_sicilian_hybrid::FRANCO_SICILIAN_HYBRID;
pub mod french_benoni;
pub use french_benoni::FRENCH_BENONI;
pub mod hawk_variation;
pub use hawk_variation::HAWK_VARIATION;
pub mod hromadka_system;
pub use hromadka_system::HROMADKA_SYSTEM;
pub mod kings_indian_system;
pub use kings_indian_system::KINGS_INDIAN_SYSTEM;
pub mod kings_pawn_line;
pub use kings_pawn_line::KINGS_PAWN_LINE;
pub mod knights_tour_variation;
pub use knights_tour_variation::KNIGHTS_TOUR_VARIATION;
pub mod mikenas_variation;
pub use mikenas_variation::MIKENAS_VARIATION;
pub mod modern_variation;
pub use modern_variation::MODERN_VARIATION;
pub mod old_benoni;
pub use old_benoni::OLD_BENONI;
pub mod pawn_storm_variation;
pub use pawn_storm_variation::PAWN_STORM_VARIATION;
pub mod semi_benoni;
pub use semi_benoni::SEMI_BENONI;
pub mod snail_variation;
pub use snail_variation::SNAIL_VARIATION;
pub mod taimanov_variation;
pub use taimanov_variation::TAIMANOV_VARIATION;
pub mod uhlmann_variation;
pub use uhlmann_variation::UHLMANN_VARIATION;
pub mod weenink_variation;
pub use weenink_variation::WEENINK_VARIATION;
pub mod woozle;
pub use woozle::WOOZLE;
pub mod zilbermints_benoni_gambit;
pub use zilbermints_benoni_gambit::ZILBERMINTS_BENONI_GAMBIT;
