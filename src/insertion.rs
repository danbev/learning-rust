fn insertion_sort(arr: &mut[int]) {
    let (mut i, n)  = (1, arr.len());
    while i < n {
        let mut j = i;
        while j > 0 {
            if arr[j] < arr[j-1] {
                let tmp = arr[j-1];
                arr[j-1] = arr[j];
                arr[j] = tmp;
            }
            j -= 1;
        }
        i += 1;
    }
}

#[test]
fn insertion_sort_test() {
    let expected: [int, ..5] = [1, 2, 3, 4, 5];
    let mut arr: [int, ..5] = [2, 4, 1, 3, 5];
    insertion_sort(arr);
    assert!(arr == expected);
}

#[test]
fn insertion_sort_non_equal_test() {
    let expected: [int, ..5] = [2, 1, 3, 4, 5];
    let mut arr: [int, ..5] = [2, 4, 1, 3, 5];
    insertion_sort(arr);
    assert!(arr != expected);
}

