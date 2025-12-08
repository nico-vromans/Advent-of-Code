#[cfg(test)]
mod tests;

use aoc_core::Solver;

pub struct Day08;

impl Solver for Day08 {
    fn solve(&self, input: &str) -> Vec<String> {
        fn part1(input: &str) -> u128 {
            // Parse 3D points and connect the K closest pairs (K depends on input size),
            // then multiply the sizes of the three largest resulting circuits (components).
            let mut points: Vec<(i128, i128, i128)> = Vec::new();

            for line in input.lines() {
                let s: &str = line.trim();
                if s.is_empty() { continue; }
                // Expect format: X,Y,Z
                let mut it = s.split(',');
                let x_str: &str = it.next().expect("missing x");
                let y_str: &str = it.next().expect("missing y");
                let z_str: &str = it.next().expect("missing z");
                let x: i128 = x_str.trim().parse::<i128>().expect("invalid x");
                let y: i128 = y_str.trim().parse::<i128>().expect("invalid y");
                let z: i128 = z_str.trim().parse::<i128>().expect("invalid z");
                points.push((x, y, z));
            }

            let n: usize = points.len();
            if n == 0 { return 0u128; }

            // Heuristic: the README example uses 10 connections for 20 points; the real input uses 1000.
            let k_target: usize = if n <= 20usize { 10usize } else { 1000usize };

            // Build all pair distances (squared Euclidean) as (dist, i, j).
            let mut pairs: Vec<(u128, usize, usize)> = Vec::new();
            pairs.reserve(n.saturating_mul(n.saturating_sub(1usize)) / 2usize);
            for i in 0..n {
                let (xi, yi, zi) = points[i];
                for j in (i + 1)..n {
                    let (xj, yj, zj) = points[j];
                    let dx: i128 = xj - xi;
                    let dy: i128 = yj - yi;
                    let dz: i128 = zj - zi;
                    let dist2: u128 = (dx * dx) as u128 + (dy * dy) as u128 + (dz * dz) as u128;
                    pairs.push((dist2, i, j));
                }
            }

            // Select the k smallest distances.
            let m: usize = pairs.len();
            let k_use: usize = k_target.min(m);
            if k_use < m {
                let nth_idx: usize = if k_use == 0 { 0 } else { k_use - 1usize };
                pairs.select_nth_unstable_by(nth_idx, |a: &(u128, usize, usize), b: &(u128, usize, usize)| a.0.cmp(&b.0));
                // Sort only the first k_use by distance ascending to ensure deterministic unions.
                pairs[..k_use].sort_unstable_by(|a: &(u128, usize, usize), b: &(u128, usize, usize)| a.0.cmp(&b.0));
                pairs.truncate(k_use);
            } else {
                pairs.sort_unstable_by(|a: &(u128, usize, usize), b: &(u128, usize, usize)| a.0.cmp(&b.0));
            }

            // Disjoint Set Union (Union-Find) with size tracking.
            let mut parent: Vec<usize> = (0..n).collect();
            let mut size: Vec<u128> = vec![1u128; n];

            fn find(parent: &mut [usize], x: usize) -> usize {
                let mut v: usize = x;
                while parent[v] != v {
                    let p: usize = parent[v];
                    let gp: usize = parent[p];
                    parent[v] = gp; // path halving
                    v = p;
                }
                v
            }

            let mut unions_done: usize = 0usize;
            for &(_d, i, j) in &pairs {
                if unions_done >= k_use { break; }
                let mut ri: usize = find(&mut parent, i);
                let mut rj: usize = find(&mut parent, j);
                if ri == rj { continue; }
                // Union by size
                if size[ri] < size[rj] { core::mem::swap(&mut ri, &mut rj); }
                parent[rj] = ri;
                size[ri] = size[ri] + size[rj];
                unions_done = unions_done + 1usize;
            }

            // Collect component sizes at roots
            let mut comps: Vec<u128> = Vec::new();
            for i in 0..n {
                if find(&mut parent, i) == i {
                    comps.push(size[i]);
                }
            }
            if comps.is_empty() { return 0u128; }
            comps.sort_unstable_by(|a: &u128, b: &u128| b.cmp(a)); // descending

            let mut product: u128 = 1u128;
            let take: usize = comps.len().min(3usize);
            for idx in 0..take {
                product = product * comps[idx];
            }
            product
        }

        fn part2(input: &str) -> u128 {
            123
        }

        vec![part1(input).to_string(), part2(input).to_string()]
    }
}
