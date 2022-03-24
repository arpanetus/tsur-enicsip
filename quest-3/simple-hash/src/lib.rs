use std::collections::HashMap;

fn contain(h: &HashMap<&str, i32>, s: &str) -> bool {
    h.get(s) != None
}
fn remove(h: &mut HashMap<&str, i32>, s: &str) {
    if contain(h, s) {
        h.remove(s);
    }
}

#[test]
fn test_contains() {
    let mut s = HashMap::new();

    s.insert("Pedro", 43);
    s.insert("Ralph", 12);
    s.insert("Johnny", 546);
    s.insert("Albert", 12323214);

    assert_eq!(true, contain(&s, "Pedro"));
    assert_eq!(true, contain(&s, "Ralph"));
    assert_eq!(true, contain(&s, "Johnny"));
    assert_eq!(true, contain(&s, "Albert"));
    assert_eq!(false, contain(&s, "Marco"));
    assert_eq!(false, contain(&s, "Joan"));
    assert_eq!(false, contain(&s, "Louise"));
}

#[test]
fn test_remove() {
    let mut n = HashMap::new();
    n.insert("Dani Sordo", 37);
    n.insert("Sébastien Loeb", 46);
    n.insert("Ott Tanak", 32);
    n.insert("Thierry Neuville", 32);

    remove(&mut n, "Dani Sordo");
    assert_eq!(true, contain(&n, "Ott Tanak"));
    assert_eq!(false, contain(&n, "Dani Sordo"))
}
