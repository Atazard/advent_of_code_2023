use std::fs;

fn main() {
    let input_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "\\src\\part_2_test_input.txt");

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
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calibrate_with_spelled_out_digits() {
        // Test with spelled-out digits
        let calibration_document = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .join("\n");

        assert_eq!(calibrate(&calibration_document), 281);
    }

    #[test]
    fn test_calibrate_with_mixed_digits() {
        // Test with a mix of numeric digits and spelled-out digits
        let calibration_document =
            vec!["1two3", "four5", "six7eight", "nine10", "eleven12"].join("\n");

        assert_eq!(calibrate(&calibration_document), 78);
    }
}
