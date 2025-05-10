use shakmaty::Move;
use shakmaty::san::SanPlus;
use shakmaty::uci::UciMove;
use crate::Code;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Opening<'a> {
    pub code: Code,
    /// The top-level name of the opening.
    pub name: &'a str,
    /// The variation of the opening.
    ///
    /// This is an array because there may be multiple variation layers.
    pub variation: &'a [&'a str],
    pub moves: &'a [Move],
    pub sans: &'a [SanPlus],
    pub uci: &'a [UciMove],
}