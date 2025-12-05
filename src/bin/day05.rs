// bin/day05.rs

use aoc2025::*;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.start_pgm(env!("CARGO_BIN_NAME"))?;

    let mut ranges1 = Vec::new();
    for (n, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        match line.split_once('-') {
            None => {
                error!("Invalid line #{n}: {line:?}");
                bail!("Failed to parse input.");
            }
            Some((a, b)) => {
                ranges1.push((a.parse::<i64>()?, b.parse::<i64>()?));
            }
        }
    }
    info!("Ranges: {ranges1:?}");

    let mut ids = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line?;
        ids.push(line.parse::<i64>()?);
    }
    info!("IDs: {ids:?}");

    let fresh_count = ids.iter().filter(|&&id| in_ranges(id, &ranges1)).count();
    println!("Part 1: {fresh_count}");

    let mut round = 0;
    let mut new_ranges = Vec::new();
    let mut old_ranges = ranges1;
    loop {
        round += 1;
        info!("# Combining ranges, take #{round}");
        info!("- ranges({}): {:?}", old_ranges.len(), old_ranges);
        for &range in old_ranges.iter() {
            match find_overlapping_range(range, &new_ranges) {
                Some(overlapping) => {
                    new_ranges[overlapping] =
                        combine_ranges(range, new_ranges[overlapping])
                }
                None => new_ranges.push(range),
            }
        }
        if new_ranges.len() == old_ranges.len() {
            break;
        }
        old_ranges.clear();
        new_ranges.drain(..).for_each(|e| old_ranges.push(e));
    }
    info!("Combined ranges({}): {:?}", new_ranges.len(), new_ranges);
    println!("Part 2: {}", count_ids(&new_ranges));
    Ok(())
}

fn in_ranges(id: i64, ranges: &[(i64, i64)]) -> bool {
    ranges.iter().filter(|r| (r.0..=r.1).contains(&id)).count() > 0
}

fn find_overlapping_range(
    range: (i64, i64),
    ranges: &[(i64, i64)],
) -> Option<usize> {
    ranges
        .iter()
        .enumerate()
        .filter_map(|(idx, &r1)| {
            if ranges_overlap(r1, range) {
                Some(idx)
            } else {
                None
            }
        })
        .next()
}

fn ranges_overlap(r1: (i64, i64), r2: (i64, i64)) -> bool {
    debug!("overlap? {r1:?}, {r2:?}");
    (r1.0..=r1.1).contains(&r2.0)
        || (r1.0..=r1.1).contains(&r2.1)
        || (r2.0..=r2.1).contains(&r1.0)
        || (r2.0..=r2.1).contains(&r1.1)
}

fn combine_ranges(range1: (i64, i64), range2: (i64, i64)) -> (i64, i64) {
    let ret = (cmp::min(range1.0, range2.0), cmp::max(range1.1, range2.1));
    info!("combined: ({range1:?}, {range2:?}) --> {ret:?}");
    ret
}

fn count_ids(ranges: &[(i64, i64)]) -> i64 {
    ranges.iter().map(|&(min, max)| 1 + max - min).sum()
}
// EOF
