use looping::*;
use std::io::BufReader;

use std::io;

fn main() {
    check(&mut BufReader::new(&mut io::stdin()), &mut io::stdout())
}
