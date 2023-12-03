// https://adventofcode.com/2023/day/2
// only 12 red cubes, 13 green cubes, and 14 blue cubes?

use regex::Regex;

pub fn process(input: &str) -> u32 {
    input
        .lines()
        .enumerate()
        .map(|(i, e)| process_line(i, e))
        .fold(0, |a, c| a + c)
}

fn process_line(index: usize, input: &str) -> u32 {
    // check if line is a valid game or not
    let rr = Regex::new(r"(?<red>\d+) red").unwrap();
    let rg = Regex::new(r"(?<green>\d+) green").unwrap();
    let rb = Regex::new(r"(?<blue>\d+) blue").unwrap();
    let reds: Vec<u32> = rr
        .captures_iter(input)
        .map(|caps| {
            let (_, [red]) = caps.extract();
            red
        })
        .map(|e| e.parse().unwrap())
        .collect();
    let greens: Vec<u32> = rg
        .captures_iter(input)
        .map(|caps| {
            let (_, [green]) = caps.extract();
            green
        })
        .map(|e| e.parse().unwrap())
        .collect();
    let blues: Vec<u32> = rb
        .captures_iter(input)
        .map(|caps| {
            let (_, [blue]) = caps.extract();
            blue
        }) // extract digits assoc with color
        .map(|e| e.parse().unwrap()) // convert digits to integers
        .collect();

    if (reds.iter().max().unwrap() <= &12
        && greens.iter().max().unwrap() <= &13
        && blues.iter().max().unwrap() <= &14)
    {
        return index as u32 + 1;
    } else {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn process_sum_correctly() {
        let test_string = String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        let result1 = process(&test_string);
        let expected = 8;

        assert_eq!(result1, expected);
    }
}
