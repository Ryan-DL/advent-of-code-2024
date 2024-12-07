use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn execute(path: &str) -> io::Result<()> {
    let input_file = File::open(&path)?;
    let reader = io::BufReader::new(input_file);
    let equations: Vec<(u64, Vec<u64>)> = reader
        .lines()
        .filter_map(|line| line.ok()) // Filter out any lines that fail to read
        .filter(|line| !line.trim().is_empty()) // Skip empty lines
        .map(|line| {
            let parts: Vec<&str> = line.split(':').map(str::trim).collect();
            let key = parts[0].parse::<u64>().expect("Invalid number for key");
            let values = parts[1]
                .split_whitespace()
                .map(|val| val.parse::<u64>().expect("Invalid number in values"))
                .collect();
            (key, values)
        })
        .collect();

    let mut solution_total = 0;
    for equation in equations {
        let (solution, factors) = equation;

        //Flip this on or off to generate permutations for concatenation.
        let part2 = true;

        let operator_permutations = generate_char_permutations(factors.len() - 1, part2);
        // True means multiply, false means sum.
        for (_index, operators) in operator_permutations.iter().enumerate() {
            let mut total = 0;

            for i in 0..factors.len() {
                if i == 0 {
                    // Add the first element so we not adding to zero or something.
                    total += factors[i];
                    continue;
                }

                let operator = operators.get(i - 1).unwrap();
                match operator {
                    'm' => total = total * factors[i],
                    'a' => total = total + factors[i],
                    'c' => {
                        total = (total.to_string() + &factors[i].to_string())
                            .parse::<u64>()
                            .unwrap()
                    }
                    _ => panic!("oh no"),
                }
            }

            if total == solution {
                solution_total += total;
                break;
            }
        }
    }
    println!("Solution Total: {:?}", solution_total);

    Ok(())
}

fn generate_char_permutations(length: usize, part2: bool) -> Vec<Vec<char>> {
    if length == 0 {
        return vec![vec![]];
    }

    // Get permutations for length - 1
    let smaller_permutations = generate_char_permutations(length - 1, part2);
    let mut result = Vec::new();

    for perm in smaller_permutations {
        let mut with_m = perm.clone();
        with_m.push('m'); //Multiply
        result.push(with_m);

        let mut with_a = perm.clone();
        with_a.push('a'); // Add
        result.push(with_a);

        if part2 {
            let mut with_c = perm.clone();
            with_c.push('c'); // Concat
            result.push(with_c);
        }
    }

    result
}
