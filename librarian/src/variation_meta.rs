use crate::LineMeta;
use deunicode::deunicode;
use heck::{ToShoutySnekCase, ToSnekCase};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;

pub struct VariationMeta {
    pub name: Rc<str>,
    pub variations: RefCell<HashMap<Rc<str>, Rc<Self>>>,
    pub parent: Option<Rc<Self>>,
    pub lines: RefCell<HashSet<LineMeta>>,
}

impl VariationMeta {
    /// Fetches the names of all the parents and turns them into identifiers (see [`Self::snek_name`]).
    ///
    /// For example, `Sicilian Defense: Najdorf, Classical` will return
    /// `["sicilian_defense", "najdorf", "classical"]`.
    pub fn full_snek_name(&self) -> VecDeque<String> {
        let mut full_name = VecDeque::with_capacity(10);

        full_name.push_front(self.snek_name());

        let mut parent = self.parent.clone();

        while let Some(current) = parent {
            full_name.push_front(current.snek_name());
            parent = current.parent.clone();
        }

        full_name
    }

    /// Deunicodes the name and removes `'`, because `Queen's Gambit` should be `QUEENS_GAMBIT`,
    /// not `QUEEN_S_GAMBIT`.
    pub fn identifier(&self) -> String {
        deunicode(&self.name).replace('\'', "")
    }

    /// Returns the lower_snek_case identifier of the name.
    ///
    /// If the identifier starts with a number, the `n` character is prepended.
    pub fn snek_name(&self) -> String {
        let mut identifier = self.identifier().to_snek_case();

        if identifier.starts_with(char::is_numeric) {
            identifier.insert(0, 'n');
        }

        identifier
    }

    #[expect(non_snake_case, reason = "relevant")]
    /// Returns the SHOUTY_SNEK_CASE identifier of the name.
    ///
    /// If the identifier starts with a number, the `N` character is prepended.
    pub fn SNEK_NAME(&self) -> String {
        let mut identifier = self.identifier().TO_SHOUTY_SNEK_CASE();

        if identifier.starts_with(char::is_numeric) {
            identifier.insert(0, 'N');
        }

        identifier
    }

    /// Gets the original name, as seen in [lichess-org/chess-openings](https://github.com/lichess-org/chess-openings).
    ///
    /// For example, `"Sicilian Defense: Najdorf Variation, Main Line"`.
    pub fn original_name(&self) -> String {
        let mut names = Vec::with_capacity(10);

        names.insert(0, self.name.clone());

        let mut parent = &self.parent;

        while let Some(current) = &parent {
            names.insert(0, current.name.clone());
            parent = &current.parent;
        }

        let root = names.remove(0);
        let non_root_joined = names.join(", ");

        format!(
            "{root}{}",
            if non_root_joined.is_empty() {
                String::new()
            } else {
                format!(": {non_root_joined}")
            }
        )
    }

    /// Returns a string that represents the item of this variation.
    pub fn item_string(&self) -> String {
        let original_name = self.original_name();
        let identifier = self.SNEK_NAME();
        let name = &self.name;

        let parent = if let Some(parent) = &self.parent {
            format!("Some(&super::{})", parent.SNEK_NAME())
        } else {
            "None".to_string()
        };

        let variations = self
            .variations
            .borrow()
            .iter()
            .map(|(name, variation)| {
                assert_eq!(name, &variation.name);

                format!("&{}", variation.SNEK_NAME())
            })
            .collect::<Vec<_>>()
            .join(",\n");

        let lines = self
            .lines
            .borrow()
            .iter()
            .map(|line| line.expression_string(&identifier))
            .collect::<Vec<_>>()
            .join(",\n");

        // Get the module path
        let path = self
            .full_snek_name()
            .into_iter()
            .rev()
            .skip(1)
            .rev()
            .map(|s| format!("{s}::"))
            .collect::<String>();

        format!(
            r##"#[cfg_attr(feature = "alloc", doc = r#"```rust
# use reco::book::{path}{identifier};
assert_eq!({identifier}.original_name(), "{original_name}");
```"#)]
pub static {identifier}: Variation = Variation {{
    name: "{name}",
    parent: {parent},
    variations: &[{variations}],
    lines: &[{lines}]
}};"##
        )
    }
}
