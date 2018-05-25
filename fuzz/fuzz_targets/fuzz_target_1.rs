#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate join_bench;
//extern crate arbitrary;

fuzz_target!(|data: (String, Box<[String]>)| {
    let (sep, slice) = data;
    let old_string = slice.join(&sep);
    let new_string = join_bench::join_new(&slice, &sep);
    assert_eq!(old_string, new_string);
    assert!(new_string.capacity() >= new_string.len());
});
