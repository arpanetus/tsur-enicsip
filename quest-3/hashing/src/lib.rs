pub fn mean(list: &Vec<i32>) -> f64 {
    list.iter()
        .fold(0.0, |acc, el| acc + (*el as f64 / list.len() as f64))
}

pub fn median(list: &Vec<i32>) -> f64 {
    let mut list = list.clone();
    list.sort_by(|a, b| a.cmp(b));
    let mid = list.len() / 2;

    if list.len() == 0 {
        0.0
    } else if list.len() % 2 == 0 {
        (list[mid] + list[mid + 1]) as f64 / 2.0
    } else {
        list[mid] as f64
    }
}

use std::collections::HashMap;

pub fn mode(list: &Vec<i32>) -> i32 {
    list.iter()
        .fold(HashMap::new(), |mut acc, el| {
            *acc.entry(*el).or_insert(0) += 1;
            acc
        })
        .iter()
        .fold(
            (0, 0),
            |acc, el| if acc.1 > *el.1 { acc } else { (*el.0, *el.1) },
        )
        .0
}

#[allow(dead_code)]
fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.00000000000001
}

#[test]
fn test_mean() {
    let v = vec![4, 7, 5, 2, 5, 1, 3];
    assert!(approx_eq(mean(&v), 3.857142857142857));
}

#[test]
fn test_median() {
    let v = vec![4, 7, 5, 2, 5, 1, 3];
    assert_eq!(median(&v), 4.0);
}

#[test]
fn test_mode() {
    let v = vec![4, 7, 5, 2, 5, 1, 3];
    assert_eq!(mode(&v), 5);
}
