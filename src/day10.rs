use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{self, BufRead},
};

pub fn execute(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut grid: Vec<Vec<i32>> = Vec::new();

    let part1 = false;

    for line in reader.lines() {
        let line = line?;
        let row: Vec<i32> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i32)
            .collect();
        grid.push(row);
    }

    let mut results = vec![];

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &digit) in row.iter().enumerate() {
            if digit == 0 {
                if part1 {
                    let score = bfs_trailhead_score(&grid, (row_idx, col_idx));
                    results.push(((row_idx, col_idx), score));
                } else {
                    let rating =
                        dfs_trailhead_rating(&grid, (row_idx, col_idx), &mut HashSet::new());
                    results.push(((row_idx, col_idx), rating));
                }
            }
        }
    }

    let mut res: usize = 0;
    for ((row, col), score) in results {
        println!("Start at ({} , {}): {} unique paths to 9", row, col, score);
        res += score
    }

    println!("Total Score: {:?}", res);

    Ok(())
}

fn bfs_trailhead_score(grid: &Vec<Vec<i32>>, start: (usize, usize)) -> usize {
    let directions = vec![
        (0, 1),  // Right
        (1, 0),  // Down
        (0, -1), // Left
        (-1, 0), // Up
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();
    let mut reachable_nines = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some(((row, col), current_height)) = queue.pop_front() {
        if visited.contains(&(row, col)) {
            continue;
        }
        visited.insert((row, col));

        let current_value = grid[row][col];

        if current_value != current_height {
            continue;
        }

        // If height is 9, add to reachable set
        if current_value == 9 {
            reachable_nines.insert((row, col));
            continue;
        }

        // Explore valid neighbors
        for &(d_row, d_col) in &directions {
            let new_row = row as isize + d_row;
            let new_col = col as isize + d_col;
            if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                let neighbor_value = grid[new_row][new_col];
                if neighbor_value == current_height + 1 {
                    queue.push_back(((new_row, new_col), neighbor_value));
                }
            }
        }
    }

    reachable_nines.len()
}

fn dfs_trailhead_rating(
    grid: &Vec<Vec<i32>>,
    current: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    let directions = vec![
        (0, 1),  // Right
        (1, 0),  // Down
        (0, -1), // Left
        (-1, 0), // Up
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let (row, col) = current;

    // Prevent revisiting cells in the current path
    if visited.contains(&current) {
        return 0;
    }
    visited.insert(current);

    let current_height = grid[row][col];
    if current_height == 9 {
        visited.remove(&current); // Backtrack
        return 1; // Found a valid trail
    }

    let mut count = 0;

    for &(direction_row, direction_col) in &directions {
        let new_row = row as isize + direction_row;
        let new_col = col as isize + direction_col;

        if new_row >= 0 && new_row < rows as isize && new_col >= 0 && new_col < cols as isize {
            let new_row = new_row as usize;
            let new_col = new_col as usize;
            let neighbor_height = grid[new_row][new_col];

            if neighbor_height == current_height + 1 {
                count += dfs_trailhead_rating(grid, (new_row, new_col), visited);
            }
        }
    }

    visited.remove(&current); // Backtrack
    count
}
