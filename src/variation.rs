use crate::Line;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::format;
#[cfg(feature = "alloc")]
use alloc::collections::vec_deque::VecDeque;

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
    pub lines: &'static [Line],
}

impl Variation {
    #[cfg(feature = "alloc")]
    /// Gets the original name, as seen in [lichess-org/chess-openings](https://github.com/lichess-org/chess-openings).
    ///
    /// For example, `"Sicilian Defense: Najdorf Variation, Main Line"`.
    pub fn original_name(&self) -> String {
        let mut names = VecDeque::with_capacity(10);

        names.push_front(self.name);

        let mut parent = &self.parent;

        while let Some(current) = &parent {
            names.push_front(current.name);
            parent = &current.parent;
        }

        #[expect(unsafe_code, reason = "we push the name of this variation, it will have 1 element")]
        let root = unsafe { names.remove(0).unwrap_unchecked() };
        let names = names.make_contiguous();
        let non_root_joined = names.join(", ");

        format!("{root}{}", if non_root_joined.is_empty() {
            String::new()
        } else {
            format!(": {non_root_joined}")
        })
    }
}