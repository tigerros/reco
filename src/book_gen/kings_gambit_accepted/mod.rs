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
# use reco::book::KINGS_GAMBIT_ACCEPTED;
assert_eq!(KINGS_GAMBIT_ACCEPTED.original_name(), "King's Gambit Accepted");
```"#
)]
pub static KINGS_GAMBIT_ACCEPTED: Variation = Variation {
    name: "King's Gambit Accepted",
    parent: None,
    variations: &[
        &ABBAZIA_DEFENSE,
        &ALLGAIER,
        &ALLGAIER_GAMBIT,
        &AUSTRALIAN_GAMBIT,
        &BASMAN_GAMBIT,
        &BECKER_DEFENSE,
        &BISHOPS_GAMBIT,
        &BLACHLY_GAMBIT,
        &BONSCH_OSMOLOVSKY_VARIATION,
        &BREYER_GAMBIT,
        &CARRERA_GAMBIT,
        &CUNNINGHAM_DEFENSE,
        &DODO_VARIATION,
        &DOUBLE_MUZIO_GAMBIT,
        &EISENBERG_VARIATION,
        &FISCHER_DEFENSE,
        &GAGA_GAMBIT,
        &GHULAM_KASSIM_GAMBIT,
        &GIANUTIO_COUNTERGAMBIT,
        &GRECO_GAMBIT,
        &HANSTEIN_GAMBIT,
        &KIESERITZKY,
        &KIESERITZKY_GAMBIT,
        &KINGS_KNIGHTS_GAMBIT,
        &KOTOV_GAMBIT,
        &LOLLI_GAMBIT,
        &LOPEZ_GIANUTIO_COUNTERGAMBIT,
        &MAC_LEOD_DEFENSE,
        &MASON_KERES_GAMBIT,
        &MAYET_GAMBIT,
        &MC_DONNELL_GAMBIT,
        &MIDDLETON_COUNTERGAMBIT,
        &MODERN_DEFENSE,
        &MUZIO_GAMBIT,
        &MUZIO_GAMBIT_ACCEPTED,
        &ORSINI_GAMBIT,
        &PARIS_GAMBIT,
        &PHILIDOR_GAMBIT,
        &QUAADE_GAMBIT,
        &ROSENTRETER_GAMBIT,
        &SALVIO_GAMBIT,
        &SCHALLOPP_DEFENSE,
        &SCHURIG_GAMBIT,
        &SILBERSCHMIDT_GAMBIT,
        &STAMMA_GAMBIT,
        &SORENSEN_GAMBIT,
        &TARTAKOWER_GAMBIT,
        &TRADITIONAL_VARIATION,
        &TUMBLEWEED,
        &VILLEMSON_GAMBIT,
        &WAGENBACH_DEFENSE,
    ],
    lines: &[Line {
        parent: &KINGS_GAMBIT_ACCEPTED,
        code: Code {
            volume: Volume::C,
            category: Category::new_static::<33>(),
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
                role: Pawn,
                from: F2,
                capture: None,
                to: F4,
                promotion: None,
            },
            Normal {
                role: Pawn,
                from: E5,
                capture: Some(Pawn),
                to: F4,
                promotion: None,
            },
        ],
        setup: Setup {
            board: Board::from_bitboards(
                ByRole {
                    pawn: Bitboard(67272520239206144),
                    knight: Bitboard(4755801206503243842),
                    bishop: Bitboard(2594073385365405732),
                    rook: Bitboard(9295429630892703873),
                    queen: Bitboard(576460752303423496),
                    king: Bitboard(1152921504606846992),
                },
                ByColor {
                    black: Bitboard(18441958999642341376),
                    white: Bitboard(268488703),
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
    }],
};
pub mod abbazia_defense;
pub use abbazia_defense::ABBAZIA_DEFENSE;
pub mod allgaier;
pub use allgaier::ALLGAIER;
pub mod allgaier_gambit;
pub use allgaier_gambit::ALLGAIER_GAMBIT;
pub mod australian_gambit;
pub use australian_gambit::AUSTRALIAN_GAMBIT;
pub mod basman_gambit;
pub use basman_gambit::BASMAN_GAMBIT;
pub mod becker_defense;
pub use becker_defense::BECKER_DEFENSE;
pub mod bishops_gambit;
pub use bishops_gambit::BISHOPS_GAMBIT;
pub mod blachly_gambit;
pub use blachly_gambit::BLACHLY_GAMBIT;
pub mod bonsch_osmolovsky_variation;
pub use bonsch_osmolovsky_variation::BONSCH_OSMOLOVSKY_VARIATION;
pub mod breyer_gambit;
pub use breyer_gambit::BREYER_GAMBIT;
pub mod carrera_gambit;
pub use carrera_gambit::CARRERA_GAMBIT;
pub mod cunningham_defense;
pub use cunningham_defense::CUNNINGHAM_DEFENSE;
pub mod dodo_variation;
pub use dodo_variation::DODO_VARIATION;
pub mod double_muzio_gambit;
pub use double_muzio_gambit::DOUBLE_MUZIO_GAMBIT;
pub mod eisenberg_variation;
pub use eisenberg_variation::EISENBERG_VARIATION;
pub mod fischer_defense;
pub use fischer_defense::FISCHER_DEFENSE;
pub mod gaga_gambit;
pub use gaga_gambit::GAGA_GAMBIT;
pub mod ghulam_kassim_gambit;
pub use ghulam_kassim_gambit::GHULAM_KASSIM_GAMBIT;
pub mod gianutio_countergambit;
pub use gianutio_countergambit::GIANUTIO_COUNTERGAMBIT;
pub mod greco_gambit;
pub use greco_gambit::GRECO_GAMBIT;
pub mod hanstein_gambit;
pub use hanstein_gambit::HANSTEIN_GAMBIT;
pub mod kieseritzky;
pub use kieseritzky::KIESERITZKY;
pub mod kieseritzky_gambit;
pub use kieseritzky_gambit::KIESERITZKY_GAMBIT;
pub mod kings_knights_gambit;
pub use kings_knights_gambit::KINGS_KNIGHTS_GAMBIT;
pub mod kotov_gambit;
pub use kotov_gambit::KOTOV_GAMBIT;
pub mod lolli_gambit;
pub use lolli_gambit::LOLLI_GAMBIT;
pub mod lopez_gianutio_countergambit;
pub use lopez_gianutio_countergambit::LOPEZ_GIANUTIO_COUNTERGAMBIT;
pub mod mac_leod_defense;
pub use mac_leod_defense::MAC_LEOD_DEFENSE;
pub mod mason_keres_gambit;
pub use mason_keres_gambit::MASON_KERES_GAMBIT;
pub mod mayet_gambit;
pub use mayet_gambit::MAYET_GAMBIT;
pub mod mc_donnell_gambit;
pub use mc_donnell_gambit::MC_DONNELL_GAMBIT;
pub mod middleton_countergambit;
pub use middleton_countergambit::MIDDLETON_COUNTERGAMBIT;
pub mod modern_defense;
pub use modern_defense::MODERN_DEFENSE;
pub mod muzio_gambit;
pub use muzio_gambit::MUZIO_GAMBIT;
pub mod muzio_gambit_accepted;
pub use muzio_gambit_accepted::MUZIO_GAMBIT_ACCEPTED;
pub mod orsini_gambit;
pub use orsini_gambit::ORSINI_GAMBIT;
pub mod paris_gambit;
pub use paris_gambit::PARIS_GAMBIT;
pub mod philidor_gambit;
pub use philidor_gambit::PHILIDOR_GAMBIT;
pub mod quaade_gambit;
pub use quaade_gambit::QUAADE_GAMBIT;
pub mod rosentreter_gambit;
pub use rosentreter_gambit::ROSENTRETER_GAMBIT;
pub mod salvio_gambit;
pub use salvio_gambit::SALVIO_GAMBIT;
pub mod schallopp_defense;
pub use schallopp_defense::SCHALLOPP_DEFENSE;
pub mod schurig_gambit;
pub use schurig_gambit::SCHURIG_GAMBIT;
pub mod silberschmidt_gambit;
pub use silberschmidt_gambit::SILBERSCHMIDT_GAMBIT;
pub mod stamma_gambit;
pub use stamma_gambit::STAMMA_GAMBIT;
pub mod sorensen_gambit;
pub use sorensen_gambit::SORENSEN_GAMBIT;
pub mod tartakower_gambit;
pub use tartakower_gambit::TARTAKOWER_GAMBIT;
pub mod traditional_variation;
pub use traditional_variation::TRADITIONAL_VARIATION;
pub mod tumbleweed;
pub use tumbleweed::TUMBLEWEED;
pub mod villemson_gambit;
pub use villemson_gambit::VILLEMSON_GAMBIT;
pub mod wagenbach_defense;
pub use wagenbach_defense::WAGENBACH_DEFENSE;
