use std::fs;

static ALPHA: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn main() {
    let alpha_codes = ALPHA.to_vec();

    let input = fs::read_to_string("input.txt").expect("Couldn't read input file.");
    let rucksacks: Vec<&str> = input.lines().collect();

    let rucksack_groups = rucksacks.chunks(3);

    let priority_sum: i32 = rucksack_groups
        .into_iter()
        .map(|rucksack_group| {
            let first_rucksack_chars = rucksack_group[0].chars();
            let second_rucksack = rucksack_group[1];
            let third_rucksack = rucksack_group[2];

            let mut common_item = '_';

            for item in first_rucksack_chars {
                match second_rucksack.find(item) {
                    None => {
                        continue;
                    }
                    Some(_) => match third_rucksack.find(item) {
                        None => {
                            continue;
                        }
                        Some(_) => {
                            common_item = item;
                            break;
                        }
                    },
                }
            }

            let priority = alpha_codes
                .iter()
                .position(|alpha_code| alpha_code == &common_item)
                .expect("Couldn't find an alpha code for {common_item}");

            return (priority as i32) + 1;
        })
        .sum();

    println!("Total: {priority_sum}");
}
