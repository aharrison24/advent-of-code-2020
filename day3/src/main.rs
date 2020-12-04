use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Rem;
use structopt::StructOpt;

#[derive(PartialEq, Debug)]
struct State {
    line_index: i32,
    count: i32,
    sample_x: i32,
    sample_y: i32,
}

impl Default for State {
    fn default() -> State {
        State {
            line_index: 0,
            count: 0,
            sample_x: 0,
            sample_y: 0,
        }
    }
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn reducer(acc: State, line: &str, x_step: i32, y_step: i32) -> State {
    // Skip lines until we hit the next y sample point
    if acc.line_index < acc.sample_y {
        return State {
            line_index: acc.line_index + 1,
            ..acc
        };
    }

    // Compute next sample location
    let result = State {
        sample_x: (acc.sample_x + x_step).rem(line.len() as i32),
        sample_y: acc.sample_y + y_step,
        line_index: acc.line_index + 1,
        ..acc
    };

    // Increment count if there is a tree at the current location
    match line.chars().nth(acc.sample_x as usize).unwrap() {
        '#' => State {
            count: acc.count + 1,
            ..result
        },
        _ => result,
    }
}

fn count_trees_hit(x_step: i32, y_step: i32, lines: &Vec<String>) -> i32 {
    lines
        .iter()
        .fold(Default::default(), |acc, x| reducer(acc, x, x_step, y_step))
        .count
}

fn main() {
    let args = Cli::from_args();

    let file = File::open(&args.path).expect("Cannot open file");

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect_vec();

    println!(
        "Part1: Number of trees hit for slope (3,1) = {}",
        count_trees_hit(3, 1, &lines)
    );

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let tree_counts = slopes
        .iter()
        .map(|&(x_step, y_step)| count_trees_hit(x_step, y_step, &lines));
    println!(
        "Part2: Product of trees hit for multiple slopes = {}",
        tree_counts.product::<i32>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dots_do_not_increase_the_count() {
        assert_eq!(
            reducer(Default::default(), "....", 3, 1),
            State {
                line_index: 1,
                count: 0,
                sample_x: 3,
                sample_y: 1
            }
        );
    }

    #[test]
    fn hash_does_increase_the_count() {
        assert_eq!(
            reducer(Default::default(), "#...", 3, 1),
            State {
                line_index: 1,
                count: 1,
                sample_x: 3,
                sample_y: 1
            }
        );
    }

    #[test]
    fn xpos_wraps_at_line_end() {
        assert_eq!(
            reducer(
                State {
                    sample_x: 1,
                    ..Default::default()
                },
                "#...",
                3,
                1
            ),
            State {
                line_index: 1,
                count: 0,
                sample_x: 0,
                sample_y: 1
            }
        );
    }
}
