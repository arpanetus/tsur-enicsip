pub fn delete_and_backspace(s: &mut String) {
    let mut cs: Vec<char> = vec![];
    let mut i = 0;

    for s in s.chars() {
        if s == '+' {
            i += 1;
        } else if s == '-' {
            cs.pop();
        } else {
            if i <= 0 {
                cs.push(s);
            } else {
                i -= 1;
            }
        }
    }

    s.clear();
    s.push_str(cs.into_iter().collect::<String>().as_str());
}

enum Op {
    Plus,
    Minus,
}

fn op_fn(op: Op) -> impl Fn(i32, i32) -> i32 {
    match op {
        Op::Plus => |a, b| a + b,
        Op::Minus => |a, b| a - b,
    }
}

pub fn is_correct(ss: &mut Vec<&str>) -> i32 {
    ss.iter_mut().for_each(|s| {
        let exp = s.split('=').collect::<Vec<&str>>();
        if exp.len() != 2 {
            println!("exp: {:?}", exp);
            return;
        }
        *s = "✘";

        let res = exp[1].parse::<i32>().unwrap();
        let mut cur_op: Option<Op> = None;
        let (mut lv, mut rv) = (vec![], vec![]);
        let mut curv = &mut lv;
        for c in exp[0].chars() {
            match c {
                '+' => {
                    curv = &mut rv;
                    cur_op = Some(Op::Plus);
                }
                '-' => {
                    curv = &mut rv;
                    cur_op = Some(Op::Minus);
                }
                _ => {
                    curv.push(c);
                }
            }
        }

        let (l, r) = (
            lv.iter().collect::<String>().parse::<i32>().unwrap(),
            rv.iter().collect::<String>().parse::<i32>().unwrap(),
        );

        let mut pos_res = 0;

        match cur_op {
            Some(op) => {
                pos_res = op_fn(op)(l, r);
            }
            None => return,
        }

        if pos_res == res {
            *s = "✔";
        }
    });

    return ((ss.iter().filter(|s| **s == "✔").count() as f32) / (ss.len() as f32) * 100.0) as i32;
}

#[test]
fn reference_string() {
    let mut a_1 = String::from("bpp--o+er+++sskroi-++lcw");
    let mut a_2 = String::from("hs-+deasdasd------l+++dsdp");
    let mut a_3 = String::from("pad-rtic+eulqw--+rar");
    delete_and_backspace(&mut a_1);
    delete_and_backspace(&mut a_2);
    delete_and_backspace(&mut a_3);
    assert_eq!(a_1, "borrow".to_string());
    assert_eq!(a_2, "help".to_string());
    assert_eq!(a_3, "particular".to_string());
}

#[test]
fn reference_vec() {
    let mut b_1: Vec<&str> = vec!["2+2=4", "3+2=5", "10-3=3", "5+5=10"];
    let mut b_2: Vec<&str> = vec!["1+2=4", "0+2=5", "10-3=3", "41+5=10"];
    let mut b_3: Vec<&str> = vec!["2+2=4", "3+2=5", "10-3=7", "5+5=10"];
    let result_1 = is_correct(&mut b_1);
    let result_2 = is_correct(&mut b_2);
    let result_3 = is_correct(&mut b_3);
    assert_eq!(b_1, vec!["✔", "✔", "✘", "✔"]);
    assert_eq!(b_2, vec!["✘", "✘", "✘", "✘"]);
    assert_eq!(b_3, vec!["✔", "✔", "✔", "✔"]);

    assert_eq!(result_1, 75);
    assert_eq!(result_2, 0);
    assert_eq!(result_3, 100);
}
