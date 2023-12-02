fn main() {
    let file_path: &str = "./src/input.txt";
    let content: String = std::fs::read_to_string(file_path).expect("should read from file");
    let input_vector = content.lines().collect::<Vec<&str>>();
    let final_sum = sum_of_input_vector(input_vector);
    println!("{}", final_sum)
}

pub fn sum_of_input_vector(input: Vec<&str>) -> i32 {
    input
        .iter()
        .map(|parsed_result| {
            isolate_first_and_last_string(parsed_result)
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

pub fn isolate_first_and_last_string(input: &str) -> String {
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
}
