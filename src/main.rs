use itertools::Itertools;
use std::collections::HashSet;

fn reduce(
    map: &[usize],         // Karnaugh map
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
            .all(|j| map[idx ^ (1 << i) ^ j.iter().fold(0, |acc, &&k| acc | (1 << k))] != 0)
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

fn karnaugh(map: &[usize]) -> HashSet<String> {
    let n = map.len().ilog2() as usize;
    let mut min_terms = HashSet::new();
    let mut dims = Vec::with_capacity(n);
    for (i, &val) in map.iter().enumerate() {
        if val != 0 {
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

fn main() {
    // 3 variable map (https://tinyurl.com/mt33tp33):
    let map = [1, 1, 1, 1, 0, 1, 1, 0];

    println!("min-terms for 3-variable map: {:?}", karnaugh(&map));

    // 6 variable map (https://tinyurl.com/mphrf63h):
    let map = [
        1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1,
        1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    println!("min-terms for 6-variable map: {:?}", karnaugh(&map));
}
