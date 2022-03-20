use borrow_the_reference::*;

fn main() {
    let mut a = String::from("bpp--o+er+++sskroi-++lcw");

    delete_and_backspace(&mut a);

    println!("{}", a);


    let mut b_1: Vec<&str> = vec!["2+2=4", "3+2=5", "10-3=3", "5+5=10"];

    let result_1 = is_correct(&mut b_1);

    println!("{}, {:?}",result_1, b_1);
}
