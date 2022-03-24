pub fn to_url(s: &str) -> String {
    s.chars()
        .map(|a| {
            if a == ' ' {
                vec!['%', '2', '0']
            } else {
                vec![a]
            }
        })
        .flatten()
        .collect::<String>()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_url() {
		assert_eq!(to_url("this is my search"), "this%20is%20my%20search");
	}
}