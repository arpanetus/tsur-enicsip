use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut hm = HashMap::new();

    s1.chars().zip(s2.chars()).for_each(|(c1, c2)| {
        *hm.entry(c1).or_insert(0) += 1;
        *hm.entry(c2).or_insert(0) -= 1;
    });

    println!("{:?}", hm);

    hm.iter().filter(|e| *e.1 != 0).count() == 0
}

#[test]
fn permutation_ascii() {
    assert!(is_permutation("abcde", "edbca"));
    assert!(!is_permutation("avcde", "edbca"));
    assert!(!is_permutation("cde", "edbca"));
    assert!(is_permutation("code", "deco"));
    assert!(!is_permutation("code", "deeco"));
    assert!(!is_permutation("codde", "deeco"));
}

#[test]
fn permutation_unicode() {
    assert!(is_permutation("hello♥", "♥oelhl"));
    assert!(!is_permutation("♥", "♥♥"));
}
