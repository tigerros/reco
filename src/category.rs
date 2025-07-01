use core::ops::{Deref, DerefMut};
use deranged::RangedU8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The 0-99 category of an opening.
pub struct Category(pub RangedU8<0, 99>);

impl Category {
    /// An array of all possible categories in ascending order.
    pub const ALL: [Self; 100] = {
        let mut all = [Self(RangedU8::new_static::<0>()); 100];

        let mut i = 0u8;

        while i < 100 {
            #[expect(
                clippy::indexing_slicing,
                clippy::as_conversions,
                reason = "const expr"
            )]
            {
                all[i as usize] = Self(RangedU8::new(i).unwrap());
            }

            i += 1;
        }

        all
    };
}

impl From<RangedU8<0, 99>> for Category {
    fn from(value: RangedU8<0, 99>) -> Self {
        Self(value)
    }
}

impl From<Category> for RangedU8<0, 99> {
    fn from(value: Category) -> Self {
        value.0
    }
}

impl Deref for Category {
    type Target = RangedU8<0, 99>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Category {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for Category {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        #[expect(clippy::unwrap_used, reason = "integer is in range")]
        Ok(RangedU8::new(u.int_in_range(0..=99)?).unwrap().into())
    }
}

#[cfg(feature = "proptest")]
impl proptest::arbitrary::Arbitrary for Category {
    type Parameters = ();
    type Strategy = proptest::strategy::Map<core::ops::RangeInclusive<u8>, fn(u8) -> Self>;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        use proptest::strategy::Strategy;

        #[expect(clippy::unwrap_used, reason = "integer is in range")]
        (0..=99).prop_map(|n| Self(RangedU8::new(n).unwrap()))
    }
}
