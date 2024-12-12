use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

pub fn execute(path: &str) -> io::Result<()> {
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut stones: Vec<i64> = contents
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let stones_part2 = stones.clone();

    let mut blinks = 0;

    while blinks < 25 {
        let mut append_stones: Vec<i64> = Vec::new();

        for stone_index in 0..stones.len() {
            let stone_number = stones[stone_index];

            match stone_number {
                // Rule 1: Replace with a stone that is now number 1.
                0 => {
                    stones[stone_index] = 1;
                }
                // Rule 2: If the stone has an even number of digits, divide it into two stones.
                _ if stone_number.to_string().len() % 2 == 0 => {
                    let stone_str = stone_number.to_string();
                    let mid = stone_str.len() / 2;
                    let left: i64 = stone_str[..mid].parse().unwrap();
                    let right: i64 = stone_str[mid..].parse().unwrap();
        
                    stones[stone_index] = left;
                    append_stones.insert(0, right);
                }
                // Rule 3: Multiply the stone number by 2024.
                _ => {
                    stones[stone_index] = stone_number * 2024;
                }
            }
        }

        //Add the new stones.
        for append in append_stones {
            stones.insert(0, append);
        }
        blinks += 1;

        println!("Blink computed...");
    }

    println!("Stones after 25 blinks: {:?}", stones.len());

    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    // Part 2
    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    let mut cache = HashMap::new();

    let mut part2_total: i64 = 0;
    for &stone in stones_part2.iter() {
        part2_total += solve(stone, 75, &mut cache);
    }

    println!("Stones after 75 blinks: {}", part2_total);

    Ok(())
}

// Full disclosure, this is not entirely my solution
// and I needed some explanations on the memoization concept here.
// Other solutions using O(1) insertion structures were helpful for speed, but a cache for 
// already seen results seems to me the fastest solution.
fn solve(stone: i64, blinks: u32, cache: &mut HashMap<(i64, u32), i64>) -> i64 {
    if blinks == 0 {
        return 1;
    }

    // If we've seen this solution to this state before, we return with the value
    if let Some(&val) = cache.get(&(stone, blinks)) {
        return val;
    }

    let val = match stone {
        0 => solve(1, blinks - 1, cache),
        _ if stone.to_string().len() % 2 == 0 => {
            let stone_str = stone.to_string();
            let len = stone_str.len();
    
            let mid = len / 2;
            let left: i64 = stone_str[..mid].parse().unwrap();
            let right: i64 = stone_str[mid..].parse().unwrap();
    
            solve(left, blinks - 1, cache) + solve(right, blinks - 1, cache)
        }
        _ => solve(stone * 2024, blinks - 1, cache),
    };

    cache.insert((stone, blinks), val);
    val
}
