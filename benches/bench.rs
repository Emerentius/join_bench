
#![allow(unused_variables)]
#[macro_use]
extern crate criterion;
extern crate join_bench;
extern crate rand;
#[macro_use] extern crate lazy_static;

use rand::Rng;
use join_bench::join_new;
use join_bench::join_new_vec;
use join_bench::concat_new_vec;
use criterion::Criterion;

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

fn bench_join(c: &mut Criterion) {

    let mut separators: Vec<String> = vec!["abcd".into()];
    separators.extend(
        (1..=2).map(|n| 10usize.pow(n))
            .map(|len| rand::thread_rng().gen_ascii_chars().take(len).collect())
    );

    let mut inputs = vec![];
    for offset in (0..3).map(|n| n * N) {
        for &slice_len in [10,100, 1000, 10_000].iter() {
            for sep in &separators {
                inputs.push((&SLICE[offset..offset+slice_len], sep.clone()));
            }
        }
    }

    // initialize static
    SLICE.len();

    for input in inputs {
        let strings = input.0;
        let sep = input.1;
        let string_len = strings[0].len();
        let n_strings = strings.len();
        let sep_len = sep.len();

        let sep_ = sep.clone();
        let strings_ = strings.clone();
        let bench = criterion::Benchmark::new(
                "old_join",
                move |b| b.iter(|| strings_.join(&sep_)),
            )
            .with_function("new_join", move |b| b.iter(|| join_new(strings, &sep),));
        c.bench(&format!("len:{}_n:{}_sep_len:{}", string_len, n_strings, sep_len),  bench);
    }
}

fn vec_bench_join(c: &mut Criterion) {

    let separator = 0;

    let mut inputs = vec![];
    for offset in (0..3).map(|n| n * N) {
        for &slice_len in [
            10,
            100,
            1000,
            10_000
        ].iter() {
            inputs.push((&BYTE_SLICE[offset..offset+slice_len], separator));
        }
    }

    // initialize static
    BYTE_SLICE.len();

    for input in inputs {
        let slice = input.0;
        let sep = input.1;
        let string_len = slice[0].len();
        let slice_len = slice.len();
        let sep_len = 1;

        let sep_ = sep.clone();
        let slice_ = slice.clone();
        let bench = criterion::Benchmark::new(
                "vec_old_join",
                move |b| b.iter(|| slice_.join(&sep_)),
            )
            .with_function("vec_new_join", move |b| b.iter(|| join_new_vec(slice, &sep),));
        c.bench(&format!("len:{}_n:{}_sep_len:{}", string_len, slice_len, sep_len),  bench);
    }
}

fn concat_std(c: &mut Criterion) {
    c.bench_function("concat_std", |b| b.iter(|| SLICE[0..1000].concat()));
}

fn concat(c: &mut Criterion) {
    c.bench_function("concat", |b| b.iter(|| join_new(&SLICE[0..1000], "")));
}

fn vec_concat_std(c: &mut Criterion) {
    c.bench_function("vec_concat_std", |b| b.iter(|| BYTE_SLICE[0..1000].concat()));
}

fn vec_concat(c: &mut Criterion) {
    c.bench_function("vec_concat", |b| b.iter(|| concat_new_vec(&BYTE_SLICE[0..1000])));
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(1000))
        .measurement_time(std::time::Duration::from_millis(3000));
    targets = bench_join, concat, concat_std, vec_bench_join, vec_concat_std, vec_concat
);
criterion_main!(benches);
