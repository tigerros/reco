use shakmaty::{Move, Setup};
use reco::Code;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct LineMeta {
    pub code: Code,
    pub moves: Vec<Move>,
    pub setup: Setup,
}