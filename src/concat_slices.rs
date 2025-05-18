use core::mem::MaybeUninit;

/// Soundly concatenates all items in the given slices.
/// References items, because we don't assume [`T`] is [`Copy`].
///
/// **Only run this at compile time.**
pub const fn concat_slices<'a, const LEN: usize, T: 'a>(slices: &'a [&'a [T]]) -> [&'a T; LEN] {
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
            assert!(out_i < LEN, "LEN lesser than length of all items");

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

    // Index should normally not be equal to the length
    // but remember that after the last item (index) is processed,
    // we still increment out_i.
    assert!(out_i == LEN, "LEN greater than length of all items");

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
    unsafe {
        core::mem::transmute_copy(&out)
    }
}

#[cfg(test)]
mod tests {
    use crate::concat_slices;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn empty() {
        let mut slices = Vec::<&[usize]>::with_capacity(1000);

        for _ in 0..1000 {
            assert_eq!(concat_slices::<0, usize>(slices.as_slice()), [&0; 0]);
            slices.push(&[]);
        }
    }

    #[test]
    fn equal_lengths() {
        assert_eq!(
            concat_slices::<6, usize>(&[&[1, 2], &[3, 4], &[5, 6]]),
            [&1, &2, &3, &4, &5, &6]
        );
    }

    #[test]
    fn different_lengths() {
        let mut slices = Vec::<&[usize]>::with_capacity(1 + 6 + 2 + 1 + 3);

        for _ in 0..1 {
            slices.push(&[]);
        }

        assert_eq!(concat_slices::<0, usize>(slices.as_slice()), [&0; 0]);
        slices.push(&[1, 2]);

        for _ in 0..6 {
            slices.push(&[]);
        }

        assert_eq!(concat_slices::<2, usize>(slices.as_slice()), [&1, &2]);
        slices.push(&[3]);

        for _ in 0..2 {
            slices.push(&[]);
        }

        assert_eq!(
            concat_slices::<{ 2 + 1 }, usize>(slices.as_slice()),
            [&1, &2, &3]
        );
        slices.push(&[4, 5, 6]);
        assert_eq!(
            concat_slices::<{ 2 + 1 + 3 }, usize>(slices.as_slice()),
            [&1, &2, &3, &4, &5, &6]
        );
    }

    #[test]
    #[should_panic = "specified LEN is greater than the length of all items"]
    fn len_greater_than_length_of_all_items() {
        concat_slices::<5, usize>(vec![&[4][..], &[2, 1][..]].as_slice());
    }

    #[test]
    #[should_panic = "specified LEN is lesser than the length of all items"]
    fn len_lesser_than_length_of_all_items() {
        concat_slices::<1, usize>(vec![&[4][..], &[2, 1][..]].as_slice());
    }
}
