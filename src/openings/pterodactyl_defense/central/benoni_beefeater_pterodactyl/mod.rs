#[allow(unused_imports, clippy::enum_glob_use, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Move::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Role::{Pawn, Knight, Bishop, Rook, Queen, King};
#[allow(clippy::enum_glob_use, reason = "there's 64 variants in this enum, importing them all is stupid")]
use shakmaty::Square::*;
#[allow(unused_imports, reason = "because the code is generated, we don't know if it's going to be used")]
use shakmaty::Color::{Black, White};
use shakmaty::bitboard::Bitboard;
use shakmaty::board::Board;
use shakmaty::{ByRole, ByColor, Setup};
use core::num::NonZeroU32;
use crate::{Opening, Code, Volume};
use deranged::RangedU8;
use core::panic;

/// Pterodactyl Defense: Central, Benoni Beefeater Pterodactyl.
pub const BENONI_BEEFEATER_PTERODACTYL: [Opening<'static, &str>; 1] = [Opening {
    code: Code {
        volume: Volume::A,
        subcategory: RangedU8::new_static::<40>(),
    },
    name: "Pterodactyl Defense",
    variation: &["Central", "Benoni Beefeater Pterodactyl"],
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
        from: G7,
        capture: None,
        to: G6,
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
        role: Bishop,
        from: F8,
        capture: None,
        to: G7,
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
        from: D7,
        capture: None,
        to: D6,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: E2,
        capture: None,
        to: E4,
        promotion: None,
    },
    Normal {
        role: Bishop,
        from: G7,
        capture: Some(
            Knight,
        ),
        to: C3,
        promotion: None,
    },
    Normal {
        role: Pawn,
        from: B2,
        capture: Some(
            Bishop,
        ),
        to: C3,
        promotion: None,
    },
    Normal {
        role: Queen,
        from: D8,
        capture: None,
        to: A5,
        promotion: None,
    },
],
    setup: &Setup {
        board: Board::from_bitboards(
            ByRole {
                pawn: Bitboard(50463237543878912),
                knight: Bitboard(4755801206503243840),
                bishop: Bitboard(288230376151711780),
                rook: Bitboard(9295429630892703873),
                queen: Bitboard(4294967304),
                king: Bitboard(1152921504606846992)
            },
            ByColor {
                black: Bitboard(15542845925297750016),
                white: Bitboard(34695602685)
            }
        ),
        promoted: Bitboard(0),
        pockets: None,
        turn: White,
        castling_rights: Bitboard(9295429630892703873),
        ep_square: None,
        remaining_checks: None,
        halfmoves: 1,
        fullmoves: if let Some(fullmoves) = NonZeroU32::new(7) { fullmoves } else { panic!("fullmoves is zero") },
    },
}];