mod run_length_encoding {

    pub fn encode(text: &str) -> String {
        let mut encoded_string = String::new();
        let mut char_check: char = text.chars().nth(0).unwrap();
        let mut count = 0;
        for (i, char) in text.chars().enumerate() {
            if char_check == char {
                count += 1
            } else {
                encoded_string = encoded_string + &count.to_string() + &char_check.to_string();
                char_check = char;
            }
        }
        encoded_string = encoded_string + &count.to_string() + &char_check.to_string();
        println!("{}", encoded_string);
        encoded_string
    }

    pub fn decode(text: &str) -> String {
        let mut decoded_string = String::new();
        let slice: Vec<char> = text.chars().collect();
        for char in slice.chunks(2) {
            for _ in 0..char[0].to_string().parse::<i32>().unwrap() {
                decoded_string.push(char[1]);
            }
        }
        println!("{decoded_string}");
        decoded_string
    }
}

fn main() {
    let string = "1F2s4d";
    run_length_encoding::decode(string);
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
