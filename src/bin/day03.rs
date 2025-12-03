// bin/day03.rs

use aoc2025::*;

const PART2_LEN: usize = 12;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.start_pgm(env!("CARGO_BIN_NAME"))?;

    let batts = io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|n| n as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    // part 1

    let mut total1 = 0;
    for batt in batts.iter() {
        // Note: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by
        // "If several elements are equally maximum, the last element is returned."
        // we use rev() because we want the first
        let (idx1, &num1) = batt
            .iter()
            .enumerate()
            .rev()
            .skip(1)
            .max_by(|a, b| a.1.cmp(b.1))
            .unwrap();
        let (mut idx2, &num2) = batt[idx1 + 1..]
            .iter()
            .enumerate()
            .rev()
            .max_by(|a, b| a.1.cmp(b.1))
            .unwrap();
        idx2 += idx1 + 1;
        let len = batt.len();
        let jtg = (num1 * 10 + num2) as u64;
        info!(
            "Batt({len}) {}:\n{} at {},{}",
            batt.iter().map(ToString::to_string).collect::<String>(),
            jtg,
            idx1,
            idx2
        );
        total1 += jtg;
    }
    println!("Part 1 total: {total1}");

    // part 2

    let mut total2 = 0;
    for batt in batts.iter() {
        let batt_len = batt.len();
        let mut start_idx = 0;
        info!(
            "Batt({batt_len}) {}",
            batt.iter().map(ToString::to_string).collect::<String>()
        );

        let mut nums = Vec::new();
        for i in (0..PART2_LEN).rev() {
            let (mut idx, &num) = batt[start_idx..batt_len - i]
                .iter()
                .enumerate()
                .rev()
                // .filter(|n| {
                //    debug!("- check {n:?}");
                //    true
                // })
                .max_by(|a, b| a.1.cmp(b.1))
                .unwrap();
            idx += start_idx;

            debug!("Found: {num}@{idx}");
            nums.push(num);
            start_idx = idx + 1;
        }
        let final_num = nums
            .iter()
            .map(ToString::to_string)
            .collect::<String>()
            .parse::<u64>()?;
        info!("Found nums({}): {:?}", nums.len(), final_num);
        total2 += final_num;
    }
    println!("Part 2 total: {total2}");

    Ok(())
}
// EOF
