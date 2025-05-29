use reco::Code;
use shakmaty::{Move, Setup};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct LineMeta {
    pub code: Code,
    pub moves: Vec<Move>,
    pub setup: Setup,
}
