use core::num::NonZeroU32;
use core::panic;
use deranged::RangedU8;
use reco_core::{Code, Opening, Volume};
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
use shakmaty::Square::*;
use shakmaty::bitboard::Bitboard;
use shakmaty::board::Board;
use shakmaty::{ByColor, ByRole, Setup};

/// Benoni Defense.
pub const BENONI_DEFENSE: [Opening<'static, &str>; 2] = [
    Opening {
        code: Code {
            volume: Volume::A,
            category: RangedU8::new_static::<56>(),
        },
        name: "Benoni Defense",
        variation: &[],
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
        setup: &Setup {
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
                panic!("fullmoves is zero")
            },
        },
    },
    Opening {
        code: Code {
            volume: Volume::A,
            category: RangedU8::new_static::<61>(),
        },
        name: "Benoni Defense",
        variation: &[],
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
        setup: &Setup {
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
                panic!("fullmoves is zero")
            },
        },
    },
];
pub mod mikenas_variation;
pub use mikenas_variation::MIKENAS_VARIATION;
pub mod taimanov_variation;
pub use taimanov_variation::TAIMANOV_VARIATION;
pub mod zilbermints_benoni_gambit;
pub use zilbermints_benoni_gambit::ZILBERMINTS_BENONI_GAMBIT;
pub mod franco_sicilian_hybrid;
pub use franco_sicilian_hybrid::FRANCO_SICILIAN_HYBRID;
pub mod weenink_variation;
pub use weenink_variation::WEENINK_VARIATION;
pub mod benoni_indian_defense;
pub use benoni_indian_defense::BENONI_INDIAN_DEFENSE;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod woozle;
pub use woozle::WOOZLE;
pub mod benoni_staunton_gambit;
pub use benoni_staunton_gambit::BENONI_STAUNTON_GAMBIT;
pub mod czech_benoni_defense;
pub use czech_benoni_defense::CZECH_BENONI_DEFENSE;
pub mod benoni_gambit_accepted;
pub use benoni_gambit_accepted::BENONI_GAMBIT_ACCEPTED;
pub mod pawn_storm_variation;
pub use pawn_storm_variation::PAWN_STORM_VARIATION;
pub mod french_benoni;
pub use french_benoni::FRENCH_BENONI;
pub mod uhlmann_variation;
pub use uhlmann_variation::UHLMANN_VARIATION;
pub mod hawk_variation;
pub use hawk_variation::HAWK_VARIATION;
pub mod knights_tour_variation;
pub use knights_tour_variation::KNIGHTS_TOUR_VARIATION;
pub mod benoni_gambit;
pub mod semi_benoni;
pub use semi_benoni::SEMI_BENONI;
pub mod snail_variation;
pub use snail_variation::SNAIL_VARIATION;
pub mod hromadka_system;
pub use hromadka_system::HROMADKA_SYSTEM;
pub mod modern_variation;
pub use modern_variation::MODERN_VARIATION;
pub mod kings_indian_system;
pub use kings_indian_system::KINGS_INDIAN_SYSTEM;
pub mod four_pawns_attack;
pub use four_pawns_attack::FOUR_PAWNS_ATTACK;
pub mod kings_pawn_line;
pub use kings_pawn_line::KINGS_PAWN_LINE;
pub mod cormorant_gambit;
pub use cormorant_gambit::CORMORANT_GAMBIT;
pub mod old_benoni;
pub use old_benoni::OLD_BENONI;
pub mod fianchetto_variation;
pub use fianchetto_variation::FIANCHETTO_VARIATION;
