use std::io::BufRead;
use std::io::Write;

static ANS: &str = "The letter e\n";
static QUE: &str = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?\n";
static WIN_DEF: &str = "It took you 2 trials to get the right answer\n";


pub fn check(input: &mut impl BufRead, output: &mut impl Write) {
    output.write(QUE.as_bytes());
    println!("output: {}", QUE);
    
    let mut attempt = 0;

    while true {
        let mut s = String::new();
        
        input.read_line(&mut s);
        
        if s == "" {
            continue
        }

        attempt+=1;

        if s == ANS {
            output.write(format!("It took you {} trials to get the right answer\n", attempt).as_bytes());

            return
        }

        output.write(QUE.as_bytes());
        println!("output: {}", QUE);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check() {
        let mut output: Vec<u8> = Vec::new();
        
        let mut input_val = "ANSWER\n".to_owned();
        input_val.push_str(ANS);

        let mut output_val = "".to_owned();
        output_val.push_str(QUE);
        output_val.push_str(QUE);
        output_val.push_str(WIN_DEF);


        check(&mut input_val.as_bytes(), &mut output);

        assert_eq!(&output, output_val.as_bytes());
    }
}