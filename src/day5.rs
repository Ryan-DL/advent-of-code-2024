use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

pub fn execute(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    let mut bad_updates: Vec<Vec<u32>> = Vec::new(); // Part 2

    let mut is_parsing_pairs = true;
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            is_parsing_pairs = false;
            continue;
        }

        if is_parsing_pairs {
            if let Some((left, right)) = line.split_once('|') {
                let left: u32 = left.trim().parse().expect("Invalid number in pair");
                let right: u32 = right.trim().parse().expect("Invalid number in pair");
                ordering_rules
                    .entry(left)
                    .or_insert_with(Vec::new)
                    .push(right);
            }
        } else {
            let group: Vec<u32> = line
                .split(',')
                .map(|num| num.trim().parse().expect("Invalid number in group"))
                .collect();
            updates.push(group);
        }
    }

    let mut total = 0;
    for update_list in updates {
        let mut update_is_valid = true;

        for (index, update) in update_list.iter().enumerate() {
            if ordering_rules.get(update) == None {
                continue;
            }
            let ordering_rules_for_this_number = ordering_rules.get(update).unwrap();

            // Get all the indencies for all of the numbers that appear in the update list.
            let all_indices: Vec<usize> = update_list
                .iter()
                .enumerate()
                .filter(|&(_, &x)| ordering_rules_for_this_number.contains(&x))
                .map(|(i, _)| i)
                .collect();

            //If any of the indecies appear prior to the current update were looking at, big yikes.
            for i in all_indices {
                if i < index {
                    update_is_valid = false;
                }
            }
        }

        if update_is_valid {
            println!("Update is valid {:?}", update_list);
            total += get_middle_value(&update_list);
        } else {
            println!("Update is not valid: {:?}", update_list);
            bad_updates.push(update_list.clone());
        }
    }

    println!(
        "Total of the middle of all the valid update lists: {:?}",
        total
    );
    //-=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=-
    // Part 2
    //-=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=--=-
    let mut total_part_2 = 0;

    println!("Starting with {:?}", bad_updates.len());
    for mut bad_update in bad_updates {
        //BUBBLE SORT BABY
        bubble_sort(&mut bad_update, ordering_rules.clone());

        let mut update_is_valid = true;

        for (index, update) in bad_update.iter().enumerate() {
            if ordering_rules.get(update) == None {
                continue;
            }
            let ordering_rules_for_this_number = ordering_rules.get(update).unwrap();

            // Get all the indencies for all of the numbers that appear in the update list.
            let all_indices: Vec<usize> = bad_update
                .iter()
                .enumerate()
                .filter(|&(_, &x)| ordering_rules_for_this_number.contains(&x))
                .map(|(i, _)| i)
                .collect();

            //If any of the indecies appear prior to the current update were looking at, big yikes.
            for i in all_indices {
                if i < index {
                    update_is_valid = false;
                }
            }
        }

        if update_is_valid {
            total_part_2 += get_middle_value(&bad_update);
            //For part 2
        } else {
            println!("Update is not valid: {:?}", bad_update);
        }
    }
    println!(
        "Total of the middle of all the validated lists lists: {:?}",
        total_part_2
    );

    Ok(())
}

fn get_middle_value(update_list: &Vec<u32>) -> u32 {
    let middle_index = update_list.clone().len() / 2;
    update_list[middle_index]
}

// O(N^2) but I don't care to figure out a better way.
fn bubble_sort(arr: &mut Vec<u32>, ordering_rules: HashMap<u32, Vec<u32>>) {
    let mut n = arr.len();
    while n > 1 {
        let mut new_n = 0;
        for i in 1..n {
            let first = arr[i - 1];
            let second = arr[i];

            let should_swap = if let Some(rules) = ordering_rules.get(&first) {
                !rules.contains(&second)
            } else if let Some(rules) = ordering_rules.get(&second) {
                rules.contains(&first)
            } else {
                false
            };

            if should_swap {
                arr.swap(i - 1, i);
                new_n = i;
            }
        }
        n = new_n;
    }
}
