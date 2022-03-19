use ownership::first_subword;

fn main() {
    let words: Vec<String> = vec![
        "veryGood".to_owned(),
        "very_good".to_owned(),
        "very".to_owned(),
        "".to_owned(),
    ];


    for word in words {
        println!("word: {}, fsw: {} ", word.clone(), first_subword(word));
    }
}
