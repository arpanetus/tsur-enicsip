pub fn str_len(s:&str) -> usize {
    s.len()
}

#[test]
fn str_len_test() {
	struct TstLit<'a> {
		str: &'a str,
		l: usize,
	}

	struct TstString {
		str: String,
		l: usize,
	}

	let tsts = vec![
		TstLit { str: "hello", l: 5 },
		TstLit { str: "how", l: 3 },
		TstLit {
			str: "are you",
			l: 7,
		},
		TstLit {
			str: "change",
			l: 6,
		},
	];
	let o_tsts = vec![
		TstString {
			str: "hello".to_string(),
			l: 5,
		},
		TstString {
			str: "how".to_string(),
			l: 3,
		},
		TstString {
			str: "are you".to_string(),
			l: 7,
		},
		TstString {
			str: "change".to_string(),
			l: 6,
		},
	];

	for t in tsts.iter() {
		assert_eq!(t.l, str_len(t.str));
	}

	for t in o_tsts.iter() {
		assert_eq!(t.l, str_len(&t.str));
	}
}