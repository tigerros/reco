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
# use reco::book::DUTCH_DEFENSE;
assert_eq!(DUTCH_DEFENSE.original_name(), "Dutch Defense");
```"#)]
pub static DUTCH_DEFENSE: Variation = Variation {
    name: "Dutch Defense",
    variations: &[&OMEGA_ISIS_GAMBIT,
&KREJCIK_GAMBIT,
&BLACKBURNE_VARIATION,
&LENINGRAD_VARIATION,
&LENINGRAD,
&RUBINSTEIN_VARIATION,
&STAUNTON_GAMBIT,
&KRAUSE_VARIATION,
&FIANCHETTO_VARIATION,
&JANZEN_KORCHNOI_GAMBIT,
&KORCHNOI_ATTACK,
&HEVENDEHL_GAMBIT,
&ALEKHINE_VARIATION,
&MANHATTAN_GAMBIT,
&BELLON_GAMBIT,
&HORT_ANTOSHIN_SYSTEM,
&STONEWALL_VARIATION,
&ALAPIN_VARIATION,
&SENECHAUD_GAMBIT,
&FIANCHETTO_ATTACK,
&STAUNTON_GAMBIT_ACCEPTED,
&SPIELMANN_GAMBIT,
&KINGFISHER_GAMBIT,
&SEMI_LENINGRAD_VARIATION,
&BLACKMARS_SECOND_GAMBIT,
&CLASSICAL_VARIATION,
&NORMAL_VARIATION,
&RAPHAEL_VARIATION,
&BLADEL_VARIATION,
&NIMZO_DUTCH_VARIATION,
&HOPTON_ATTACK,
&QUEENS_KNIGHT_VARIATION],
    parent: None,
    lines: &[Line {
    code: Code {
        volume: Volume::A,
        category: Category::new_static::<80>()
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
        role: Pawn,
        from: F7,
        capture: None,
        to: F5,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(62769057379710720),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18437455536917053440),
                white: Bitboard(134281215)
            }
        ),
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
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
},
Line {
    code: Code {
        volume: Volume::A,
        category: Category::new_static::<84>()
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
        role: Pawn,
        from: F7,
        capture: None,
        to: F5,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: C2,
        capture: None,
        to: C4,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(62769057446818560),
                knight: Bitboard(4755801206503243842),
                bishop: Bitboard(2594073385365405732),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(576460752303423496),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(18437455536917053440),
                white: Bitboard(201389055)
            }
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
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};pub mod omega_isis_gambit;
pub use omega_isis_gambit::OMEGA_ISIS_GAMBIT;
pub mod krejcik_gambit;
pub use krejcik_gambit::KREJCIK_GAMBIT;
pub mod blackburne_variation;
pub use blackburne_variation::BLACKBURNE_VARIATION;
pub mod leningrad_variation;
pub use leningrad_variation::LENINGRAD_VARIATION;
pub mod leningrad;
pub use leningrad::LENINGRAD;
pub mod rubinstein_variation;
pub use rubinstein_variation::RUBINSTEIN_VARIATION;
pub mod staunton_gambit;
pub use staunton_gambit::STAUNTON_GAMBIT;
pub mod krause_variation;
pub use krause_variation::KRAUSE_VARIATION;
pub mod fianchetto_variation;
pub use fianchetto_variation::FIANCHETTO_VARIATION;
pub mod janzen_korchnoi_gambit;
pub use janzen_korchnoi_gambit::JANZEN_KORCHNOI_GAMBIT;
pub mod korchnoi_attack;
pub use korchnoi_attack::KORCHNOI_ATTACK;
pub mod hevendehl_gambit;
pub use hevendehl_gambit::HEVENDEHL_GAMBIT;
pub mod alekhine_variation;
pub use alekhine_variation::ALEKHINE_VARIATION;
pub mod manhattan_gambit;
pub use manhattan_gambit::MANHATTAN_GAMBIT;
pub mod bellon_gambit;
pub use bellon_gambit::BELLON_GAMBIT;
pub mod hort_antoshin_system;
pub use hort_antoshin_system::HORT_ANTOSHIN_SYSTEM;
pub mod stonewall_variation;
pub use stonewall_variation::STONEWALL_VARIATION;
pub mod alapin_variation;
pub use alapin_variation::ALAPIN_VARIATION;
pub mod senechaud_gambit;
pub use senechaud_gambit::SENECHAUD_GAMBIT;
pub mod fianchetto_attack;
pub use fianchetto_attack::FIANCHETTO_ATTACK;
pub mod staunton_gambit_accepted;
pub use staunton_gambit_accepted::STAUNTON_GAMBIT_ACCEPTED;
pub mod spielmann_gambit;
pub use spielmann_gambit::SPIELMANN_GAMBIT;
pub mod kingfisher_gambit;
pub use kingfisher_gambit::KINGFISHER_GAMBIT;
pub mod semi_leningrad_variation;
pub use semi_leningrad_variation::SEMI_LENINGRAD_VARIATION;
pub mod blackmars_second_gambit;
pub use blackmars_second_gambit::BLACKMARS_SECOND_GAMBIT;
pub mod classical_variation;
pub use classical_variation::CLASSICAL_VARIATION;
pub mod normal_variation;
pub use normal_variation::NORMAL_VARIATION;
pub mod raphael_variation;
pub use raphael_variation::RAPHAEL_VARIATION;
pub mod bladel_variation;
pub use bladel_variation::BLADEL_VARIATION;
pub mod nimzo_dutch_variation;
pub use nimzo_dutch_variation::NIMZO_DUTCH_VARIATION;
pub mod hopton_attack;
pub use hopton_attack::HOPTON_ATTACK;
pub mod queens_knight_variation;
pub use queens_knight_variation::QUEENS_KNIGHT_VARIATION;
