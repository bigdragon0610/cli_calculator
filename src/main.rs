use std::io::{self, Write};

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let ans = parse_formula(buffer.trim());
        if let Some(ans) = ans {
            println!("{}", ans);
        }
    }
}

fn parse_formula(formula: &str) -> Option<isize> {
    let operators = ['+', '-'];
    let mut formula = formula.chars().filter(|c| !c.is_whitespace()).peekable();
    let mut ans: isize = 0;
    let mut operator = '+';

    if let Some(c) = formula.peek() {
        if *c == '-' {
            operator = '-';
        }
    }

    while let Some(c) = formula.next() {
        if c.is_ascii_digit() {
            let mut num_str = c.to_string();
            while let Some(next_c) = formula.peek() {
                if let Some(next_num) = next_c.to_digit(10) {
                    num_str = format!("{}{}", num_str, next_num);
                    formula.next();
                } else {
                    break;
                }
            }
            let num = num_str.parse::<isize>().unwrap();
            match operator {
                '+' => {
                    ans += num;
                }
                '-' => {
                    ans -= num;
                }
                _ => (),
            }
        } else if operators.contains(&c) {
            if let Some(next_c) = formula.peek() {
                if operators.contains(next_c) {
                    println!("Error: Unexpected operator '{}' following '{}'", next_c, c);
                    return None;
                }
            }
            operator = c;
        } else {
            println!("Error: Unexpected character '{}'", c);
            return None;
        }
    }
    Some(ans)
}

#[test]
fn test_parse_formula() {
    assert_eq!(parse_formula("+-"), None);
    assert_eq!(parse_formula("-+"), None);
    assert_eq!(parse_formula("- 1 +- 1"), None);
    assert_eq!(parse_formula("1"), Some(1));
    assert_eq!(parse_formula("-1"), Some(-1));
    assert_eq!(parse_formula("1 + 9"), Some(10));
    assert_eq!(parse_formula("10 - 1 10"), Some(-100));

    assert_eq!(parse_formula("*"), None);
    assert_eq!(parse_formula("/"), None);
    assert_eq!(parse_formula("("), None);
    assert_eq!(parse_formula(")"), None);
    assert_eq!(parse_formula("a"), None);
}
