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
# use reco::book::caro_kann_defense::advance_variation::van_der_wiel_attack::DREYEV_DEFENSE;
assert_eq!(DREYEV_DEFENSE.original_name(), "Caro-Kann Defense: Advance Variation, Van der Wiel Attack, Dreyev Defense");
```"#)]
pub static DREYEV_DEFENSE: Variation = Variation {
    name: "Dreyev Defense",
    variations: &[],
    parent: Some(&super::VAN_DER_WIEL_ATTACK),
    lines: &[Line {
    code: Code {
        volume: Volume::B,
        category: Category::new_static::<12>()
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
        from: C7,
        capture: None,
        to: C6,
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
        from: D7,
        capture: None,
        to: D5,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E4,
        capture: None,
        to: E5,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: C8,
        capture: None,
        to: F5,
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
        role: Queen,
        from: D8,
        capture: None,
        to: B6,
        promotion: None,
    },
],
    setup: Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(68402920600692480),
                knight: Bitboard(4755801206503505984),
                bishop: Bitboard(2305843146652647460),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(2199023255560),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(17578400539425636352),
                white: Bitboard(68854015997)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 3,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(5) {
            fullmoves
        } else {
            #[expect(clippy::unreachable, reason = "intentional. It's in a const expression")]
            { unreachable!() }
        }
    }
}]
};