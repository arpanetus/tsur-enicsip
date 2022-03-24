pub fn bubble_sort(v: &mut Vec<i32>) {
    loop {
        let mut swpd = false;
        for i in 1..v.len() {
            if v[i - 1] > v[i] {
                swpd = true;
                let tmp = v[i];
                v[i] = v[i - 1];
                v[i - 1] = tmp;
            }
        }
        if !swpd {
            return;
        }
    }
}

#[test]
fn test_ordering() {
    let ref mut v = vec![3, 2, 4, 5, 1, 7];
    let mut b = v.clone();

    b.sort();
    bubble_sort(v);
    assert_eq!(*v, b);
}
