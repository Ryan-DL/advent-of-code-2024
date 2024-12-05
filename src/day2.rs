use std::fs::File;
use std::io::{self, BufRead};

enum Ordering {
    Accending,
    Decending,
}

pub fn execute(path: &str) -> io::Result<()> {
    let file = File::open(&path)?;

    let reader: io::BufReader<File> = io::BufReader::new(file);

    let reports: Vec<Vec<i32>> = reader
        .lines() // Read lines from the file
        .filter_map(Result::ok)
        .map(|line| {
            let parts: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();
            parts
        })
        .collect();

    let mut safe_reports = 0;

    for report in reports.clone() {
        if is_safe_report(report) {
            safe_reports += 1;
        }
    }
    println!("Total safe reports {:?}", safe_reports);

    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    // Part 2
    // -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
    let mut safe_reports_with_dampener = 0;

    for mut report in reports {
        // Very nasty way of checking if there are more increasing or decreasing numbers. I'm sure theres a better way of doing this.
        let (increases, decreases) = report.windows(2).fold((0, 0), |(inc, dec), pair| {
            if pair[1] > pair[0] {
                (inc + 1, dec)
            } else if pair[1] < pair[0] {
                (inc, dec + 1)
            } else {
                (inc, dec)
            }
        });

        let ordering = if increases > decreases {
            Ordering::Accending
        } else {
            Ordering::Decending
        };

        let mut scary_indexs: Vec<usize> = Vec::new();
        for (index, current) in report.iter().enumerate() {
            if index == 0 {
                continue;
            }
            let last = report[index - 1];

            //IF the ordering is one way, but we see the opposite, we should be concerned about this.
            match ordering {
                Ordering::Accending => {
                    if last > *current {
                        scary_indexs.push(index - 1);

                        //Handle the edge case of the ending
                        if index == (report.len() - 1) {
                            scary_indexs.push(index);
                        }
                    }
                }
                Ordering::Decending => {
                    if last < *current {
                        scary_indexs.push(index - 1);
                        //Handle the edge case of the ending
                        if index == (report.len() - 1) {
                            scary_indexs.push(index);
                        }
                    }
                }
            }

            let diff = (current - last).abs();
            if diff == 0 || diff > 3 {
                scary_indexs.push(index - 1);

                if index == (report.len() - 1) {
                    scary_indexs.push(index);
                }
            }
        }

        // Try removing all the indexes we think there may be a problem with, and see if they are safe.
        for index in scary_indexs.clone() {
            let mut cloned_report = report.clone();
            cloned_report.remove(index);

            if is_safe_report(cloned_report) {
                safe_reports_with_dampener += 1;
                break;
            }
        }

        if scary_indexs.len() == 0 {
            if is_safe_report(report) {
                safe_reports_with_dampener += 1;
            }
        }
    }
    println!(
        "Total safety reports with dampener {:?}",
        safe_reports_with_dampener
    );
    Ok(())
}

fn is_safe_report(report: Vec<i32>) -> bool {
    let increasing = report.windows(2).all(|pair| pair[0] < pair[1]);
    let decreasing = report.windows(2).all(|pair| pair[0] > pair[1]);

    let safe_level = report.windows(2).all(|pair| {
        let diff = (pair[0] - pair[1]).abs();
        if diff > 0 && diff < 4 {
            true
        } else {
            false
        }
    });

    if (increasing && safe_level) || (decreasing && safe_level) {
        return true;
    }

    return false;
}
