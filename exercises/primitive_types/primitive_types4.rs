// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    let nice_slice: [i32;3] = a.into_iter().filter(|z| (2 <= (*z)) && ((*z) <= 4)).collect::<Vec<i32>>().try_into().unwrap();

    assert_eq!([2, 3, 4], nice_slice)
}
