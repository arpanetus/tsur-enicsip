use std::collections::HashMap;

use simple_hash::*;

fn main() {
	let mut hash: HashMap<&str, i32> = HashMap::new();
	hash.insert("Daniel", 122);
	hash.insert("Ashley", 333);
	hash.insert("Katie", 334);
	hash.insert("Robert", 14);

	println!(
		"Does the HashMap contains the name Roman? => {}",
		contain(hash.clone(), "Roman")
	);
	println!(
		"Does the HashMap contains the name Katie? => {}",
		contain(hash.clone(), "Katie")
	);
	println!("Removing Robert {:?}", remove(hash.clone(), "Robert"));
	println!("Hash {:?}", hash);
}