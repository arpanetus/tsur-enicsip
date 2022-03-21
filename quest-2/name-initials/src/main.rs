use name_initials::*;

fn main() {
	let mut names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
	println!("{:?}", initials(&mut names));
	// output: ["H. P.", "S. E.", "J. L.", "B. O."]
}