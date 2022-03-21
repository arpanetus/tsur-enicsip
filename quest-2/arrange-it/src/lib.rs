// this one works like a charm but sometimes it allocates more memory than the solution
pub fn arrange_phrase_ideal(phrase: &str) -> String {
    let mut words: Vec<(i32, String)> = vec![];
    let mut cur_num = String::new();
    let mut cur_word = String::new();

    for c in phrase.chars() {
        match c {
            c if c.is_ascii_digit() => {
                cur_num.push(c);
            }

            ' ' => {
                words.push((cur_num.parse::<i32>().unwrap(), cur_word.clone()));

                cur_num.clear();
                cur_word.clear();
            }

            _ => {
                cur_word.push(c);
            }
        }
    }


    words.push((cur_num.parse::<i32>().unwrap(), cur_word));
    

    let mut words_to_compile: Vec<&str> = vec![""; words.len()];
    for (i, word) in &words {
        words_to_compile[(i - 1) as usize] = word;
    }

    words_to_compile.join(" ")
}


pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<(i32, String)> = vec![];
    let mut cur_num = '0';
    let mut cur_word = String::new();

    for c in phrase.chars() {
        match c {
            c if c.is_ascii_digit() => {
                cur_num = c;
            }

            ' ' => {
                words.push((cur_num.to_digit(10).unwrap() as i32, cur_word.clone()));

                cur_num = '0';
                cur_word.clear();
            }

            _ => {
                cur_word.push(c);
            }
        }
    }


    words.push((cur_num.to_digit(10).unwrap() as i32, cur_word));
    
    words.sort_by(|a, b| a.0.cmp(&b.0));


    let mut words_to_compile: Vec<&str> = vec![""; words.len()];
    for (i, word) in &words {
        words_to_compile[(i - 1) as usize] = word;
    }

    words_to_compile.join(" ")

}



#[allow(unused_imports)]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[allow(unused_imports)]
use jemalloc_ctl::{epoch, stats};

#[allow(dead_code)]
fn arrange_phrase_sol(phrase: &str) -> String {
    let nbrs: Vec<&str> = phrase.matches(char::is_numeric).collect();
    let a = &phrase.replace(char::is_numeric, "");
    let mut m: Vec<&str> = a.split_whitespace().collect();

    for (i, ele) in nbrs.iter().enumerate() {
        let strs: Vec<&str> = a.split_whitespace().collect();
        m[ele.parse::<usize>().unwrap() - 1] = strs[i];
    }
    m.join(" ")
}

#[test]
fn test_heap_memory_allocation() {
    // the statistics tracked by jemalloc are cached
    // The epoch controls when they are refreshed
    let e = epoch::mib().unwrap();
    // allocated: number of bytes allocated by the application
    let allocated = stats::allocated::mib().unwrap();
    let test_value = "4of Fo1r pe6ople g3ood th5e the2";

    arrange_phrase_sol(test_value);
    // this will advance with the epoch giving the its old value
    // where we read the updated heap allocation using the `allocated.read()`
    e.advance().unwrap();
    let solution = allocated.read().unwrap();

    arrange_phrase(test_value);
    e.advance().unwrap();
    let student = allocated.read().unwrap();

    assert!(
        student <= solution,
        "your heap allocation is {}, and it must be less or equal to {}",
        student,
        solution
    );
}

#[test]
fn test_function() {
    let cases = vec![
        "4of Fo1r pe6ople g3ood th5e the2",
        "is2 Thi1s T4est 3a",
        "w7ork t3he a4rt o5f Per1formance is2 a6voiding",
    ];
    for v in cases {
        assert_eq!(arrange_phrase(v), arrange_phrase_sol(v));
    }
}
