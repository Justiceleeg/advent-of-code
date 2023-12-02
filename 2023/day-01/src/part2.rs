// https://adventofcode.com/2023/day/1

pub fn process(input: &str) -> String {
    input.lines().map(process_line).sum::<u32>().to_string()
}

fn process_line(input: &str) -> u32 {
    let input = input
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9");
    let numeric_input = input.chars().filter(|c| c.is_numeric());
    let mut first_num = numeric_input.clone().take(1).collect::<String>();
    let last_num = numeric_input.rev().take(1).collect::<String>();
    first_num.push_str(&last_num);
    first_num.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    /// this test case is from the real input
    /// it tests two overlapping numbers
    /// where the second number should succeed
    #[case("fivezg8jmf6hrxnhgxxttwoneg", 51)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, process_line(line))
    }
}
