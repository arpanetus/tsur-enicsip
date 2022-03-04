pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val)
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    match vec.get(index) {
        Some(x) => x.to_owned(),
        None => "".to_owned()
    }   
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut s = vec!["o".to_string()];
        insert(&mut s, "t".to_string());

        assert_eq!(s.len(), 2);
        assert_eq!(s[1], "t".to_string())
    }

    #[test]
    fn test_at_idx() {
        let mut s = vec!["o".to_string()];
        assert_eq!(at_index(&mut s, 0), "o");
        assert_eq!(at_index(&mut s, 1), "");
    }
}