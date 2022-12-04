use std::fs;

fn main() {
    let input = fs::read_to_string("input.example.txt").expect("Couldn't read input file.");
    let lines: Vec<&str> = input.lines().collect();

    // for line in lines.iter() {
    //     println!("Line? {line}");
    // }

    let range_pairs = lines.iter().map(|line| {
        let ranges: Vec<&str> = line.split(",").collect();
        
        let first_range: Vec<&str> = ranges[0].split("-").collect();
        let second_range: Vec<&str> = ranges[1].split("-").collect();

        let first_lower = first_range.split("")


        let range_numbers: Vec<i32> = line
            .split(",")
            .flat_map(|range| {
                range
                    .split("-")
                    .map(|range_index| range_index.parse().unwrap())
            })
            .collect();

        let range_chunks = range_numbers.chunks(2);

        let largest_idx: i32 = range_chunks
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .expect("Cant find the largest index!") as i32;

        let smaller_idx = if largest_idx == 0 { 1 } else { 0 };   
        // let ranges: Vec<i32> = range_chunks
        //     .map(|range_chunk| range_chunk[1] - range_chunk[0])
        //     .max

        // println!("Largest index: {largest_idx}");

        // return largest_idx;

        let larger_range:  = range_chunks.collect()[0];
        let smaller_range = range_chunks[smaller_idx];




    });

    println!("Goodbye 😊");
}
