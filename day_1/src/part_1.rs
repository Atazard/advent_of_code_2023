use std::fs;

fn main() {
    let input_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "\\src\\_part_1_input.txt");

    let calibration_document = match fs::read_to_string(input_file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading {}: {}", input_file_path, err);
            return;
        }
    };

    let sum_of_calibration_values = calibrate(&calibration_document);
    println!(
        "The sum of the calibration values is: {}",
        sum_of_calibration_values
    );
}

fn calibrate(calibration_document: &str) -> u32 {
    calibration_document
        .lines()
        .map(|line| {
            let first_digit = line.chars().find(|c| c.is_ascii_digit()).unwrap_or('0');
            let last_digit = line.chars().rev().find(char::is_ascii_digit).unwrap_or('0');
            let combined_number = format!("{}{}", first_digit, last_digit);
            combined_number.parse::<u32>().unwrap_or(0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibration_example() {
        let calibration_document =
            vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"].join("\n");

        // Test the calibrate function with the example calibration document
        assert_eq!(calibrate(&calibration_document), 142);
    }

    #[test]
    fn test_empty_calibration() {
        // Test with an empty calibration document
        assert_eq!(calibrate(""), 0);
    }

    #[test]
    fn test_single_digit_calibration() {
        // Test with a calibration document containing a few digit values
        let calibration_document = "12345";
        assert_eq!(calibrate(calibration_document), 15);
    }

    #[test]
    fn test_no_digit_calibration() {
        // Test with a calibration document containing lines without digits
        let calibration_document = "abc";
        assert_eq!(calibrate(calibration_document), 0);
    }
}
