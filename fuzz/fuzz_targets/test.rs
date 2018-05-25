#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate join_bench;
//extern crate arbitrary;

fuzz_target!(|data: &[u8]| {
    panic!("blub");
});
/*
fuzz_target!(|data: &[u8]| {
    if data.iter().map(|&n| n as u64).sum::<u64>() > 10000 {
        panic!("Too big");
    }
});
*/
