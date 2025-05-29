use crate::Line;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[expect(
    missing_copy_implementations,
    reason = "copy semantics conflict with the tree structure"
)]
/// A named variation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variation {
    pub(crate) name: &'static str,
    pub(crate) variations: &'static [&'static Self],
    pub(crate) parent: Option<&'static Self>,
    pub(crate) lines: &'static [Line],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// See [`Variation::validity`].
pub enum ValidityError {
    /// `self` is not a root variation (it has a parent).
    NonRoot,
    /// A variation has a parent, but an incorrect one.
    IncorrectParent(&'static Variation),
    /// A variation does not have a parent.
    MissingParent(&'static Variation),
}

impl Variation {
    /// The name of this variation.
    ///
    /// This is not the full name, you'll need to look at all the parents to get that.
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// The variations of this variation.
    pub const fn variations(&self) -> &'static [&'static Self] {
        self.variations
    }

    /// The parent variation of this variation.
    ///
    /// [`None`] if this is a root variation.
    pub const fn parent(&self) -> Option<&'static Self> {
        self.parent
    }

    /// A variation with the same name can contain multiple lines.
    ///
    /// For example, A05 Zukertort has two lines:
    /// - `1. Nf3 Nf6`
    /// - `1. Nf3 Nf6 2. Nc3 Nc6`
    pub const fn lines(&self) -> &'static [Line] {
        self.lines
    }

    /// Finds the root of this variation.
    ///
    /// Returns `self` if `self` is a root variation.
    pub const fn root(&self) -> &Self {
        let Some(mut parent) = self.parent else {
            return &self;
        };

        while let Some(grandparent) = parent.parent {
            parent = grandparent;
        }

        parent
    }

    /// Like [`Self::validity`], but doesn't error on a non-root variation.
    ///
    /// [`ValidityError::NonRoot`] is guaranteed not to occur.
    fn non_root_validity(&self) -> Result<(), ValidityError> {
        for variation in self.variations {
            let Some(parent) = variation.parent else {
                return Err(ValidityError::MissingParent(variation));
            };

            if !core::ptr::eq(parent, self) {
                return Err(ValidityError::IncorrectParent(variation));
            }

            variation.non_root_validity()?;
        }

        Ok(())
    }

    /// Recursively checks if [`Self::variations`] all have `self` as the parent.
    /// Can only be called on a root variation.
    ///
    /// # Errors
    /// See [`ValidityError`].
    pub fn validity(&self) -> Result<(), ValidityError> {
        if self.parent.is_some() {
            return Err(ValidityError::NonRoot);
        }

        self.non_root_validity()?;

        Ok(())
    }

    /// Like [`Self::walk`], but also walks `self` initially.
    pub fn walk_with_self<F, T>(&'static self, walker: &mut F) -> Option<T>
    where
        F: FnMut(&'static Variation) -> Option<T>,
    {
        walker(self)?;

        self.walk(walker)?;

        None
    }

    /// Recursively walks [`Self::variations`], stopping if the
    /// `walker` returns [`Some`].
    ///
    /// First, it walks a variation and then it's subvariations.
    ///
    /// See also [`Self::walk_with_self`].
    pub fn walk<F, T>(&self, walker: &mut F) -> Option<T>
    where
        F: FnMut(&'static Variation) -> Option<T>,
    {
        for variation in self.variations {
            walker(variation)?;
            variation.walk(walker)?;
        }

        None
    }

    #[cfg(feature = "alloc")]
    /// Gets the original name, as seen in [lichess-org/chess-openings](https://github.com/lichess-org/chess-openings).
    ///
    /// For example, `"Sicilian Defense: Najdorf Variation, Main Line"`.
    ///
    /// If `self` is a root variation, [`Self::name`] is returned and nothing is allocated.
    pub fn original_name(&self) -> Cow<'static, str> {
        let Some(mut parent) = self.parent else {
            return Cow::Borrowed(self.name);
        };

        let mut names = Vec::with_capacity(10);

        names.push(self.name);
        names.push(parent.name);

        while let Some(grandparent) = parent.parent {
            names.push(grandparent.name);
            parent = grandparent;
        }

        let mut name = String::with_capacity((self.name.len() + parent.name.len()) * 3);

        #[expect(unsafe_code, reason = "names is guaranteed to have self.name and parent.name")]
        name.push_str(unsafe { names.get_unchecked(names.len() - 1) });
        name.push_str(": ");

        let mut i = names.len() - 2;

        loop {
            #[expect(unsafe_code, reason = "i is within bounds")]
            name.push_str(unsafe { names.get_unchecked(i) });

            if i == 0 {
                break;
            }

            name.push_str(", ");

            i -= 1;
        }

        Cow::Owned(name)
    }
}
