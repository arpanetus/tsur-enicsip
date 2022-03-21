pub fn initials(names: &Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            let mut ins = String::new();

            for c in name.chars() {
                if c.is_ascii_uppercase() {
                    ins.push_str(format!("{}. ", c).as_str());
                }
            }

            ins[..ins.len() - 2].to_string()
        })
        .collect::<Vec<String>>()
}

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[allow(unused_imports)]
use jemalloc_ctl::{epoch, stats};

#[allow(dead_code)]
struct Test<'a> {
    names: Vec<&'a str>,
    result: Vec<&'a str>,
}

// these are not my comments, these are the comments from the original code, and it's shit:
// solution that will run against the students solution
// this function uses the less heap allocation
#[allow(dead_code)]
fn initials_sol(arr: &mut Vec<&str>) -> Vec<String> {
    arr.iter()
        .map(|ele| {
            let mut names = ele.split_whitespace();
            let mut a = names.next().unwrap().chars().nth(0).unwrap().to_string();
            a.push_str(". ");
            let mut b = names.next().unwrap().chars().nth(0).unwrap().to_string();
            b.push_str(".");
            a.push_str(&b);
            a
        })
        .collect()
}

#[test]
fn test_memory_allocation() {
    // the statistics tracked by jemalloc are cached
    // The epoch controls when they are refreshed
    let e = epoch::mib().unwrap();
    // allocated: number of bytes allocated by the application
    let allocated = stats::allocated::mib().unwrap();
    let mut test_value = vec![
        "Lee Silva",
        "Harry Potter",
        "Someone Else",
        "J. L.",
        "Barack Obama",
    ];

    initials_sol(&mut test_value);
    // this will advance with the epoch giving the its old value
    // where we read the updated heap allocation using the `allocated.read()`
    e.advance().unwrap();
    let solution = allocated.read().unwrap();

    initials(&mut test_value);
    e.advance().unwrap();
    let student = allocated.read().unwrap();

    assert!(
        student <= solution,
        "your heap alloc is {}, and it must be less than or equal {}",
        student,
        solution
    );
}
