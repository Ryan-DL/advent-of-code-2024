use std::{
    fs::File,
    io::{self, BufReader, Read},
};
pub fn execute(path: &str) -> io::Result<()> {
    let part1 = false;
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let chars: Vec<char> = buffer.into_iter().map(|byte| byte as char).collect();

    let mut finalized_string: Vec<String> = Vec::new();
    let mut block_id: u32 = 0;

    for (index, digit) in chars.iter().map(|c| c.to_digit(10).unwrap()).enumerate() {
        let is_block_number: bool = index % 2 == 0; //Even are block files, odd are free space.

        //Block logic
        if is_block_number {
            let mut blocks = repeat_string(digit as usize, &block_id.to_string());
            finalized_string.append(&mut blocks);
            block_id += 1;

        // Free space logic
        } else {
            let mut periods = repeat_string(digit as usize, ".");
            finalized_string.append(&mut periods);
        }
    }

    if part1 {
        while !is_compacted(&finalized_string) {
            let last_char = get_last_index(&finalized_string);
            let first_period = find_first_dot(&finalized_string);

            match (last_char, first_period) {
                (Some(last), Some(first)) => {
                    finalized_string.swap(last, first);
                }
                _ => {
                    println!("Oops, something went wrong.");
                }
            }
        }
    }

    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    // Part 2
    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    //Rolling down the block IDS as an i32, easier to work with.
    let mut rolldown = block_id as i32;

    if part1 == false {
        rolldown = -1; // We're just gonna skip
    }

    while rolldown >= 0 {
        let number_to_move: Vec<String> = finalized_string
            .iter()
            .filter(|x| *x == &rolldown.to_string())
            .cloned()
            .collect();

        //Search space reduction so we're not looking at periods after for replacement.
        let left_index = finalized_string
            .iter()
            .position(|s| s == &rolldown.to_string());
        if left_index.is_none() {
            //This number doesnt exist in teh string, oh well.
            rolldown -= 1;
            continue;
        }

        let free_space: Vec<(usize, usize)> =
            find_contiguous_spaces(&finalized_string, left_index.unwrap());
        for (amount, first_index) in free_space {
            if number_to_move.len() > amount {
                continue;
            }

            finalized_string.iter_mut().for_each(|s| {
                if *s == rolldown.to_string() {
                    *s = ".".to_string();
                }
            });

            for i in 0..number_to_move.len() {
                finalized_string[first_index + i] = rolldown.to_string();
            }
            break;
        }

        rolldown -= 1;
    }

    //Checksum logic is the same for both parts 1 and 2.
    let mut checksum: i64 = 0;
    for (index, val) in finalized_string.iter().enumerate() {
        if val == "." {
            continue;
        }
        let i = val.parse::<i64>().unwrap();
        checksum += i * index as i64;
    }
    println!("Checksum for computed file is: {:?}", checksum);

    Ok(())
}

fn repeat_string(amount: usize, input: &str) -> Vec<String> {
    vec![input.to_string(); amount]
}

fn is_compacted(input: &Vec<String>) -> bool {
    let mut seen_period = false;

    for item in input {
        if item == "." {
            // If we encounter a period
            seen_period = true;
        } else if seen_period {
            // If we see a character after a period, we're not fully compacted.
            return false;
        }
    }
    true
}

fn get_last_index(vec: &Vec<String>) -> Option<usize> {
    vec.iter()
        .enumerate()
        .rev()
        .find(|&(_, s)| s != ".")
        .map(|(index, _)| index)
}

fn find_first_dot(vec: &Vec<String>) -> Option<usize> {
    vec.iter()
        .enumerate()
        .find(|&(_, s)| s == ".")
        .map(|(index, _)| index)
}
fn find_contiguous_spaces(vec: &Vec<String>, left_index: usize) -> Vec<(usize, usize)> {
    let mut results = Vec::new();
    let mut current_count = 0;
    let mut current_start = None;

    for (i, item) in vec.iter().enumerate() {
        if i > left_index {
            break;
        }

        if item == "." {
            if current_start.is_none() {
                current_start = Some(i);
            }
            current_count += 1;
        } else {
            if let Some(start) = current_start {
                results.push((current_count, start));
                current_count = 0;
                current_start = None;
            }
        }
    }
    if let Some(start) = current_start {
        results.push((current_count, start));
    }
    results
}
