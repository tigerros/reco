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

/// Center Game.
pub const CENTER_GAME: [Opening<'static, &str>; 2] = [
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<20>(),
        },
        name: "Center Game",
        variation: &[],
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
                role: Pawn,
                from: D2,
                capture: None,
                to: D4,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67272588556035840),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18441959067824947200),
                    white: Bitboard(402712575),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 0,
            fullmoves: if let Some(fullmoves) = NonZeroU32::new(2) {
                fullmoves
            } else {
                panic!("fullmoves is zero")
            },
        },
    },
    Opening {
        code: Code {
            volume: Volume::C,
            category: RangedU8::new_static::<21>(),
        },
        name: "Center Game",
        variation: &[],
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
                role: Pawn,
                from: D2,
                capture: None,
                to: D4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E5,
                capture: Some(Pawn),
                to: D4,
                promotion: None,
            },
            Normal {
                role: Queen,
                from: D1,
                capture: Some(Pawn),
                to: D4,
                promotion: None,
            },
        ],
        setup: &Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67272519702341376),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752437641216),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18441958999105470464),
                    white: Bitboard(402712567),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
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
];
pub mod kieseritzky_variation;
pub use kieseritzky_variation::KIESERITZKY_VARIATION;
pub mod paulsen_attack_variation;
pub use paulsen_attack_variation::PAULSEN_ATTACK_VARIATION;
pub mod charousek_variation;
pub use charousek_variation::CHAROUSEK_VARIATION;
pub mod halasz_mc_donnell_gambit;
pub use halasz_mc_donnell_gambit::HALASZ_MC_DONNELL_GAMBIT;
pub mod lanc_arnold_gambit;
pub use lanc_arnold_gambit::LANC_ARNOLD_GAMBIT;
pub mod hall_variation;
pub use hall_variation::HALL_VARIATION;
pub mod ross_gambit;
pub use ross_gambit::ROSS_GAMBIT;
pub mod kupreichik_variation;
pub use kupreichik_variation::KUPREICHIK_VARIATION;
pub mod berger_variation;
pub use berger_variation::BERGER_VARIATION;
pub mod von_der_lasa_gambit;
pub use von_der_lasa_gambit::VON_DER_LASA_GAMBIT;
pub mod normal_variation;
pub use normal_variation::NORMAL_VARIATION;
pub mod l_hermet_variation;
pub use l_hermet_variation::L_HERMET_VARIATION;
