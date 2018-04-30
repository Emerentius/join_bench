macro_rules! spezialize_for_lengths {
    ($separator:expr, $target:expr, $iter:expr; $($num:expr),*) => {
        let mut target = $target;
        let iter = $iter;
        let sep_len = $separator.len();
        let sep_bytes = $separator.as_bytes();
        match $separator.len() {
            $(
                // loops with hardcoded sizes run much faster
                // specialize the cases with small separator lengths
                $num => {
                    for s in iter {
                        target.get_unchecked_mut(..$num)
                            .copy_from_slice(sep_bytes);

                        let s_bytes = s.borrow().as_bytes();
                        let offset = s_bytes.len();
                        target = {target}.get_unchecked_mut($num..);
                        target.get_unchecked_mut(..offset)
                            .copy_from_slice(s_bytes);
                        target = {target}.get_unchecked_mut(offset..);
                    }
                },
            )*
            0 => {
                // concat, same principle without the separator
                for s in iter {
                    let s_bytes = s.borrow().as_bytes();
                    let offset = s_bytes.len();
                    target.get_unchecked_mut(..offset)
                        .copy_from_slice(s_bytes);
                    target = {target}.get_unchecked_mut(offset..);
                }
            },
            _ => {
                // arbitrary non-zero size fallback
                for s in iter {
                    target.get_unchecked_mut(..sep_len)
                        .copy_from_slice(sep_bytes);

                    let s_bytes = s.borrow().as_bytes();
                    let offset = s_bytes.len();
                    target = {target}.get_unchecked_mut(sep_len..);
                    target.get_unchecked_mut(..offset)
                        .copy_from_slice(s_bytes);
                    target = {target}.get_unchecked_mut(offset..);
                }
            }
        }
    };
}

pub fn join_new<S: std::borrow::Borrow<str>>(slice: &[S], sep: &str) -> String {
    let sep_len = sep.len();
    let mut iter = slice.iter();
    if let Some(first) = iter.next() {
        // this is wrong without the guarantee that `slice` is non-empty
        // if the `len` calculation overflows, we'll panic
        // we would have run out of memory anyway and the rest of the function requires
        // the entire String pre-allocated for safety
        //
        // this is the exact len of the resulting String
        let len =  sep_len.checked_mul(slice.len() - 1).and_then(|n| {
            slice.iter().map(|s| s.borrow().len()).try_fold(n, usize::checked_add)
        }).expect("attempt to join into String with len > usize::MAX");

        // crucial for safety
        let mut result = String::with_capacity(len);

        unsafe {
            let mut result = result.as_mut_vec();
            result.extend_from_slice(first.borrow().as_bytes());

            {
                let pos = result.len();
                let mut target = result.get_unchecked_mut(pos..len);

                // copy separator and strs over without bounds checks
                // generate loops with hardcoded offsets for small separators
                // massive improvements possible (~ x2)
                spezialize_for_lengths!(sep, target, iter; 1, 2, 3, 4);
            }
            result.set_len(len);
        }
        result
    } else {
        String::new()
    }
}
