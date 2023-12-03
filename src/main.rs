fn main() {
    let file_path: &str = "./src/input.txt";
    let content: String = std::fs::read_to_string(file_path).expect("should read from file");
    let input_vector = content.lines().collect::<Vec<&str>>();

    let sum_vector: i32 = input_vector.iter().map(|&input| {parse_input_using_aho_corasick(input).parse::<i32>().unwrap()}).sum();
    let final_sum = modified_sum_of_input_vector(input_vector);
    println!("{}", sum_vector);
    println!("{}", final_sum)}


// // // // // PART 2 // // // // //

pub fn modified_sum_of_input_vector(input: Vec<&str>) -> i32 {
    input
        .iter()
        .map(|parsed_result| {
            parse_input_using_aho_corasick(parsed_result)
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}
fn convert_format_from_word_to_number(input: &str) -> i16 {
    let conversion: &str = match input {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "1" => "1",
        "2" => "2",
        "3" => "3",
        "4" => "4",
        "5" => "5",
        "6" => "6",
        "7" => "7",
        "8" => "8",
        "9" => "9",
        _ => "oops",
    };

    conversion.parse().unwrap()
}

fn parse_input_using_aho_corasick(input: &str) -> String {
    let patterns = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let haystack = input;

    let ac = aho_corasick::AhoCorasick::new(patterns).unwrap();
    let mut matches = vec![];
    for mat in ac.find_iter(haystack) {
        matches.push(mat);
    }

    let first_match = matches.get(0).unwrap();
    let last_match = matches.get(matches.len() - 1).unwrap();

    let first_result =
        convert_format_from_word_to_number(&haystack[first_match.start()..first_match.end()]);
    let last_result =
        convert_format_from_word_to_number(&haystack[last_match.start()..last_match.end()]);

    format!("{}{}", first_result, last_result)
}

// // // // // PART 1 // // // // //

fn part_1() {
    let file_path: &str = "./src/input.txt";
    let content: String = std::fs::read_to_string(file_path).expect("should read from file");
    let input_vector = content.lines().collect::<Vec<&str>>();
    let final_sum = sum_of_input_vector(input_vector);
    println!("{}", final_sum);
}

fn sum_of_input_vector(input: Vec<&str>) -> i32 {
    input
        .iter()
        .map(|parsed_result| {
            isolate_first_and_last_number_as_string(parsed_result)
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

fn isolate_first_and_last_number_as_string(input: &str) -> String {
    let mut data_store: Vec<String> = vec![];

    let _ = &input.chars().try_for_each(|c| {
        if c.is_numeric() {
            data_store.push(String::from(c));
            None
        } else {
            Some(())
        }
    });

    let _ = &input.chars().rev().try_for_each(|c| {
        if c.is_numeric() {
            data_store.push(String::from(c));
            None
        } else {
            Some(())
        }
    });

    data_store.iter().map(|x| x.to_string()).collect::<String>()
}

// // // // // // // // // //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_sum_vector_array() {
        let input_vector: Vec<&str> = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let input_sum: i32 = sum_of_input_vector(input_vector);

        assert_eq!(input_sum, 142);
    }

    #[test]
    fn test_conversion_from_word_to_number() {
        assert_eq!(convert_format_from_word_to_number("one"), 1)
    }

    #[test]
    fn aho_corasick_application() {
        use aho_corasick::AhoCorasick;

        let patterns = &["one", "1", "two"];
        let haystack = "Nobody two 1 one 1 maple in their apple flavored Snapple.";

        let ac = AhoCorasick::new(patterns).unwrap();
        let mut matches = vec![];
        for mat in ac.find_iter(haystack) {
            matches.push(mat);
        }

        let first_match = matches.get(0).unwrap();
        let last_match = matches.get(matches.len() - 1).unwrap();

        let first_result = &haystack[first_match.start()..first_match.end()];
        let last_result = &haystack[last_match.start()..last_match.end()];

        assert_eq!(first_result, "two");
        assert_eq!(last_result, "1");
    }

    #[test]
    fn functional_application_of_algo() {
        assert_eq!(parse_input_using_aho_corasick("a23"), "23");
        assert_eq!(parse_input_using_aho_corasick("zoneight234"), "14");
        assert_eq!(parse_input_using_aho_corasick("7pqrstsixteen"), "76");
    }

    #[test]
    fn mixed_vector_round_2_application_of_algo() {
        let input_vector: Vec<&str> = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        let input_sum: i32 = modified_sum_of_input_vector(input_vector);
        assert_eq!(input_sum, 281);
    }

    #[test]
    fn handling_single_matches() {
        assert_eq!(parse_input_using_aho_corasick("a2b"), "22");
        assert_eq!(parse_input_using_aho_corasick("one3rrbseven3sevenpnnrnrz6"), "16");
        assert_eq!(parse_input_using_aho_corasick("1sevenseven7ld"), "17");
        assert_eq!(parse_input_using_aho_corasick("f6four7mttnqsvzmbsqljdzqhcpnprscsggvvsevennine"), "69");
    }

    #[test]
    fn fish_for_main_errors() {
        main()
    }


}
