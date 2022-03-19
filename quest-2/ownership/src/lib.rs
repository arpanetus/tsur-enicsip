pub fn first_subword(s: String) -> String {
    let mut ci = s.chars();

    let mut prev_c: char;
    match ci.next() {
        Some(c) => {
            prev_c = c;
        }
        None => return s,
    }

    let mut sw: Vec<char> = vec![prev_c];

    for c in ci {
        if prev_c.is_ascii_lowercase() && c.is_ascii_uppercase() || c == '_' {
            break;
        }

        prev_c = c;
        sw.push(c);
    }

    return sw.into_iter().collect();
}


#[test]
fn test_first_subword() {
    let words: Vec<String> = vec![
        "veryGood".to_owned(),
        "very_good".to_owned(),
        "very".to_owned(),
        "".to_owned(),
    ];

    let answers: Vec<String> = vec![
        "very".to_owned(),
        "very".to_owned(),
        "very".to_owned(),
        "".to_owned(),
    ];
    
    words.iter().zip(answers).for_each(|(word, answer)| {
        assert_eq!(first_subword(word.to_owned()), answer);
    });
}
