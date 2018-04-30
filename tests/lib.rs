extern crate join_bench;

#[test]
fn correct_output() {
    let slices: [&[_]; 3] = [
        &["abc", "def", "foo"],
        &[],
        &["abc"],
    ];
    let seps = ["shrt", "looooooong", ""];

    for strs in &slices {
        for sep in &seps {
            let regular_joined = strs.join(sep);
            let new_joined = join_bench::join_new(&strs, sep);

            assert_eq!(regular_joined, new_joined);
            assert_eq!(regular_joined.capacity(), new_joined.capacity());
            assert_eq!(regular_joined.len(), new_joined.len());
        }
    }
}
