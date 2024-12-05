use std::collections::HashSet;
use std::fs;
use std::io::{self};

pub fn execute(path: &str) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    //Representes cells that have been visited already.
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &char) in row.iter().enumerate() {

            if visited.contains(&(row_idx.try_into().unwrap(), col_idx.try_into().unwrap())) {
                continue;
            }

            if char == 'X' {
                dfs(&mut visited, &grid, (row_idx, col_idx), None, &vec!['X']);
            }
        }
    }

    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    // Part 2;
    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    let mut total_mas = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &char) in row.iter().enumerate() {
            if char == 'A' {
                if is_mas((row_idx, col_idx), &grid) {
                    total_mas += 1;
                }
            }
        }
    }

    println!("Total mas: {:?}", total_mas);

    Ok(())
}

fn next_letter(c: char) -> Option<char> {
    match c {
        'X' => Some('M'),
        'M' => Some('A'),
        'A' => Some('S'),
        _ => None,
    }
}

fn is_xmas(accumulated_chars: Vec<char>) -> bool {
    accumulated_chars == vec!['X', 'M', 'A', 'S']
}

fn dfs(
    visited: &mut HashSet<(i32, i32)>,
    graph: &Vec<Vec<char>>,
    current: (usize, usize),
    last_direction: Option<(i32, i32)>,
    accumulated_chars: &Vec<char>,
) {
    let directions: Vec<(i32, i32)> = vec![
        (0, 1),   // North
        (1, 1),   // Northeast
        (1, 0),   // East
        (1, -1),  // Southeast
        (0, -1),  // South
        (-1, -1), // Southwest
        (-1, 0),  // West
        (-1, 1),  // Northwest
    ];

    let rows = graph.len();
    let cols = graph.get(0).unwrap().len() as i32;

    //Base case, if we've found XMAS 
    if is_xmas(accumulated_chars.to_vec()) {
        // This is a bit of a hack to let me know the total amount of christmas's found,
        // I use stdout to know the amount of lines printed so I'm not dealing with an accumulator in the code.
        println!("Found a christmas!");
        return;
    }

    //Don't visit visited nodes, I guess.
    if visited.contains(&(current.0 as i32, current.1 as i32)) {
        return;
    }

    // Add the current node to visited.
    visited.insert((current.0 as i32, current.1 as i32));

    let current_char: char = graph[current.0][current.1];

    for direction in &directions {
        if last_direction != None {
            let last_dir = last_direction.unwrap();
            if direction.clone() != last_dir {
                //We don't care about zig zagging items, directions must match.
                visited.remove(&(current.0 as i32, current.1 as i32));
                continue;
            }
        }

        //Bounds checking
        let inspect_row: i32 = current.0 as i32 + direction.0;
        let inspect_col: i32 = current.1 as i32 + direction.1;

        let safe_row: bool = inspect_row >= 0 && inspect_row < rows as i32;
        let safe_col: bool = inspect_col >= 0 && inspect_col < cols;
        if !safe_row || !safe_col {
            continue;
        }

        let potential_next_char = graph[inspect_row as usize][inspect_col as usize];

        //We acutally don't need this now but its still good to have
        if let Some(next_char) = next_letter(current_char) {
            if next_char == potential_next_char {
                let mut new_vec = accumulated_chars.clone();
                new_vec.push(next_char);
                dfs(
                    visited,
                    graph,
                    (inspect_row as usize, inspect_col as usize),
                    Some(*direction),
                    &new_vec,
                );
            }
        }
    }
    visited.remove(&(current.0 as i32, current.1 as i32));
}

fn is_mas(current: (usize, usize), graph: &Vec<Vec<char>>) -> bool {
    let rows = graph.len();
    let cols = graph.get(0).unwrap().len() as i32;

    let left_to_right_directions: Vec<(i32, i32)> = vec![
        (-1, 1), // Northwest
        (1, -1), // Southeast
    ];

    let right_to_left_directions: Vec<(i32, i32)> = vec![
        (1, 1),   // Northeast
        (-1, -1), // Southwest
    ];

    let mut left_to_right_text: Vec<char> = Vec::new();
    let mut right_to_left_text: Vec<char> = Vec::new();

    for direction in &left_to_right_directions {
        let inspect_row: i32 = current.0 as i32 + direction.0;
        let inspect_col: i32 = current.1 as i32 + direction.1;

        let safe_row: bool = inspect_row >= 0 && inspect_row < rows as i32;
        let safe_col: bool = inspect_col >= 0 && inspect_col < cols;
        if !safe_row || !safe_col {
            return false;
        }
        let potential_next_char = graph[inspect_row as usize][inspect_col as usize];
        left_to_right_text.push(potential_next_char);
    }

    for direction in &right_to_left_directions {
        let inspect_row: i32 = current.0 as i32 + direction.0;
        let inspect_col: i32 = current.1 as i32 + direction.1;

        let safe_row: bool = inspect_row >= 0 && inspect_row < rows as i32;
        let safe_col: bool = inspect_col >= 0 && inspect_col < cols;
        if !safe_row || !safe_col {
            return false;
        }
        let potential_next_char = graph[inspect_row as usize][inspect_col as usize];
        right_to_left_text.push(potential_next_char);
    }

    if contains_ms_or_sm(&left_to_right_text) && contains_ms_or_sm(&right_to_left_text) {
        return true;
    }

    // No mas
    return false;
}

fn contains_ms_or_sm(chars: &Vec<char>) -> bool {
    let string_rep: String = chars.iter().collect(); // Convert Vec<char> to String
    string_rep.contains("MS") || string_rep.contains("SM")
}
