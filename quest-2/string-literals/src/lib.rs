pub fn is_empty(s: &str) -> bool {
    s.len() == 0
}

pub fn is_ascii(s: &str) -> bool {
    s.chars().all(|c| if c as u32 > 127 { false } else { true })
}

pub fn contains(a: &str, b: &str) -> bool {
    if (a.len() == b.len() && a == b) || b.len() == 0 {
        return true;
    }

    if a.len() == 0 || b.len() > a.len() {
        return false;
    }

    if a.len() == b.len() && a != b {
        return false;
    }

    for i in 0..(a.len() - b.len() + 1) {
        if a[i..(i + b.len())] == *b {
            return true;
        }
    }

    false
}

pub fn split_at(s: &str, n: usize) -> (&str, &str) {
    if s.len() < n {
        return (s, "");
    }

    (&s[..n], &s[n..])
}

pub fn find(s: &str, c: char) -> Option<usize> {
    for (i, c_) in s.chars().enumerate() {
        if c_ == c {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        assert_eq!(is_empty(""), true);
        assert_eq!(is_empty(" "), false);
        assert_eq!(is_empty("a"), false);
    }

    #[test]
    fn test_is_ascii() {
        assert_eq!(is_ascii(""), true);
        assert_eq!(is_ascii(" "), true);
        assert_eq!(is_ascii("a"), true);
        assert_eq!(is_ascii("р"), false);
        assert_eq!(is_ascii("рәст"), false);
        assert_eq!(is_ascii("рәст!"), false);
    }

    #[test]
    fn test_contains() {
        assert_eq!(contains("", ""), true);
        assert_eq!(contains("", " "), false);
        assert_eq!(contains("", "a"), false);
        assert_eq!(contains(" ", ""), true);
        assert_eq!(contains(" ", " "), true);
        assert_eq!(contains(" ", "a"), false);
        assert_eq!(contains("a", ""), true);
        assert_eq!(contains("a", " "), false);
        assert_eq!(contains("a", "a"), true);
        assert_eq!(contains("a", "b"), false);
        assert_eq!(contains("a", "ab"), false);
        assert_eq!(contains("ab", "a"), true);
        assert_eq!(contains("ab", "ab"), true);
        assert_eq!(contains("ab", "b"), true);
        assert_eq!(contains("ab", "ba"), false);
        assert_eq!(contains("ab", "baa"), false);
    }

    #[test]
    fn test_split_at() {
        assert_eq!(split_at("", 0), ("", ""));
        assert_eq!(split_at("", 1), ("", ""));
        assert_eq!(split_at("a", 0), ("", "a"));
        assert_eq!(split_at("a", 1), ("a", ""));
        assert_eq!(split_at("a", 2), ("a", ""));
        assert_eq!(split_at("ab", 0), ("", "ab"));
        assert_eq!(split_at("ab", 1), ("a", "b"));
        assert_eq!(split_at("ab", 2), ("ab", ""));
        assert_eq!(split_at("ab", 3), ("ab", ""));
    }

    #[test]
    fn test_find() {
        assert_eq!(find("", 'a'), None);
        assert_eq!(find("a", 'a'), Some(0));
        assert_eq!(find("a", 'b'), None);
        assert_eq!(find("ab", 'a'), Some(0));
        assert_eq!(find("ab", 'b'), Some(1));
        assert_eq!(find("ab", 'c'), None);
    }
}
