use crate::Code;
use core::fmt::Debug;
use shakmaty::{Move, Setup};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Line {
    /// The ECO code of the line.
    pub code: Code,
    /// The moves of the line.
    pub moves: &'static [Move],
    /// The position that occurs after the last move in [`Self.moves`](Self#structfield.moves) is played.
    pub setup: Setup,
}

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::Volume;
//     use alloc::string::{String, ToString};
//     use alloc::{format, vec};
//     use deranged::RangedU8;
//     use shakmaty::Square;
//
//     /// Returns the same opening but with different name types and mixed [`Cow`] variants.
//     fn opening() -> (
//         Opening<'static, String>,
//         Opening<'static, Cow<'static, str>>,
//     ) {
//         const SETUP: &Setup = &Setup::initial();
//
//         (
//             Opening {
//                 code: Code {
//                     volume: Volume::A,
//                     category: RangedU8::new_static::<67>(),
//                 },
//                 name: Cow::Owned(vec!["Grünfeld".to_string(), "Smyslov".to_string()]),
//                 moves: Cow::Borrowed(&[Move::Castle {
//                     king: Square::A2,
//                     rook: Square::H8,
//                 }]),
//                 setup: Cow::Owned(Setup::initial()),
//             },
//             Opening {
//                 code: Code {
//                     volume: Volume::A,
//                     category: RangedU8::new_static::<67>(),
//                 },
//                 name: Cow::Borrowed(&[Cow::Borrowed("Grünfeld"), Cow::Borrowed("Smyslov")]),
//                 moves: Cow::Owned(vec![Move::Castle {
//                     king: Square::A2,
//                     rook: Square::H8,
//                 }]),
//                 setup: Cow::<'static, Setup>::Borrowed(SETUP),
//             },
//         )
//     }
//
//     #[test]
//     fn eq() {
//         assert_eq!(opening().0, opening().1);
//     }
//
//     #[test]
//     fn debug() {
//         assert_eq!(
//             format!("{:#?}", opening().0),
//             r#"Opening {
//     code: Code {
//         volume: A,
//         category: 67,
//     },
//     name: [
//         "Grünfeld",
//         "Smyslov",
//     ],
//     moves: [
//         Castle {
//             king: A2,
//             rook: H8,
//         },
//     ],
//     setup: Setup {
//         board: r n b q k b n r
//         p p p p p p p p
//         . . . . . . . .
//         . . . . . . . .
//         . . . . . . . .
//         . . . . . . . .
//         P P P P P P P P
//         R N B Q K B N R
//         ,
//         promoted: . . . . . . . .
//         . . . . . . . .
//         . . . . . . . .
//         . . . . . . . .
//         . . . . . . . .
//         . . . . . . . .
//         . . . . . . . .
//         . . . . . . . .
//         ,
//         pockets: None,
//         turn: White,
//         castling_rights: 1 . . . . . . 1
//         . . . . . . . .
//         . . . . . . . .
//         . . . . . . . .
//         . . . . . . . .
//         . . . . . . . .
//         . . . . . . . .
//         1 . . . . . . 1
//         ,
//         ep_square: None,
//         remaining_checks: None,
//         halfmoves: 0,
//         fullmoves: 1,
//     },
// }"#
//         );
//     }
// }
