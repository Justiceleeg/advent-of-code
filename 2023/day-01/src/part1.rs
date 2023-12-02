// https://adventofcode.com/2023/day/1

pub fn process(input: &str) -> String {
    input.lines().map(process_line).sum::<u32>().to_string()
}

fn process_line(input: &str) -> u32 {
    let numeric_input = input.chars().filter(|c| c.is_numeric());
    let mut first_num = numeric_input.clone().take(1).collect::<String>();
    let last_num = numeric_input.rev().take(1).collect::<String>();
    first_num.push_str(&last_num);
    first_num.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_and_last_number_combined() {
        let test_string = String::from("1abc2");
        let test_string_num_mid = String::from("abd1b4c2aa");
        let result1 = process_line(&test_string);
        let result2 = process_line(&test_string_num_mid);
        let expected = 12;

        assert_eq!(result1, expected);
        assert_eq!(result2, expected);
    }

    #[test]
    fn only_number_combined() {
        let test_string = String::from("treb7uchet");
        let expected = 77;
        let result = process_line(&test_string);
        assert_eq!(result, expected);
    }
}
