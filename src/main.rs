#![feature(test)]
extern crate test;
extern crate rand;
extern crate time;
use rand::Rng;
#[macro_use] extern crate lazy_static;

const N: usize = 10_000;

lazy_static! {
    static ref SLICE: Vec<String> = {
        let mut rng = rand::StdRng::new().unwrap();
        let mut strings = vec![];
        for i in 1..4 {
            for _ in 0..N {
                let len = 10usize.pow(i);
                strings.push(rng.gen_ascii_chars().take(len).collect());
            }
        }
        strings
    };

    static ref BYTE_SLICE: Vec<&'static [u8]> = {
        SLICE.iter()
            .map(|s| s.as_bytes())
            .collect()
    };
}

macro_rules! spezialize_for_lengths {
    ($separator:expr, $target:expr, $iter:expr; $($num:expr),*) => {
        let mut target = $target;
        let iter = $iter;
        let sep_len = $separator.len();
        let sep_bytes = $separator.as_bytes();
        match $separator.len() {
            $(
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
                for s in iter {
                    let s_bytes = s.borrow().as_bytes();
                    let offset = s_bytes.len();
                    target.get_unchecked_mut(..offset)
                        .copy_from_slice(s_bytes);
                    target = {target}.get_unchecked_mut(offset..);
                }
            },
            _ => {
                // fallback
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

static SEP: &str = "aaaa";
fn join_new<S: std::borrow::Borrow<str>>(slice: &[S], sep: &str) -> String {
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

                // generate loops with hardcoded offsets for small separators
                // massive improvements possible
                spezialize_for_lengths!(sep, target, iter; 1, 2, 3, 4);
            }
            result.set_len(len);
        }
        result
    } else {
        String::new()
    }
}

fn main() {
    fn measure_times<F: FnMut(&[String])-> String>(mut f: F) -> [[f64; 4]; 3] {
        let mut durations = [[0.0; 4]; 3];
        for (idx,string_len) in (1..4).map(|i| 10usize.pow(i)).enumerate() {
            let offset = idx * N;
            for (i, &slice_len) in [10, 100, 1000, 10_000].iter().enumerate() {
                let start = std::time::Instant::now();
                //let duration = time::Duration::span(|| {
                    for _ in 0..1000 {
                        f(&SLICE[offset..offset+slice_len]);
                    }
                //});
                let end = std::time::Instant::now();
                let duration = end - start;
                let duration = duration.as_secs() as f64 + (duration.subsec_nanos() as f64) * 1e-9;

                //if i == 0 { continue }
                durations[idx][i] = duration;
                //println!("string; length: {:4}, slice len: {:5} => {:.4}s", string_len, slice_len, duration*1000.);
            }
        }
        durations
    }
    SLICE.len();
    /*
    let mut separators: Vec<String> = vec!["abcd".into()];
    separators.extend(
        (1..2).map(|n| 10usize.pow(n))
            .map(|len| rand::thread_rng().gen_ascii_chars().take(len).collect())
    );
    */

    println!("        length    slice len    speedup");
    let durations = measure_times(|slice| slice.join(SEP));
    let durations_new = measure_times(|slice| join_new(slice, SEP));;
    for (idx,string_len) in (1..4).map(|i| 10usize.pow(i)).enumerate() {
        let offset = idx * N;
        for (i, &slice_len) in [10,100, 1000, 10_000].iter().enumerate() {
            //for sep in &separators {
                let speedup = durations[idx][i] / durations_new[idx][i];
                println!("string;   {:5}       {:5}       {:.2}", string_len, slice_len, speedup);
            //}
        }
    }
}

#[bench]
fn string_small_regular(b: &mut test::Bencher) {
    b.iter(||
        for _ in 0..REPETITIONS { SLICE[..10].join(SEP); }
    )
}

#[bench]
fn string_small_new(b: &mut test::Bencher) {
    b.iter(||
        for _ in 0..REPETITIONS { join_new(&SLICE[..10], SEP); }
    )
}

#[bench]
fn string_large_regular(b: &mut test::Bencher) {
    b.iter(||
        for _ in 0..REPETITIONS { SLICE.join(SEP); }
    )
}

#[bench]
fn string_large_new(b: &mut test::Bencher) {
    b.iter(||
       for _ in 0..REPETITIONS { join_new(&SLICE, SEP); }
    )
}

#[bench]
fn string_empty_sep(b: &mut test::Bencher) {
    b.iter(|| {
       for _ in 0..REPETITIONS { join_new(&SLICE, ""); }
    })
}

#[bench]
fn string_concat(b: &mut test::Bencher) {
    b.iter(|| {
       for _ in 0..REPETITIONS { SLICE.concat(); }
    })
}

#[test]
fn string_new_regular() {
    assert_eq!(SLICE[..10].join("_"), join_new(&SLICE[..10], "_"));
    assert_eq!(SLICE.join("_"), join_new(&SLICE, "_"));
}

static SEP_VEC: &u8 = &20;

const REPETITIONS: usize = 1;

#[bench]
fn vec_small_regular(b: &mut test::Bencher) {
    b.iter(||
        for _ in 0..REPETITIONS {
            BYTE_SLICE[..10].join(SEP_VEC);
        }
    )
}

#[bench]
fn vec_large_regular(b: &mut test::Bencher) {
    b.iter(||
        for _ in 0..REPETITIONS {
            BYTE_SLICE.join(SEP_VEC);
        }
    )
}

#[test]
fn byte_equality() {
    for (s, &b) in SLICE.iter().zip(BYTE_SLICE.iter()) {
        assert_eq!(s.as_bytes(), b);
    }
}
