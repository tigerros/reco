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
#[allow(
    clippy::doc_markdown,
    reason = "clippy confuses opening names for items"
)]
/// Van Geet Opening
pub static VAN_GEET_OPENING: Variation = Variation {
    name: "Van Geet Opening",
    parent: None,
    variations: &[
        &TUBINGEN_GAMBIT,
        &VENEZOLANA_VARIATION,
        &REVERSED_SCANDINAVIAN,
        &DOUGHERTY_GAMBIT,
        &DUNST_PERRENET_GAMBIT,
        &SICILIAN_TWO_KNIGHTS,
        &NOWOKUNSKI_GAMBIT,
        &KLUEVER_GAMBIT,
        &HULSEMANN_GAMBIT,
        &PFEIFFER_GAMBIT,
        &BERLIN_GAMBIT,
        &TWYBLE_ATTACK,
        &MYERS_ATTACK,
        &WARSTEINER_GAMBIT,
        &SLEIPNIR_GAMBIT,
        &HECTOR_GAMBIT,
        &GRUNFELD_DEFENSE,
        &BILLOCKUS_JOHANSEN_GAMBIT,
        &JENDROSSEK_GAMBIT,
        &LAROCHE_GAMBIT,
        &MELLEBY_GAMBIT,
        &DAMHAUG_GAMBIT,
        &LIEBIG_GAMBIT,
        &REVERSED_NIMZOWITSCH,
        &NAPOLEON_ATTACK,
        &GLADBACHER_GAMBIT,
        &BATTAMBANG_VARIATION,
        &DUSSELDORF_GAMBIT,
        &NOVOSIBIRSK_VARIATION,
        &HERGERT_GAMBIT,
    ],
    lines: &[Line {
        code: Code {
            volume: Volume::A,
            category: Category::new_static::<0>(),
        },
        moves: &[Normal {
            role: Knight,
            from: B1,
            capture: None,
            to: C3,
            promotion: None,
        }],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(71776119061282560),
                    knight: Bitboard(4755801206503505984),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18446462598732840960),
                    white: Bitboard(327677),
                },
            ),
            promoted: Bitboard(0),
            pockets: None,
            turn: Black,
            castling_rights: Bitboard(9295429630892703873),
            ep_square: None,
            remaining_checks: None,
            halfmoves: 1,
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
pub mod tubingen_gambit;
pub use tubingen_gambit::TUBINGEN_GAMBIT;
pub mod venezolana_variation;
pub use venezolana_variation::VENEZOLANA_VARIATION;
pub mod reversed_scandinavian;
pub use reversed_scandinavian::REVERSED_SCANDINAVIAN;
pub mod dougherty_gambit;
pub use dougherty_gambit::DOUGHERTY_GAMBIT;
pub mod dunst_perrenet_gambit;
pub use dunst_perrenet_gambit::DUNST_PERRENET_GAMBIT;
pub mod sicilian_two_knights;
pub use sicilian_two_knights::SICILIAN_TWO_KNIGHTS;
pub mod nowokunski_gambit;
pub use nowokunski_gambit::NOWOKUNSKI_GAMBIT;
pub mod kluever_gambit;
pub use kluever_gambit::KLUEVER_GAMBIT;
pub mod hulsemann_gambit;
pub use hulsemann_gambit::HULSEMANN_GAMBIT;
pub mod pfeiffer_gambit;
pub use pfeiffer_gambit::PFEIFFER_GAMBIT;
pub mod berlin_gambit;
pub use berlin_gambit::BERLIN_GAMBIT;
pub mod twyble_attack;
pub use twyble_attack::TWYBLE_ATTACK;
pub mod myers_attack;
pub use myers_attack::MYERS_ATTACK;
pub mod warsteiner_gambit;
pub use warsteiner_gambit::WARSTEINER_GAMBIT;
pub mod sleipnir_gambit;
pub use sleipnir_gambit::SLEIPNIR_GAMBIT;
pub mod hector_gambit;
pub use hector_gambit::HECTOR_GAMBIT;
pub mod grunfeld_defense;
pub use grunfeld_defense::GRUNFELD_DEFENSE;
pub mod billockus_johansen_gambit;
pub use billockus_johansen_gambit::BILLOCKUS_JOHANSEN_GAMBIT;
pub mod jendrossek_gambit;
pub use jendrossek_gambit::JENDROSSEK_GAMBIT;
pub mod laroche_gambit;
pub use laroche_gambit::LAROCHE_GAMBIT;
pub mod melleby_gambit;
pub use melleby_gambit::MELLEBY_GAMBIT;
pub mod damhaug_gambit;
pub use damhaug_gambit::DAMHAUG_GAMBIT;
pub mod liebig_gambit;
pub use liebig_gambit::LIEBIG_GAMBIT;
pub mod reversed_nimzowitsch;
pub use reversed_nimzowitsch::REVERSED_NIMZOWITSCH;
pub mod napoleon_attack;
pub use napoleon_attack::NAPOLEON_ATTACK;
pub mod gladbacher_gambit;
pub use gladbacher_gambit::GLADBACHER_GAMBIT;
pub mod battambang_variation;
pub use battambang_variation::BATTAMBANG_VARIATION;
pub mod dusseldorf_gambit;
pub use dusseldorf_gambit::DUSSELDORF_GAMBIT;
pub mod novosibirsk_variation;
pub use novosibirsk_variation::NOVOSIBIRSK_VARIATION;
pub mod hergert_gambit;
pub use hergert_gambit::HERGERT_GAMBIT;
