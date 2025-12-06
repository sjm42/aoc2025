// bin/day06.rs

use aoc2025::*;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.start_pgm(env!("CARGO_BIN_NAME"))?;

    let mut nums: Vec<Vec<i64>> = Vec::new();
    let mut ops = Vec::new();
    let mut n_items = None;
    for (n, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let items = line.split_whitespace().collect::<Vec<&str>>();
        match n_items {
            Some(n_items) => {
                if items.len() != n_items {
                    bail!(
                        "Invalid number of items ({}) on line #{}",
                        items.len(),
                        n
                    );
                }
            }
            None => n_items = Some(items.len()),
        }
        match items[0] {
            "+" | "*" => {
                ops = items.iter().map(|&item| item.to_string()).collect();
            }
            _ => {
                nums.push(
                    items
                        .iter()
                        .map(|&item| item.parse::<i64>().unwrap_or_default())
                        .collect(),
                );
            }
        }
    }
    debug!("nums: {nums:?}");
    debug!("ops: {ops:?}");

    let mut total = 0;
    for (col, op) in ops.iter().enumerate() {
        total += match op.as_str() {
            "+" => (0..nums.len()).map(|i| nums[i][col]).sum(),
            "*" => (0..nums.len()).map(|i| nums[i][col]).product(),
            _ => 0,
        };
    }
    println!("Parti 1 total: {total}");

    Ok(())
}
// EOF
