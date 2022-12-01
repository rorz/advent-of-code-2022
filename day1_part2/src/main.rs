use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input file...");

    let elf_lists: Vec<&str> = input.split("\n\n").collect();

    let mut calorie_totals: Vec<i32> = vec![];

    for elf_list in elf_lists {
        let elf_items: Vec<&str> = elf_list.lines().collect();
        let mut calories_total = 0;

        for elf_item in elf_items {
            let item_calories: i32 = elf_item.trim().parse().unwrap();
            calories_total += item_calories;
        }

        calorie_totals.push(calories_total);
    }

    calorie_totals.sort();

    let last_three_calorie_totals_sum: i32 = calorie_totals.iter().rev().take(3).sum();

    println!(
        "The highest three calorie totals' total is: {}",
        last_three_calorie_totals_sum
    );
}
