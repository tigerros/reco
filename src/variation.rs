use crate::Line;

/// A named variation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Variation {
    /// The name of this variation.
    ///
    /// This is not the full name, you'll need to look at all the parents to get that.
    pub name: &'static str,
    /// The variations of this variation.
    pub variations: &'static [&'static Self],
    /// The parent variation of this variation.
    ///
    /// [`None`] if this is a root variation.
    pub parent: Option<&'static Self>,
    /// A variation with the same name can contain multiple lines.
    ///
    /// For example, A05 Zukertort has two lines:
    /// - `1. Nf3 Nf6`
    /// - `1. Nf3 Nf6 2. Nc3 Nc6`
    pub lines: &'static [Line]
}