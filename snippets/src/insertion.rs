fn insertion_sort(arr: &mut [i32]) {
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

#[cfg(test)]
fn insertion_sort_test() {
    let expected: [i32; 5] = [1, 2, 3, 4, 5];
    let mut arr: [i32; 5] = [2, 4, 1, 3, 5];
    insertion_sort(&mut arr);
    assert!(arr == expected);
}

#[cfg(test)]
fn insertion_sort_non_equal_test() {
    let expected: [i32; 5] = [2, 1, 3, 4, 5];
    let mut arr: [i32; 5] = [2, 4, 1, 3, 5];
    insertion_sort(&mut arr);
    assert!(arr != expected);
}

