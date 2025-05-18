use core::error::Error;
use core::fmt::Display;
use core::mem::MaybeUninit;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ConcatError {
    LenGreaterThanSum,
    LenLesserThanSum,
}

impl ConcatError {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LenGreaterThanSum => "LEN greater than length of all items",
            Self::LenLesserThanSum => "LEN lesser than length of all items",
        }
    }
}

impl Display for ConcatError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Error for ConcatError {}

/// Soundly concatenates all items in the given slices.
/// References items, because we don't assume [`T`] is [`Copy`].
///
/// # Errors
/// Errors if the length of all items does not match [`LEN`].
pub const fn concat_slices<'a, const LEN: usize, T: 'a>(
    slices: &'a [&'a [T]],
) -> Result<[&'a T; LEN], ConcatError> {
    let mut out: [MaybeUninit<&'a T>; LEN] = [MaybeUninit::uninit(); LEN];
    let mut out_i = 0;
    let mut slice_i = 0;

    while slice_i < slices.len() {
        #[expect(
            clippy::indexing_slicing,
            reason = "indexes are checked, indexing is necessary because this is a const fn"
        )]
        let slice = slices[slice_i];
        let mut item_i = 0;

        while item_i < slice.len() {
            // We reached LEN but there's another item to process
            if out_i == LEN {
                return Err(ConcatError::LenLesserThanSum);
            }

            #[expect(
                clippy::indexing_slicing,
                reason = "indexes are checked, indexing is necessary because this is a const fn"
            )]
            out[out_i].write(&slice[item_i]);
            #[expect(clippy::unwrap_used, reason = "only runs in const context")]
            {
                out_i = out_i.checked_add(1).unwrap();
            }
            #[expect(clippy::unwrap_used, reason = "only runs in const context")]
            {
                item_i = item_i.checked_add(1).unwrap();
            }
        }

        #[expect(clippy::unwrap_used, reason = "only runs in const context")]
        {
            slice_i = slice_i.checked_add(1).unwrap();
        }
    }

    // Index should normally not be equal to the length so you might think that <=
    // would be applicable here, but remember that after the last item (index) is processed,
    // we still increment out_i.
    if out_i < LEN {
        return Err(ConcatError::LenGreaterThanSum);
    }

    // SAFETY:
    // MaybeUninit<T> has the same size, alignment, and ABI as T.
    // <https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#layout-1>
    //
    // If all MaybeUninits are initialized then it is safe to
    // transmute a [MaybeUninit<T>; N] to [T; N].
    //
    // All items are initialized because we assert that out_i is exactly LEN.
    // Each out_i (except the last out_i value) means that the MaybeUninit at that index is initialized.
    //
    // Similar to:
    // <https://doc.rust-lang.org/core/mem/union.MaybeUninit.html#initializing-an-array-element-by-element>
    //
    // `transmute_copy` is necessary to ignore the size check.
    // The Rust compiler considers `[T; LEN]` a "dependent" type because of `LEN`
    // and doesn't try to verify it's the same size as `[U; LEN]`.
    #[expect(unsafe_code, reason = "see comment")]
    Ok(unsafe { core::mem::transmute_copy(&out) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn empty() {
        let mut slices = Vec::<&[usize]>::with_capacity(1000);

        for _ in 0..1000 {
            assert_eq!(concat_slices::<0, usize>(slices.as_slice()), Ok([]));
            slices.push(&[]);
        }
    }

    #[test]
    fn equal_lengths() {
        assert_eq!(
            concat_slices::<6, usize>(&[&[1, 2], &[3, 4], &[5, 6]]),
            Ok([&1, &2, &3, &4, &5, &6])
        );
    }

    #[test]
    fn different_lengths() {
        let mut slices = Vec::<&[usize]>::with_capacity(1 + 6 + 2 + 1 + 3);

        for _ in 0..1 {
            slices.push(&[]);
        }

        assert_eq!(concat_slices::<0, usize>(slices.as_slice()), Ok([]));
        slices.push(&[1, 2]);

        for _ in 0..6 {
            slices.push(&[]);
        }

        assert_eq!(concat_slices::<2, usize>(slices.as_slice()), Ok([&1, &2]));
        slices.push(&[3]);

        for _ in 0..2 {
            slices.push(&[]);
        }

        assert_eq!(
            concat_slices::<{ 2 + 1 }, usize>(slices.as_slice()),
            Ok([&1, &2, &3])
        );
        slices.push(&[4, 5, 6]);
        assert_eq!(
            concat_slices::<{ 2 + 1 + 3 }, usize>(slices.as_slice()),
            Ok([&1, &2, &3, &4, &5, &6])
        );
    }
}
