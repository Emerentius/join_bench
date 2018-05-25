extern crate join_bench;

#[test]
fn correct_output() {
    let slices: [&[_]; 5] = [
        &["abc", "def", "foo"],
        &[],
        &["abc"],
        &["gabaäœ吃", "", "", "foo"],
        &[" 	╄ 	╅ 	╆ 	╇ 	╈ 	╉ 	", "", "foo"],
    ];
    let seps = ["shrt", "looooooong", "", "“o(∩∩)o...哈哈”", "中文", "œ"];

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
