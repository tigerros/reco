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
use crate::{Variation, Line, Code, Volume, Category};
use core::panic;pub static KING_S_GAMBIT_ACCEPTED: Variation = Variation {
        name: King's Gambit Accepted,
        parent: None,
        variations: &[&salvio_gambit,
&philidor_gambit,
&cunningham_defense,
&blachly_gambit,
&greco_gambit,
&wagenbach_defense,
&tartakower_gambit,
&lolli_gambit,
&basman_gambit,
&schurig_gambit,
&bishop_s_gambit,
&lopez_gianutio_countergambit,
&fischer_defense,
&king_s_knight_s_gambit,
&muzio_gambit,
&mayet_gambit,
&dodo_variation,
&traditional_variation,
&mc_donnell_gambit,
&becker_defense,
&abbazia_defense,
&silberschmidt_gambit,
&eisenberg_variation,
&kotov_gambit,
&rosentreter_gambit,
&allgaier,
&kieseritzky_gambit,
&breyer_gambit,
&gianutio_countergambit,
&gaga_gambit,
&modern_defense,
&carrera_gambit,
&kieseritzky,
&paris_gambit,
&middleton_countergambit,
&stamma_gambit,
&mac_leod_defense,
&orsini_gambit,
&villemson_gambit,
&bonsch_osmolovsky_variation,
&double_muzio_gambit,
&ghulam_kassim_gambit,
&hanstein_gambit,
&australian_gambit,
&muzio_gambit_accepted,
&sorensen_gambit,
&allgaier_gambit,
&tumbleweed,
&mason_keres_gambit,
&quaade_gambit,
&schallopp_defense],
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
                fullmoves: if let Some(fullmoves) = NonZeroU32::new(3) { fullmoves } else { unreachable!() }
            }
        }]
    }