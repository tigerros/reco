use pgn_reader::Skip;
use shakmaty::san::SanPlus;

pub struct SansPgnVisitor(pub Vec<SanPlus>);

impl pgn_reader::Visitor for SansPgnVisitor {
    type Result = ();

    fn san(&mut self, san_plus: SanPlus) {
        self.0.push(san_plus);
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn end_game(&mut self) -> Self::Result {
        ()
    }
}
