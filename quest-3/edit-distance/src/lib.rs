pub fn edit_distance(source: &str, target: &str) -> usize {
    // disgustingly slow, but who cares :)
    fn _edit_distance(source: &[char], target: &[char]) -> usize {
        match (source, target) {
            (&[], t) => t.len(),
            (s, &[]) => s.len(),
            (&[s, ref ss @ ..], &[t, ref ts @ ..]) => {
                if s == t {
                    _edit_distance(ss, ts)
                } else {
                    1 + min(
                        _edit_distance(source, ts),
                        min(_edit_distance(ss, target), _edit_distance(ss, ts)),
                    )
                }
            }
        }
    }
    _edit_distance(
        &source.chars().collect::<Vec<char>>(),
        &target.chars().collect::<Vec<char>>(),
    )
}

fn min(a: usize, b: usize) -> usize {
    if a <= b {
        a
    } else {
        b
    }
}


#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_distance() {
		assert_eq!(edit_distance("gumbo", "gambol"), 2);
		assert_eq!(edit_distance("kitten", "sitting"), 3);
		assert_eq!(edit_distance("rosettacode", "raisethysword"), 8);
	}
}