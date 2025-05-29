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
# use reco::book::KINGS_GAMBIT_ACCEPTED;
assert_eq!(KINGS_GAMBIT_ACCEPTED.original_name(), "King's Gambit Accepted");
```"#)]
pub static KINGS_GAMBIT_ACCEPTED: Variation = Variation {
    name: "King's Gambit Accepted",
    variations: &[&ORSINI_GAMBIT,
&ALLGAIER,
&MAC_LEOD_DEFENSE,
&BREYER_GAMBIT,
&LOPEZ_GIANUTIO_COUNTERGAMBIT,
&CARRERA_GAMBIT,
&GRECO_GAMBIT,
&SCHURIG_GAMBIT,
&TARTAKOWER_GAMBIT,
&BONSCH_OSMOLOVSKY_VARIATION,
&CUNNINGHAM_DEFENSE,
&AUSTRALIAN_GAMBIT,
&SORENSEN_GAMBIT,
&GAGA_GAMBIT,
&VILLEMSON_GAMBIT,
&HANSTEIN_GAMBIT,
&MC_DONNELL_GAMBIT,
&MUZIO_GAMBIT_ACCEPTED,
&ROSENTRETER_GAMBIT,
&MIDDLETON_COUNTERGAMBIT,
&QUAADE_GAMBIT,
&MAYET_GAMBIT,
&KOTOV_GAMBIT,
&TUMBLEWEED,
&DODO_VARIATION,
&BISHOPS_GAMBIT,
&PARIS_GAMBIT,
&MASON_KERES_GAMBIT,
&SILBERSCHMIDT_GAMBIT,
&KIESERITZKY,
&BLACHLY_GAMBIT,
&FISCHER_DEFENSE,
&GHULAM_KASSIM_GAMBIT,
&DOUBLE_MUZIO_GAMBIT,
&BASMAN_GAMBIT,
&LOLLI_GAMBIT,
&MUZIO_GAMBIT,
&BECKER_DEFENSE,
&GIANUTIO_COUNTERGAMBIT,
&TRADITIONAL_VARIATION,
&SALVIO_GAMBIT,
&EISENBERG_VARIATION,
&MODERN_DEFENSE,
&SCHALLOPP_DEFENSE,
&STAMMA_GAMBIT,
&PHILIDOR_GAMBIT,
&KINGS_KNIGHTS_GAMBIT,
&WAGENBACH_DEFENSE,
&ABBAZIA_DEFENSE,
&ALLGAIER_GAMBIT,
&KIESERITZKY_GAMBIT],
    parent: None,
    lines: &[Line {
    code: Code {
        volume: Volume::C,
        category: Category::new_static::<33>()
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
        capture: Some(
            Pawn,
        ),
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
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18441958999642341376),
                white: Bitboard(268488703)
            }
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
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod orsini_gambit;
pub use orsini_gambit::ORSINI_GAMBIT;
pub mod allgaier;
pub use allgaier::ALLGAIER;
pub mod mac_leod_defense;
pub use mac_leod_defense::MAC_LEOD_DEFENSE;
pub mod breyer_gambit;
pub use breyer_gambit::BREYER_GAMBIT;
pub mod lopez_gianutio_countergambit;
pub use lopez_gianutio_countergambit::LOPEZ_GIANUTIO_COUNTERGAMBIT;
pub mod carrera_gambit;
pub use carrera_gambit::CARRERA_GAMBIT;
pub mod greco_gambit;
pub use greco_gambit::GRECO_GAMBIT;
pub mod schurig_gambit;
pub use schurig_gambit::SCHURIG_GAMBIT;
pub mod tartakower_gambit;
pub use tartakower_gambit::TARTAKOWER_GAMBIT;
pub mod bonsch_osmolovsky_variation;
pub use bonsch_osmolovsky_variation::BONSCH_OSMOLOVSKY_VARIATION;
pub mod cunningham_defense;
pub use cunningham_defense::CUNNINGHAM_DEFENSE;
pub mod australian_gambit;
pub use australian_gambit::AUSTRALIAN_GAMBIT;
pub mod sorensen_gambit;
pub use sorensen_gambit::SORENSEN_GAMBIT;
pub mod gaga_gambit;
pub use gaga_gambit::GAGA_GAMBIT;
pub mod villemson_gambit;
pub use villemson_gambit::VILLEMSON_GAMBIT;
pub mod hanstein_gambit;
pub use hanstein_gambit::HANSTEIN_GAMBIT;
pub mod mc_donnell_gambit;
pub use mc_donnell_gambit::MC_DONNELL_GAMBIT;
pub mod muzio_gambit_accepted;
pub use muzio_gambit_accepted::MUZIO_GAMBIT_ACCEPTED;
pub mod rosentreter_gambit;
pub use rosentreter_gambit::ROSENTRETER_GAMBIT;
pub mod middleton_countergambit;
pub use middleton_countergambit::MIDDLETON_COUNTERGAMBIT;
pub mod quaade_gambit;
pub use quaade_gambit::QUAADE_GAMBIT;
pub mod mayet_gambit;
pub use mayet_gambit::MAYET_GAMBIT;
pub mod kotov_gambit;
pub use kotov_gambit::KOTOV_GAMBIT;
pub mod tumbleweed;
pub use tumbleweed::TUMBLEWEED;
pub mod dodo_variation;
pub use dodo_variation::DODO_VARIATION;
pub mod bishops_gambit;
pub use bishops_gambit::BISHOPS_GAMBIT;
pub mod paris_gambit;
pub use paris_gambit::PARIS_GAMBIT;
pub mod mason_keres_gambit;
pub use mason_keres_gambit::MASON_KERES_GAMBIT;
pub mod silberschmidt_gambit;
pub use silberschmidt_gambit::SILBERSCHMIDT_GAMBIT;
pub mod kieseritzky;
pub use kieseritzky::KIESERITZKY;
pub mod blachly_gambit;
pub use blachly_gambit::BLACHLY_GAMBIT;
pub mod fischer_defense;
pub use fischer_defense::FISCHER_DEFENSE;
pub mod ghulam_kassim_gambit;
pub use ghulam_kassim_gambit::GHULAM_KASSIM_GAMBIT;
pub mod double_muzio_gambit;
pub use double_muzio_gambit::DOUBLE_MUZIO_GAMBIT;
pub mod basman_gambit;
pub use basman_gambit::BASMAN_GAMBIT;
pub mod lolli_gambit;
pub use lolli_gambit::LOLLI_GAMBIT;
pub mod muzio_gambit;
pub use muzio_gambit::MUZIO_GAMBIT;
pub mod becker_defense;
pub use becker_defense::BECKER_DEFENSE;
pub mod gianutio_countergambit;
pub use gianutio_countergambit::GIANUTIO_COUNTERGAMBIT;
pub mod traditional_variation;
pub use traditional_variation::TRADITIONAL_VARIATION;
pub mod salvio_gambit;
pub use salvio_gambit::SALVIO_GAMBIT;
pub mod eisenberg_variation;
pub use eisenberg_variation::EISENBERG_VARIATION;
pub mod modern_defense;
pub use modern_defense::MODERN_DEFENSE;
pub mod schallopp_defense;
pub use schallopp_defense::SCHALLOPP_DEFENSE;
pub mod stamma_gambit;
pub use stamma_gambit::STAMMA_GAMBIT;
pub mod philidor_gambit;
pub use philidor_gambit::PHILIDOR_GAMBIT;
pub mod kings_knights_gambit;
pub use kings_knights_gambit::KINGS_KNIGHTS_GAMBIT;
pub mod wagenbach_defense;
pub use wagenbach_defense::WAGENBACH_DEFENSE;
pub mod abbazia_defense;
pub use abbazia_defense::ABBAZIA_DEFENSE;
pub mod allgaier_gambit;
pub use allgaier_gambit::ALLGAIER_GAMBIT;
pub mod kieseritzky_gambit;
pub use kieseritzky_gambit::KIESERITZKY_GAMBIT;
