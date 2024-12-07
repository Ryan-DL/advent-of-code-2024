use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn execute(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Read the grid into a 2D array of characters
    let grid: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|line| line.ok()) // Filter out errors
        .map(|line| line.chars().collect()) // Convert each line to a Vec<char>
        .collect();

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let starting_coordinates = find_coordinates(&grid, '^').unwrap();

    let mut current_position = starting_coordinates;
    let mut current_direction = Direction::North;

    let mut seen_positions: Vec<(usize, usize)> = Vec::new();
    seen_positions.push(current_position);

    while true {
        let movement = calculate_next_move(
            &grid,
            current_position,
            current_direction.clone(),
            rows,
            cols,
        );
        if movement.1.is_none() {
            break;
        }

        let new_direction = movement.0;
        let new_position = movement.1.unwrap();

        //Dont count turns!
        if current_direction == new_direction {
            if !seen_positions.contains(&(new_position.0, new_position.1)) {
                seen_positions.push((new_position.0, new_position.1));
            }
        }
        //Cleanup
        current_position = new_position;
        current_direction = new_direction;
    }
    println!(
        "Total moves taken before leaving the board: {:?}",
        seen_positions.len()
    );

    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    // Part 2
    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    // I want it clear that if another human reads this, this is a suboptimal brute force solution
    // the proper way would be to treat this as a cyclical graph and look for cycles.
    let mut total_infinite_loops = 0;

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, &ch) in row.iter().enumerate() {
            let mut copy_of_grid = grid.clone();
            copy_of_grid[row_index][col_index] = '#';

            let mut current_position = find_coordinates(&grid, '^').unwrap();
            let mut current_direction = Direction::North;

            let mut total_moves = 0;

            while true {
                let movement = calculate_next_move(
                    &copy_of_grid,
                    current_position,
                    current_direction.clone(),
                    rows,
                    cols,
                );
                if movement.1.is_none() {
                    break;
                }

                let new_direction = movement.0;
                let new_position = movement.1.unwrap();

                //Dont count turns!
                if current_direction == new_direction {
                    total_moves += 1;

                    //Theres probably a better way to know this
                    //We should have escaped the board at this point, meaning we're stuck.
                    if total_moves > (rows * cols) {
                        total_infinite_loops += 1;
                        break;
                    }
                }
                current_position = new_position;
                current_direction = new_direction;
            }
        }
    }

    println!("Total infinite loops: {:?}", total_infinite_loops);

    Ok(())
}

// Return the new direction and the next position.
fn calculate_next_move(
    grid: &Vec<Vec<char>>,
    current_position: (usize, usize),
    direction: Direction,
    rows: i32,
    cols: i32,
) -> (Direction, Option<(usize, usize)>) {
    let next_cell: (i32, i32) = match direction {
        Direction::North => (current_position.0 as i32 - 1, current_position.1 as i32),
        Direction::South => (current_position.0 as i32 + 1, current_position.1 as i32),
        Direction::East => (current_position.0 as i32, current_position.1 as i32 + 1),
        Direction::West => (current_position.0 as i32, current_position.1 as i32 - 1),
    };

    let safe_cell = is_within_grid(next_cell, rows, cols);
    if !safe_cell {
        //We're done here and found our exit path.
        return (direction, None);
    }

    let next_cell_char = grid[next_cell.0 as usize][next_cell.1 as usize];

    //Obstacle found!
    if next_cell_char == '#' {
        let next_direction: Direction = next_direction(direction);
        // We update the direction and return the the original position, because it was unsafe to move.
        // Our outer loop will check if the direction changes, if it does, we do not consider that a "new" position.
        return (next_direction, Some(current_position));
    } else {
        return (
            direction,
            Some((next_cell.0 as usize, next_cell.1 as usize)),
        );
    }
}

fn is_within_grid(proposed_next_position: (i32, i32), rows: i32, cols: i32) -> bool {
    let safe_row = proposed_next_position.0 >= 0 && proposed_next_position.0 < rows;
    let safe_col = proposed_next_position.1 >= 0 && proposed_next_position.1 < cols;
    return safe_row && safe_col;
}

fn next_direction(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::East => Direction::South,
        Direction::West => Direction::North,
    }
}

// Function to find the coordinates of a target character in the grid
fn find_coordinates(grid: &[Vec<char>], target: char) -> Option<(usize, usize)> {
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == target {
                return Some((row_idx, col_idx));
            }
        }
    }
    None
}
