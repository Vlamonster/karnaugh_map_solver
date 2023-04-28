use clap::Parser;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::Write;

fn reduce(
    map: &[Option<usize>], // Karnaugh map
    idx: usize,            // Current index
    start: usize,          // Start index for the next iteration
    end: usize,            // End index for the next iteration
    dims: &mut Vec<usize>, // Vector of indices for current iteration
    min: &mut Vec<usize>,  // Vector of indices for min term
) {
    for i in start..end {
        // Check if all subsets of `dims` are covered by the Karnaugh map
        if dims
            .iter()
            .powerset()
            .all(|j| map[idx ^ (1 << i) ^ j.iter().fold(0, |acc, &&k| acc | (1 << k))] != Some(0))
        {
            dims.push(i);
            reduce(map, idx, i + 1, end, dims, min);
            if dims.len() > min.len() {
                *min = dims.clone();
            }
            dims.pop();
        }
    }
}

fn karnaugh(map: &[Option<usize>], n: usize) -> HashSet<String> {
    let mut min_terms = HashSet::new();
    let mut dims = Vec::with_capacity(n);
    for (i, &val) in map.iter().enumerate() {
        if val == Some(1) {
            let mut min = Vec::with_capacity(n);
            reduce(map, i, 0, n, &mut dims, &mut min);

            min_terms.insert(
                format!("{i:0n$b}")
                    .chars()
                    .zip((0..n).rev())
                    .map(|(c, j)| if min.contains(&j) { 'x' } else { c })
                    .collect::<String>(),
            );
        }
    }
    min_terms
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file to use.
    #[arg(short, long)]
    input: String,

    /// Number of variables.
    #[arg(short, long)]
    nums: usize,

    /// Generate an input file.
    #[arg(short, long)]
    generate: bool,
}

fn main() {
    let args = Args::parse();

    if args.generate {
        let mut file = File::create(args.input).unwrap();
        for i in 0..1 << args.nums {
            let _ = file.write(format!("{i:0n$b}: \n", n = args.nums).as_bytes());
        }
    } else {
        let input = fs::read_to_string(args.input).unwrap();
        let map = input
            .lines()
            .map(|line| match line.split_whitespace().nth(1).unwrap_or("x") {
                "x" => None,
                "1" => Some(1),
                "0" => Some(0),
                _ => panic!("Incorrectly formatted input, line: {line}."),
            })
            .collect::<Vec<_>>();
        println!("normal: {:?}", karnaugh(&map, args.nums));

        // shitty code, but it works.
        let map = map
            .iter()
            .map(|x| match x {
                None => None,
                Some(0) => Some(1),
                Some(1) => Some(0),
                _ => panic!(),
            })
            .collect_vec();
        println!("inverted: {:?}", karnaugh(&map, args.nums));
    }
}
