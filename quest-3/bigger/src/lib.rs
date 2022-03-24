use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    *h.values().filter(|v| **v > 0).max().unwrap_or_else(|| &0)
}

#[test]
fn test_positive() {
    let mut f = HashMap::new();
    f.insert("Daniel", 12);
    f.insert("Ashley", 333);
    f.insert("Katie", 334);
    f.insert("Robert", 14);

    assert_eq!(334, bigger(f));
}

#[test]
fn test_negative() {
    let mut f = HashMap::new();
    f.insert("Daniel", 41758712);
    f.insert("Ashley", 54551444);
    f.insert("Katie", 575556334);
    f.insert("Robert", 574148);

    assert_eq!(575556334, bigger(f));
}
