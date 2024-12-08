use std::collections::HashMap;
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

pub fn execute(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Create grid from file
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut antinodes_locations: HashSet<(usize, usize)> = HashSet::new();

    let mut character_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    // Populate character map
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != '.' {
                character_map
                    .entry(cell)
                    .or_insert_with(Vec::new)
                    .push((y, x));
            }
        }
    }

    // Process antenna locations
    for (_antenna, locations) in character_map.clone() {
        for c1 in 0..locations.len() {
            for c2 in c1 + 1..locations.len() {
                let coord1 = locations[c1];
                let coord2 = locations[c2];

                let dx = coord1.0 as i32 - coord2.0 as i32;
                let dy = coord1.1 as i32 - coord2.1 as i32;

                let antinode_1_x = coord1.0 as i32 - dx * 2;
                let antinode_1_y = coord1.1 as i32 - dy * 2;

                let antinode_2_x = coord2.0 as i32 + dx * 2;
                let antinode_2_y = coord2.1 as i32 + dy * 2;

                if antinode_1_x >= 0 && antinode_1_y >= 0 {
                    add_antinode(
                        (antinode_1_x as usize, antinode_1_y as usize),
                        &grid,
                        &mut antinodes_locations,
                    );
                }

                if antinode_2_x >= 0 && antinode_2_y >= 0 {
                    add_antinode(
                        (antinode_2_x as usize, antinode_2_y as usize),
                        &grid,
                        &mut antinodes_locations,
                    );
                }
            }
        }
    }

    println!("Total antinodes: {:?}", antinodes_locations.len());

    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    // Part 2 (Modified logic, mad verbose)
    //-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let max_dimension = rows.max(cols); // Determine the larger dimension

    let mut antinodes_locations_harmonic: HashSet<(usize, usize)> = HashSet::new();
    // Process antenna locations
    for (_antenna, locations) in character_map.clone() {
        // Weird case where we need to include the location of the antenna when found.
        if locations.len() > 1 {
            antinodes_locations_harmonic.extend(locations.clone());
        }

        for c1 in 0..locations.len() {
            for c2 in c1 + 1..locations.len() {
                let coord1 = locations[c1];
                let coord2 = locations[c2];

                let dx = coord1.0 as i32 - coord2.0 as i32;
                let dy = coord1.1 as i32 - coord2.1 as i32;

                // This is a somewhat lazy solution I realized after, but we can multiply through the number of max(rows, cols)
                // in the worst case two antennas are next to each other, we need to multply through as many potential
                // cells there may be.
                // The proper solution here is to go till we are out of bunds then break.
                for multiplier in 1..=max_dimension as i32 {
                    let antinode_1_x = coord1.0 as i32 - dx * multiplier;
                    let antinode_1_y = coord1.1 as i32 - dy * multiplier;

                    let antinode_2_x = coord2.0 as i32 + dx * multiplier;
                    let antinode_2_y = coord2.1 as i32 + dy * multiplier;

                    if antinode_1_x >= 0 && antinode_1_y >= 0 {
                        add_antinode(
                            (antinode_1_x as usize, antinode_1_y as usize),
                            &grid,
                            &mut antinodes_locations_harmonic,
                        );
                    }

                    if antinode_2_x >= 0 && antinode_2_y >= 0 {
                        add_antinode(
                            (antinode_2_x as usize, antinode_2_y as usize),
                            &grid,
                            &mut antinodes_locations_harmonic,
                        );
                    }
                }
            }
        }
    }
    println!("Total antinodes: {:?}", antinodes_locations_harmonic.len());

    Ok(())
}

fn add_antinode(
    location: (usize, usize),
    grid: &[Vec<char>],
    antinodes_locations: &mut HashSet<(usize, usize)>,
) {
    if !is_within_grid(grid, location) {
        return;
    }

    if !antinodes_locations.contains(&location) {
        antinodes_locations.insert(location);
    }
}

// Additioanl check to make sure we are shown on teh grid
fn is_within_grid(grid: &[Vec<char>], position: (usize, usize)) -> bool {
    let (row, col) = position;
    if let Some(row_data) = grid.get(row) {
        col < row_data.len()
    } else {
        false
    }
}
