use regex::Regex;

fn main() {}

fn combine_first_and_last_of_any_acceptable_target_in_input(
    input: &str,
    acceptable_targets: Vec<&str>,
) -> Vec<String> {
    let v: Vec<&str> = input.split(char::is_numeric).collect();
    let mut data_store: Vec<String> = Vec::with_capacity(2);

    let first_find: Option<&&str> = v.iter().find(|&&element| {
        acceptable_targets
            .iter()
            .any(|&target_element| target_element == element)
    });

    let last_find: Option<&&str> = v.iter().rfind(|&&element| {
        acceptable_targets
            .iter()
            .any(|&target_element| target_element == element)
    });

    data_store.push(String::from(first_find.unwrap().to_owned()));
    data_store.push(String::from(last_find.unwrap().to_owned()));
    data_store
}

fn part_1() {
    let file_path: &str = "./src/input.txt";
    let content: String = std::fs::read_to_string(file_path).expect("should read from file");
    let input_vector = content.lines().collect::<Vec<&str>>();
    let final_sum = sum_of_input_vector(input_vector);
    println!("{}", final_sum);
}

pub fn sum_of_input_vector(input: Vec<&str>) -> i32 {
    input
        .iter()
        .map(|parsed_result| {
            isolate_first_and_last_number_as_string(parsed_result)
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

pub fn isolate_first_and_last_number_as_string(input: &str) -> String {
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
    fn test_split_on_numbers_and_find_first_and_last_match() {
        let v: Vec<&str> = "one1two2three".split(char::is_numeric).collect();
        let target_vector: Vec<&str> = vec!["one", "two", "three"];

        let result = combine_first_and_last_of_any_acceptable_target_in_input(
            "one1two2three",
            target_vector,
        );
        assert_eq!(result, ["one","three"])
    }
}
