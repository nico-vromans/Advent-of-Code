#[cfg(test)]
mod tests;

use aoc_core::Solver;

pub struct Day09;

impl Solver for Day09 {
    fn solve(&self, input: &str) -> Vec<String> {
        fn part1(input: &str) -> u128 {
            // Parse coordinates and find the largest axis-aligned rectangle area
            // using any two red tiles as opposite corners. Area is inclusive:
            // (|x1 - x2| + 1) * (|y1 - y2| + 1).
            let mut pts: Vec<(i128, i128)> = Vec::new();

            for line in input.lines() {
                let s: &str = line.trim();
                if s.is_empty() { continue; }
                if let Some((xs, ys)) = s.split_once(',') {
                    let x: i128 = xs.trim().parse::<i128>().expect("invalid x");
                    let y: i128 = ys.trim().parse::<i128>().expect("invalid y");
                    pts.push((x, y));
                }
            }

            let n: usize = pts.len();
            if n < 2usize { return 0u128; }

            let mut best: u128 = 0u128;
            for i in 0usize..n {
                let (x1, y1) = pts[i];
                for j in (i + 1usize)..n {
                    let (x2, y2) = pts[j];
                    let dx: i128 = if x1 >= x2 { x1 - x2 } else { x2 - x1 };
                    let dy: i128 = if y1 >= y2 { y1 - y2 } else { y2 - y1 };
                    let width: u128 = (dx as u128) + 1u128;
                    let height: u128 = (dy as u128) + 1u128;
                    let area: u128 = width * height;
                    if area > best { best = area; }
                }
            }

            best
        }

        fn part2(input: &str) -> u128 {
            // Part 2: Only rectangles fully covered by red or green tiles are allowed.
            // Green tiles are the axis-aligned paths between consecutive red tiles (wrapping)
            // plus the entire interior of the resulting simple orthogonal polygon.
            // Approach:
            // - Parse points, build vertical edges as toggle events along Y for each X.
            // - Coordinate-compress X and Y using the unique vertex coordinates.
            // - Sweep X from left to right, toggling inside Y-slabs. For each X-slab,
            //   store a prefix sum of inside booleans over Y-slabs.
            // - For each pair of red points, derive rectangle in compressed indices and
            //   verify that for every X-slab intersecting the rectangle, all Y-slabs within
            //   the rectangle are inside. Degenerate rectangles (lines) accept inclusion
            //   if either adjacent slab (left/right or below/above) is inside.

            let mut pts: Vec<(i128, i128)> = Vec::new();
            for line in input.lines() {
                let s: &str = line.trim();
                if s.is_empty() { continue; }
                if let Some((xs, ys)) = s.split_once(',') {
                    let x: i128 = xs.trim().parse::<i128>().expect("invalid x");
                    let y: i128 = ys.trim().parse::<i128>().expect("invalid y");
                    pts.push((x, y));
                }
            }

            let n: usize = pts.len();
            if n < 2usize { return 0u128; }

            // Unique sorted X and Y from vertices
            let mut xs_vals: Vec<i128> = pts.iter().map(|&(x, _)| x).collect();
            xs_vals.sort_unstable();
            xs_vals.dedup();
            let mut ys_vals: Vec<i128> = pts.iter().map(|&(_, y)| y).collect();
            ys_vals.sort_unstable();
            ys_vals.dedup();

            let xs_len: usize = xs_vals.len();
            let ys_len: usize = ys_vals.len();
            if xs_len < 2usize || ys_len < 1usize { return 0u128; }

            // Map functions
            fn lower_bound(v: &[i128], key: i128) -> usize {
                let mut lo: usize = 0; let mut hi: usize = v.len();
                while lo < hi { let mid: usize = lo + (hi - lo) / 2; if v[mid] < key { lo = mid + 1; } else { hi = mid; } }
                lo
            }
            fn map_to_slab(coords: &[i128], val: i128) -> usize {
                // Return k such that coords[k] <= val < coords[k+1], clamped to [0, len-2].
                let len: usize = coords.len();
                if len < 2 { return 0usize; }
                let idx_upper: usize = {
                    // upper_bound
                    let mut lo: usize = 0; let mut hi: usize = len;
                    while lo < hi { let mid: usize = lo + (hi - lo) / 2; if coords[mid] <= val { lo = mid + 1; } else { hi = mid; } }
                    lo
                };
                let mut k: isize = (idx_upper as isize) - 1isize;
                if k < 0 { k = 0; }
                if k as usize >= len - 1 { k = (len as isize) - 2isize; }
                k as usize
            }

            // Build events for vertical segments: at X index i, toggle Y-slabs in [yl..yh)
            let mut events: Vec<Vec<(usize, usize)>> = vec![Vec::new(); xs_len];
            // Also collect exact vertical and horizontal edge intervals for boundary checks
            use std::collections::BTreeMap;
            let mut vmap: BTreeMap<i128, Vec<(i128, i128)>> = BTreeMap::new(); // x -> list of [y0,y1] (inclusive)
            let mut hmap: BTreeMap<i128, Vec<(i128, i128)>> = BTreeMap::new(); // y -> list of [x0,x1] (inclusive)
            for i in 0usize..n {
                let (x1, y1) = pts[i];
                let (x2, y2) = pts[(i + 1usize) % n];
                if x1 == x2 {
                    let xi: usize = lower_bound(&xs_vals, x1);
                    // y interval
                    let ya: i128 = if y1 <= y2 { y1 } else { y2 };
                    let yb: i128 = if y1 <= y2 { y2 } else { y1 };
                    let yl: usize = lower_bound(&ys_vals, ya);
                    let yh: usize = lower_bound(&ys_vals, yb);
                    if yl < yh {
                        events[xi].push((yl, yh));
                    }
                    vmap.entry(x1).or_default().push((ya, yb));
                } else if y1 == y2 {
                    let xa: i128 = if x1 <= x2 { x1 } else { x2 };
                    let xb: i128 = if x1 <= x2 { x2 } else { x1 };
                    hmap.entry(y1).or_default().push((xa, xb));
                }
            }

            // Merge intervals in vmap and hmap for fast coverage checks
            fn merge_intervals(mut segs: Vec<(i128, i128)>) -> Vec<(i128, i128)> {
                if segs.is_empty() { return segs; }
                segs.sort_by(|a, b| if a.0 == b.0 { a.1.cmp(&b.1) } else { a.0.cmp(&b.0) });
                let mut merged: Vec<(i128, i128)> = Vec::with_capacity(segs.len());
                let (mut cs, mut ce) = segs[0];
                for (s, e) in segs.into_iter().skip(1) {
                    if s <= ce + 0 { // touching considered continuous (inclusive coords)
                        if e > ce { ce = e; }
                    } else {
                        merged.push((cs, ce));
                        cs = s; ce = e;
                    }
                }
                merged.push((cs, ce));
                merged
            }
            let mut vmap_m: BTreeMap<i128, Vec<(i128, i128)>> = BTreeMap::new();
            for (xk, segs) in vmap { vmap_m.insert(xk, merge_intervals(segs)); }
            let mut hmap_m: BTreeMap<i128, Vec<(i128, i128)>> = BTreeMap::new();
            for (yk, segs) in hmap { hmap_m.insert(yk, merge_intervals(segs)); }

            let y_slabs: usize = if ys_len >= 2usize { ys_len - 1usize } else { 0usize };
            if y_slabs == 0usize { return 0u128; }
            let x_slabs: usize = xs_len - 1usize;

            // Sweep across X slabs and build prefix sums of inside for each slab.
            let mut inside: Vec<bool> = vec![false; y_slabs];
            let mut pref: Vec<Vec<u16>> = vec![vec![0u16; y_slabs + 1usize]; x_slabs];

            for k in 0usize..x_slabs {
                // Apply toggles at xs_vals[k]
                for &(yl, yh) in &events[k] {
                    for yi in yl..yh { inside[yi] = !inside[yi]; }
                }
                // Build prefix true counts for slab k
                let mut acc: u16 = 0u16;
                for yi in 0usize..y_slabs {
                    if inside[yi] { acc = acc.saturating_add(1u16); }
                    pref[k][yi + 1usize] = acc;
                }
            }

            // Helper to check if for a given X-slab k, all y-slabs in [ay,by) are inside
            let all_inside_on_slab = |k: usize, ay: usize, by: usize| -> bool {
                if ay >= by { return true; }
                let total: u16 = pref[k][by] - pref[k][ay];
                total as usize == (by - ay)
            };

            // Check if vertical boundary at x covers [y0,y1] fully
            let check_vertical_line = |x: i128, y0: i128, y1: i128| -> bool {
                let a: i128 = if y0 <= y1 { y0 } else { y1 };
                let b: i128 = if y0 <= y1 { y1 } else { y0 };
                if let Some(list) = vmap_m.get(&x) {
                    for &(s, e) in list {
                        if s <= a && e >= b { return true; }
                    }
                }
                false
            };
            let check_horizontal_line = |y: i128, x0: i128, x1: i128| -> bool {
                let a: i128 = if x0 <= x1 { x0 } else { x1 };
                let b: i128 = if x0 <= x1 { x1 } else { x0 };
                if let Some(list) = hmap_m.get(&y) {
                    for &(s, e) in list {
                        if s <= a && e >= b { return true; }
                    }
                }
                false
            };

            let mut best: u128 = 0u128;
            for i in 0usize..n {
                let (x1, y1) = pts[i];
                for j in (i + 1usize)..n {
                    let (x2, y2) = pts[j];
                    let xmin: i128 = if x1 <= x2 { x1 } else { x2 };
                    let xmax: i128 = if x1 <= x2 { x2 } else { x1 };
                    let ymin: i128 = if y1 <= y2 { y1 } else { y2 };
                    let ymax: i128 = if y1 <= y2 { y2 } else { y1 };

                    // Map to slab ranges
                    if xmin == xmax {
                        // Vertical line rectangle: accept if it lies on a vertical boundary segment
                        // or if the adjacent interior (left or right slab) is fully inside on [ymin,ymax).
                        if check_vertical_line(xmin, ymin, ymax) {
                            // ok
                        } else {
                            let kx: usize = map_to_slab(&xs_vals, xmin);
                            let ay: usize = lower_bound(&ys_vals, if ymin <= ymax { ymin } else { ymax });
                            let by: usize = lower_bound(&ys_vals, if ymin <= ymax { ymax } else { ymin });
                            let mut ok_adj: bool = false;
                            if kx < x_slabs && all_inside_on_slab(kx, ay, by) { ok_adj = true; }
                            if !ok_adj && kx > 0usize && all_inside_on_slab(kx - 1usize, ay, by) { ok_adj = true; }
                            if !ok_adj { continue; }
                        }
                    } else {
                        let k_start: usize = map_to_slab(&xs_vals, xmin);
                        let k_end: usize = map_to_slab(&xs_vals, xmax);
                        let ay: usize = lower_bound(&ys_vals, if ymin <= ymax { ymin } else { ymax });
                        let by: usize = lower_bound(&ys_vals, if ymin <= ymax { ymax } else { ymin });

                        let mut ok: bool = true;
                        // Check all x-slabs intersecting rectangle interior: [k_start, k_end)
                        for k in k_start..k_end {
                            if !all_inside_on_slab(k, ay, by) { ok = false; break; }
                        }
                        if !ok {
                            // Horizontal line rectangle (by==ay): allow if it lies on horizontal boundary
                            if ymin == ymax {
                                if !check_horizontal_line(ymin, xmin, xmax) { continue; }
                            } else {
                                continue;
                            }
                        }
                    }

                    // Compute inclusive area
                    let dx: i128 = if x1 >= x2 { x1 - x2 } else { x2 - x1 };
                    let dy: i128 = if y1 >= y2 { y1 - y2 } else { y2 - y1 };
                    let width: u128 = (dx as u128) + 1u128;
                    let height: u128 = (dy as u128) + 1u128;
                    let area: u128 = width * height;
                    if area > best { best = area; }
                }
            }

            best
        }

        vec![part1(input).to_string(), part2(input).to_string()]
    }
}
