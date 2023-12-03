use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let input_file_path = concat!(env!("CARGO_MANIFEST_DIR"), "\\src\\part_1_input.txt");

    let limits: HashMap<&str, u32> = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect::<HashMap<&str, u32>>();

    let game_descriptions = fs::read_to_string(input_file_path).unwrap_or(String::new());

    let games: Vec<&str> = game_descriptions.lines().collect();

    let possible_games: Vec<&str> = games
        .iter()
        .filter(|&&game| is_game_possible(game, &limits))
        .cloned()
        .collect();

    //Example: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let possible_game_ids: Vec<u32> = possible_games
        .iter()
        //split at whitespace and get the next value (in this example '1:')
        .filter_map(|&game| game.split_whitespace().nth(1))
        //trim the ':' and parse the id
        .filter_map(|game_id| game_id.trim_matches(':').parse().ok())
        .collect();

    let sum_of_possible_ids: u32 = possible_game_ids.iter().sum();

    println!(
        "The sum of IDs of possible games is: {}",
        sum_of_possible_ids
    );
}

fn is_game_possible(game_description: &str, limits: &HashMap<&str, u32>) -> bool {
    let mut cube_counts = HashMap::new();
    cube_counts.insert("red", 0);
    cube_counts.insert("green", 0);
    cube_counts.insert("blue", 0);

    //Remove the "Game x:" part
    let re = Regex::new(r"Game \d+:").unwrap();
    let game_description_without_game_id = re.replace(game_description, "");

    let game_parts = game_description_without_game_id.split(';');

    for part in game_parts {
        let color_counts = part.split(',');
        for color_count in color_counts {
            let count_and_color: Vec<&str> = color_count.split_whitespace().collect();
            if let [count, color] = count_and_color.as_slice() {
                *cube_counts.entry(color).or_insert(0) += count.parse::<u32>().unwrap();
            }
        }
    }

    println!("{}", game_description);
    println!("{:?}", cube_counts);

    for (color, count) in &cube_counts {
        if count > limits.get(color).unwrap() {
            println!("Color: {} exceeds limit with count: {}\n", color, count);
            return false;
        }
    }

    println!();
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_game_possible_case_example_game_1() {
        let limits: HashMap<&str, u32> = [("red", 12), ("green", 13), ("blue", 14)]
            .iter()
            .cloned()
            .collect::<HashMap<&str, u32>>();

        assert!(is_game_possible(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            &limits
        ));
    }

    #[test]
    fn test_is_game_possible_case_example_game_2() {
        let limits: HashMap<&str, u32> = [("red", 12), ("green", 13), ("blue", 14)]
            .iter()
            .cloned()
            .collect::<HashMap<&str, u32>>();

        assert!(is_game_possible(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            &limits
        ));
    }

    #[test]
    fn test_is_game_possible_case_example_game_3() {
        let limits: HashMap<&str, u32> = [("red", 12), ("green", 13), ("blue", 14)]
            .iter()
            .cloned()
            .collect::<HashMap<&str, u32>>();

        assert!(!is_game_possible(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            &limits
        ));
    }

    #[test]
    fn test_is_game_possible_case_example_game_4() {
        let limits: HashMap<&str, u32> = [("red", 12), ("green", 13), ("blue", 14)]
            .iter()
            .cloned()
            .collect::<HashMap<&str, u32>>();

        assert!(!is_game_possible(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            &limits
        ));
    }

    #[test]
    fn test_is_game_possible_case_example_game_5() {
        let limits: HashMap<&str, u32> = [("red", 12), ("green", 13), ("blue", 14)]
            .iter()
            .cloned()
            .collect::<HashMap<&str, u32>>();

        assert!(is_game_possible(
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            &limits
        ));
    }
}
