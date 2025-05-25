use crate::Code;
use crate::LineMeta;
use shakmaty::Setup;

/// Returns a string that represents an expression of the given line.
pub fn get_line_expression_string(line: &LineMeta) -> String {
    let Code { volume, category } = line.code;
    let moves = &line.moves;
    let Setup {
        board,
        promoted,
        pockets,
        turn,
        castling_rights,
        ep_square,
        remaining_checks,
        halfmoves,
        fullmoves,
    } = &line.setup;
    let promoted = promoted.0;
    let castling_rights = castling_rights.0;
    let (by_role_bitboard, by_color_bitboard) = board.clone().into_bitboards();

    format!(
        r#"Line {{
            code: Code {{
                volume: Volume::{volume},
                category: Category::new_static::<{category}>()
            }},
            moves: &{moves:#?},
            setup: Setup {{
                board: Board::from_bitboards(
                    ByRole {{
                        pawn: Bitboard({}),
                        knight: Bitboard({}),
                        bishop: Bitboard({}),
                        rook: Bitboard({}),
                        queen: Bitboard({}),
                        king: Bitboard({})
                    }},
                    ByColor {{
                        black: Bitboard({}),
                        white: Bitboard({})
                    }}
                ),
                promoted: Bitboard({promoted}),
                pockets: {pockets:#?},
                turn: {turn:#?},
                castling_rights: Bitboard({castling_rights}),
                ep_square: {ep_square:#?},
                remaining_checks: {remaining_checks:#?},
                halfmoves: {halfmoves},
                fullmoves: if let Some(fullmoves) = NonZeroU32::new({fullmoves}) {{ fullmoves }} else {{ unreachable!() }}
            }}
        }}"#,
        by_role_bitboard.pawn.0,
        by_role_bitboard.knight.0,
        by_role_bitboard.bishop.0,
        by_role_bitboard.rook.0,
        by_role_bitboard.queen.0,
        by_role_bitboard.king.0,
        by_color_bitboard.black.0,
        by_color_bitboard.white.0
    )
}
