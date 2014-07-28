fn selection_sort(arr: &mut[int]) {
    let (mut i, n)  = (0, arr.len());
    while i < n {
        let (mut j, mut min) = (i, i);
        while j < n  {
            if arr[j] < arr[min] {
                min = j;
            }
            j += 1;
        }
        let tmp = arr[i];
        arr[i] = arr[min];
        arr[min] = tmp;
        i += 1;
    }
}

#[test]
fn selection_sort_test() {
    let expected: [int, ..5] = [1, 2, 3, 4, 5];
    let mut arr: [int, ..5] = [2, 4, 1, 3, 5];
    selection_sort(arr);
    assert!(arr == expected);
}

