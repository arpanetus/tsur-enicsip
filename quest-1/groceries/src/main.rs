use groceries::{insert, at_index};

fn main() {
    let mut groceries = vec![
		"yogurt".to_string(),
		"panetone".to_string(),
		"bread".to_string(),
		"cheese".to_string(),
	];
	insert(&mut groceries, String::from("nuts"));
	println!("The groceries list now = {:?}", &groceries);
	println!(
		"The second element of the grocery  list is {:?}",
		at_index(&groceries, 1)
	);
}
