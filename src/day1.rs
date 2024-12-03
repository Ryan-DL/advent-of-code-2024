use std::fs::File;
use std::io::{self, BufRead};

pub fn execute(path: &str) -> io::Result<()> {
    let file = File::open(&path)?;

    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut lists: (Vec<i32>, Vec<i32>) = reader
        .lines() // Read lines from the file
        .filter_map(Result::ok) // Ignore lines with errors
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let left = parts.next()?.parse::<i32>().ok()?;
            let right = parts.next()?.parse::<i32>().ok()?;
            Some((left, right))
        })
        .fold((Vec::new(), Vec::new()), |mut acc, (left, right)| {
            acc.0.push(left);
            acc.1.push(right);
            acc
        });

    lists.0.sort();
    lists.1.sort();

    let mut total_distance = 0;

    for i in 0..lists.0.len() {
        let left_smallest = lists.0.get(i).unwrap();
        let right_smallest = lists.1.get(i).unwrap();
        let distance = left_smallest.abs() - right_smallest.abs();
        total_distance += distance.abs();
    }

    println!("Total Distance is:  {:?}", total_distance);

    // =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    // Part 2
    // =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    let similarity_score: i32 = lists
        .0
        .iter()
        .map(|&number| {
            let occurrences_in_right = lists.1.iter().filter(|&&x| x == number).count() as i32;
            number * occurrences_in_right
        })
        .sum();

    println!("Total similiarty score: {:?}", similarity_score);

    Ok(())
}
