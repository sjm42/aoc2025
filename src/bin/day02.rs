// bin/day02.rs

use aoc2025::*;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.start_pgm(env!("CARGO_BIN_NAME"))?;

    let mut ranges = Vec::new();
    for (n, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        for r in line.split(',') {
            let (r1s, r2s) = r.split_once('-').ok_or_else(|| {
                anyhow::anyhow!("On line #{n} invalid range {r}")
            })?;
            let new_range = (r1s.parse::<u64>()?, r2s.parse::<u64>()?);
            if new_range.0 >= new_range.1 {
                bail!("Range is backwards: {r}");
            }
            ranges.push(new_range);
        }
    }
    debug!("Got ranges: {ranges:#?}");
    let mut sum1 = 0;
    let mut sum2 = 0;
    for r in ranges {
        debug!("### Checking range {r:?}");
        sum1 += find_silly1(r).iter().sum::<u64>();
        sum2 += find_silly2(r).iter().sum::<u64>();
    }
    println!("Sum1: {sum1}");
    println!("Sum2: {sum2}");
    Ok(())
}

fn find_silly1(range: (u64, u64)) -> Vec<u64> {
    let mut v = Vec::new();

    let mut num_start = range.0;
    let mut num_end = cmp::min(range.1, len_largest(num_len(num_start)));
    loop {
        if !num_is_even_len(num_start) {
            info!("- not inspecting range {num_start} .. {num_end}");
        } else {
            info!("+ inspecting range {num_start} .. {num_end}");
            v.append(&mut list_silly1((num_start, num_end)));
        }
        if num_end == range.1 {
            break;
        }
        num_start = num_end + 1;
        num_end = cmp::min(range.1, len_largest(num_len(num_start)));
    }
    v
}

fn list_silly1(range: (u64, u64)) -> Vec<u64> {
    let mut v = Vec::new();
    if num_len(range.0) != num_len(range.1) || !num_is_even_len(range.0) {
        error!("list_silly({range:?}) - invalid range");
        return v;
    }
    let factor = 10u64.pow(num_len(range.0) / 2);
    let (silly_min, silly_max) = (range.0 / factor, range.1 / factor);
    debug!("  . finding silly {silly_min} .. {silly_max} (factor {factor})");
    for i in silly_min..=silly_max {
        let silly = i * factor + i;
        if in_range(silly, range) {
            v.push(silly);
        }
    }
    debug!("  . found silliness: {v:?}");
    v
}

fn num_len(n: u64) -> u32 {
    let len = n.checked_ilog10().unwrap_or(0) + 1;
    debug!("num_len({n}) = {len}");
    len
}

fn num_is_even_len(n: u64) -> bool {
    let is_even_len = num_len(n).is_multiple_of(2);
    debug!("is_even_len({n}) = {is_even_len}");
    is_even_len
}

#[allow(dead_code)]
fn len_smallest(len: u32) -> u64 {
    // let smallest = if len <= 1 { 0 } else { 10u64.pow(len - 1) };
    let smallest = 10u64.pow(len - 1);
    debug!("smallest({len}) = {smallest}");
    smallest
}

fn len_largest(len: u32) -> u64 {
    // let largest = if len <= 1 { 9 } else { 10u64.pow(len) - 1 };
    let largest = 10u64.pow(len) - 1;
    debug!("largest({len}) = {largest}");
    largest
}

fn in_range(num: u64, range: (u64, u64)) -> bool {
    num >= range.0 && num <= range.1
}

// part 2, this time just use strings

fn find_silly2(range: (u64, u64)) -> Vec<u64> {
    (range.0..=range.1)
        .filter(|&n| is_silly2(&n.to_string()))
        .collect()
}

fn is_silly2(id: &str) -> bool {
    let len = id.len();
    for i in 1..len {
        if !len.is_multiple_of(i) {
            continue;
        }
        let subid = &id[..i];
        if id == subid.repeat(len / i) {
            return true;
        }
    }
    false
}
// EOF
