use regex::Regex;
use std::fs;
use std::io::{self};

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
}

pub fn execute(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;

    // Define the regex pattern
    let pattern = r"mul\(([\d,]*)\)";
    let re = Regex::new(pattern).unwrap();

    // Find all matches
    let matches: Vec<&str> = re.find_iter(&contents).map(|m| m.as_str()).collect();

    // Print the matches
    let mut total = 0;
    for m in matches {
        let pattern = r"\(([^)]*)\)";
        let re = Regex::new(pattern).unwrap();

        let matched_string = if let Some(captures) = re.captures(m) {
            captures[1].to_string()
        } else {
            continue;
        };

        let numbers: Vec<Option<i32>> = matched_string
            .split(",")
            .map(|x| x.parse::<i32>().ok())
            .collect();

        let all_present = !numbers.iter().any(|&value| value.is_none());

        if all_present {
            let product: i32 = numbers
                .iter()
                //this needs to be accounted for, but ignored because we're already confdient there are no Nones
                .filter_map(|&x| x)
                .product();

            total += product;
        }
    }

    println!("Total product: {:?}", total);

    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    // Part 2 (BRUTE FORCED WITH SLIDING WINDOW, YIKES)
    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"\bdon't\(\)").unwrap();
    let mul_re = Regex::new(r"mul\(([\d,]*)\)").unwrap();

    let mut window_index = 0;
    let mut instruction = Instruction::Do;
    let mut part2_total = 0;

    while window_index < contents.len() {
        let subset = &contents[window_index..];

        //Definitely not stolen from stackoverflow
        if let Some((index, command)) = [
            (do_re.find(subset).map(|mat| mat.start()), "do()"),
            (dont_re.find(subset).map(|mat| mat.start()), "dont()"),
            (mul_re.find(subset).map(|mat| mat.start()), "mul("),
        ]
        .iter()
        .filter_map(|&(idx, cmd)| idx.map(|i| (i, cmd)))
        .min_by_key(|&(i, _)| i)
        {
            match command {
                "do()" => {
                    println!("`do()` found at index {}", window_index + index);
                    instruction = Instruction::Do;
                    window_index += index + 4; // Move past `do()`
                }
                "dont()" => {
                    println!("`don't()` found at index {}", window_index + index);
                    instruction = Instruction::Dont;
                    window_index += index + 7; // Move past `don't()`
                }
                "mul(" => {
                    if let Some(captures) = mul_re.captures(subset) {
                        let matched_string = &captures[1];
                        let numbers: Vec<i32> = matched_string
                            .split(',')
                            .filter_map(|x| x.parse::<i32>().ok())
                            .collect();

                        if numbers.len() == matched_string.split(',').count() {
                            let product: i32 = numbers.iter().product();
                            if let Instruction::Do = instruction {
                                part2_total += product;
                            }
                        }

                        println!(
                            "Processed `mul` at index {}: {:?}",
                            window_index + index,
                            numbers
                        );
                    }

                    let subset_slice = &subset[index..];
                    // Find the position of the closing parenthesis in the slice
                    let closing_parenthesis_index = subset_slice.find(')').unwrap_or(0);

                    // Add 'index', the position of the closing parenthesis, and 1 to 'window_index'
                    window_index = window_index + index + closing_parenthesis_index + 1;
                }
                _ => unreachable!(),
            }
        } else {
            break;
        }
    }

    println!(
        "\nTotal decoded given the on/off instructions: {:?}",
        part2_total
    );
    Ok(())
}
