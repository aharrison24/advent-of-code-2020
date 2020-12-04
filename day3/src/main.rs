use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Rem;
use structopt::StructOpt;

#[derive(PartialEq, Debug)]
struct State {
    count: i32,
    xpos: i32,
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn reducer(state: State, line: &str) -> State {
    let next_xpos = (state.xpos + 3).rem(line.len() as i32);
    match line.chars().nth(state.xpos as usize).unwrap() {
        '#' => State {
            count: state.count + 1,
            xpos: next_xpos,
        },
        _ => State {
            xpos: next_xpos,
            ..state
        },
    }
}

fn main() {
    let args = Cli::from_args();

    let file = File::open(&args.path).expect("Cannot open file");

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect_vec();

    let result = lines.iter().fold(State{count: 0, xpos: 0}, |acc, x| reducer(acc, x));
    println!("Number of trees hit: {}", result.count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dots_do_not_increase_the_count() {
        assert_eq!(
            reducer(State { count: 0, xpos: 0 }, "...."),
            State { count: 0, xpos: 3 }
        );
    }

    #[test]
    fn hash_does_increase_the_count() {
        assert_eq!(
            reducer(State { count: 0, xpos: 0 }, "#..."),
            State { count: 1, xpos: 3 }
        );
    }

    #[test]
    fn xpos_wraps_at_line_end() {
        assert_eq!(
            reducer(State { count: 0, xpos: 1 }, "#..."),
            State { count: 0, xpos: 0 }
        );
    }

}
