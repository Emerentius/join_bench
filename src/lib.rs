pub fn join_new<S: std::borrow::Borrow<str>>(slice: &[S], sep: &str) -> String {
    unsafe {
        String::from_utf8_unchecked( join_generic(slice, sep.as_bytes()) )
    }
}

macro_rules! generic_spezialize_for_lengths {
    ($separator:expr, $target:expr, $iter:expr; $($num:expr),*) => {
        let mut target = $target;
        let iter = $iter;
        let sep_len = $separator.len();
        let sep_bytes = $separator;
        match $separator.len() {
            $(
                // loops with hardcoded sizes run much faster
                // specialize the cases with small separator lengths
                $num => {
                    for s in iter {
                        target.get_unchecked_mut(..$num)
                            .copy_from_slice(sep_bytes);

                        let s_bytes = s.borrow().as_ref();
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
                    let s_bytes = s.borrow().as_ref();
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

                    let s_bytes = s.borrow().as_ref();
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

// Works for joining both Vec<T> and String's inner vec
// the bounds for String-join are S: Borrow<str> and for Vec-join Borrow<[T]>
// [T] and str both impl AsRef<[T]> for some T
// => s.borrow().as_ref() and we always have slices
pub fn join_generic<B, T, S>(slice: &[S], sep: &[T]) -> Vec<T>
where
    T: Copy,
    B: AsRef<[T]> + ?Sized,
    S: std::borrow::Borrow<B>,
{
    let sep_len = sep.len();
    let mut iter = slice.iter();
    iter.next().map_or(vec![], |first| {
        // this is wrong without the guarantee that `slice` is non-empty
        // if the `len` calculation overflows, we'll panic
        // we would have run out of memory anyway and the rest of the function requires
        // the entire String pre-allocated for safety
        //
        // this is the exact len of the resulting String
        let len =  sep_len.checked_mul(slice.len() - 1).and_then(|n| {
            slice.iter().map(|s| s.borrow().as_ref().len()).try_fold(n, usize::checked_add)
        }).expect("attempt to join into collection with len > usize::MAX");

        // crucial for safety
        let mut result = Vec::with_capacity(len);

        unsafe {
            result.extend_from_slice(first.borrow().as_ref());

            {
                let pos = result.len();
                let target = result.get_unchecked_mut(pos..len);

                // copy separator and strs over without bounds checks
                // generate loops with hardcoded offsets for small separators
                // massive improvements possible (~ x2)
                generic_spezialize_for_lengths!(sep, target, iter; 1, 2, 3, 4);
            }
            result.set_len(len);
        }
        result
    })
}

pub fn join_new_vec<T: Copy, S: std::borrow::Borrow<[T]>>(slice: &[S], sep: &T) -> Vec<T> {
    join_generic(slice, &[*sep])
}

pub fn concat_new_vec<T: Copy, S: std::borrow::Borrow<[T]>>(slice: &[S]) -> Vec<T> {
    join_generic(slice, &[])
}

pub fn trivial_join_new_vec<T: Clone, V: std::borrow::Borrow<[T]>>(slice: &[V], sep: &T) -> Vec<T> {
    let mut iter = slice.iter();

    iter.next().map_or(vec![], |first| {
        let size: usize = slice.iter().map(|s| s.borrow().len()).sum();
        let mut result = Vec::with_capacity(size + slice.len());
        result.extend_from_slice(first.borrow());

        for v in iter {
            result.push(sep.clone());
            result.extend_from_slice(v.borrow())
        }
        result
    })
}

pub fn concat<T: Clone, V: std::borrow::Borrow<[T]>>(slice: &[V]) -> Vec<T> {
    //let mut iter = slice.iter();

    //iter.next().map_or(vec![], |first| {
        let size: usize = slice.iter().map(|s| s.borrow().len()).sum();
        let mut result = Vec::with_capacity(size + slice.len());
        //result.extend_from_slice(first.borrow());

        for v in slice {
            result.extend_from_slice(v.borrow())
        }
        result
    //})
}
