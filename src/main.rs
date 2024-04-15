use std::str::FromStr;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}
#[derive(Debug)]
enum ParseError {
    Long,
    Short,
    Invalid,
}

impl FromStr for Isbn {
    type Err = ParseError; // TODO: replace with appropriate type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pure_num = s.replace("-", "");
        let x = pure_num.chars().count();

        match x == 13 {
            true => (),
            false => {
                if x > 13 {
                    return Err(ParseError::Long);
                } else {
                    return Err(ParseError::Short);
                }
            }
        }
        let parsed_isbn = Isbn {
            raw: s.to_string(),
            digits: pure_num.chars().map(|x| (x as u8)).collect(),
        };
        Ok(parsed_isbn)
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    todo!()
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({})is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}
