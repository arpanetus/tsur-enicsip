use string_literals::*;

fn main() {
    println!("{}", is_empty(""));
    println!("{}", is_ascii("rust"));
    println!("{}", is_ascii("рәст"));
    println!("{}", contains("rust", "ru"));
    println!("{}", contains("rust", "р"));
    println!("{:?}", split_at("rust", 2));
    println!("{:?}", find("rust", 'u'));
}