pub fn capitalize_first(input: &str) -> String {
    let input = input.to_string();
    if input.len() == 0 {
        input
    } else {
        let mut inter = input.get(0..1).unwrap().to_uppercase();
        inter.extend(input.get(1..input.len()).unwrap().chars());
        inter    
    }
}

pub fn title_case(input: &str) -> String {
    input.chars().fold(String::new(), |mut acc, c| {
        if acc.len() == 0 || acc.chars().last().unwrap() == ' ' {
            acc.extend(c.to_uppercase());
            acc
        } else {
            acc.push(c);
            acc
        }
    })
}

pub fn change_case(input: &str) -> String {
    fn iul(c: char) -> Vec<char> {
        if c.is_uppercase() {
            c.to_lowercase().collect()
        } else if c.is_lowercase() {
            c.to_uppercase().collect()
        } else {
            vec![c]
        }
    }

    input.chars().map(|c| iul(c)).flatten().collect()
}


#[test]
fn test_success() {
	assert_eq!(capitalize_first("hello"), "Hello");
	assert_eq!(capitalize_first("this is working"), "This is working");
}

#[test]
fn test_title_case() {
	assert_eq!(title_case("this is a tittle"), "This Is A Tittle");
	assert_eq!(title_case("hello my name is carl"), "Hello My Name Is Carl");
}

#[test]
fn test_change_case() {
	assert_eq!(change_case("PROgraming"), "proGRAMING");
	assert_eq!(change_case("heLLo THere"), "HEllO thERE");
}

#[test]
fn test_empty() {
	assert_eq!(capitalize_first(""), "");
	assert_eq!(title_case(""), "");
	assert_eq!(change_case(""), "");
}