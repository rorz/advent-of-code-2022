use std::fs;

static ALPHA: [char; 52] = [
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g',
    'h',
    'i',
    'j',
    'k',
    'l',
    'm',
    'n',
    'o',
    'p',
    'q',
    'r',
    's',
    't',
    'u',
    'v',
    'w',
    'x',
    'y',
    'z',
    'A',
    'B',
    'C',
    'D',
    'E',
    'F',
    'G',
    'H',
    'I',
    'J',
    'K',
    'L',
    'M',
    'N',
    'O',
    'P',
    'Q',
    'R',
    'S',
    'T',
    'U',
    'V',
    'W',
    'X',
    'Y',
    'Z',
];

fn main() {
    let alpha_codes = ALPHA.to_vec();

    let input = fs::read_to_string("input.txt").expect("Couldn't read input file.");
    let rucksacks: Vec<&str> = input.lines().collect();

    let priority_sum: i32 = rucksacks
        .into_iter()
        .map(|rucksack| {
            let compartment_size = rucksack.len() / 2;
            let first_compartment = &rucksack[..compartment_size];
            let second_compartment = &rucksack[compartment_size..];

            let mut common_item = '0';

            for item in first_compartment.chars() {
                let result = second_compartment.find(item);

                match second_compartment.find(item) {
                    None => {
                        continue;
                    }
                    Some(x) => {
                        common_item = item;
                        break;
                    }
                }
            }

            println!("common item: {common_item}");

            let priority = alpha_codes
                .iter()
                .position(|alpha_code| alpha_code == &common_item)
                .expect("Couldn't find an alpha code for {common_item}");

            return (priority as i32) + 1;
        })
        .sum();

    println!("Total: {priority_sum}");
}