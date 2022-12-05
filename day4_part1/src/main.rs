use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input file.");
    let lines: Vec<&str> = input.lines().collect();

    let range_results: Vec<i32> = lines.iter().map(|line| {
        let ranges: Vec<&str> = line.split(",").collect();

        let first_range_text: Vec<&str> = ranges[0].split("-").collect();
        let first_lower: i32 = first_range_text[0].parse().unwrap();
        let first_upper: i32 = first_range_text[1].parse().unwrap();
        let first_range = first_upper - first_lower;

        let second_range_text: Vec<&str> = ranges[1].split("-").collect();
        let second_lower: i32 = second_range_text[0].parse().unwrap();
        let second_upper: i32 = second_range_text[1].parse().unwrap();
        let second_range = second_upper - second_lower;

        if second_range > first_range {
            if second_lower <= first_lower && second_upper >= first_upper {
                return 1;
            }
        }
        if first_range > second_range {
            if first_lower <= second_lower && first_upper >= second_upper {
                return 1;
            }
        }
        if first_range == second_range {
            if first_lower == second_lower && first_upper == second_upper {
                return 1;
            }
        }

        return 0;
    }).collect();

    let range_result_sum: i32 = range_results.iter().sum();

    println!("Sum: {}", range_result_sum)

}
