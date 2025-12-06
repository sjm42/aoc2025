// bin/day06.rs

use aoc2025::*;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.start_pgm(env!("CARGO_BIN_NAME"))?;

    let mut nums: Vec<Vec<i64>> = Vec::new();
    let mut ops = Vec::new();
    let mut line_len = None;
    let mut n_items = None;
    let mut sheet = Vec::new();
    for (n, line) in io::stdin().lock().lines().enumerate() {
        let mut line = line?;
        let items = line.split_whitespace().collect::<Vec<&str>>();
        match n_items {
            Some(n_items) => {
                if items.len() != n_items {
                    bail!(
                        "Invalid number of items ({}) on line #{n}",
                        items.len()
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

        match line_len {
            Some(line_len) => {
                if line.len() != line_len {
                    bail!("Invalid line length ({}) on line #{n}", line.len());
                }
            }
            None => line_len = Some(line.len()),
        }

        line.push(' ');
        sheet.push(line.into_bytes());
    }
    let sheet_ops = sheet.pop().unwrap_or_default();

    debug!("nums: {nums:?}");
    debug!("ops: {ops:?}");

    let mut total1 = 0;
    for (col, op) in ops.iter().enumerate() {
        total1 += match op.as_str() {
            "+" => (0..nums.len()).map(|i| nums[i][col]).sum(),
            "*" => (0..nums.len()).map(|i| nums[i][col]).product(),
            _ => 0,
        };
    }
    println!("Part1 total: {total1}");

    debug!("Sheet: {sheet:?}");
    debug!("Sheet ops: {sheet_ops:?}");

    let (mut total2, mut op, mut values) = (0, b' ', Vec::new());
    for col in 0..sheet_ops.len() {
        if let b'+' | b'*' = sheet_ops[col] {
            op = sheet_ops[col];
        }
        let value_b = sheet
            .iter()
            .filter_map(|row| match row[col] {
                b'0'..=b'9' => Some(row[col]),
                _ => None,
            })
            .collect::<Vec<u8>>();
        if value_b.is_empty() {
            // we are at an empty separator column, now process what we have collected
            total2 += match op {
                b'+' => values.iter().sum(),
                b'*' => values.iter().product(),
                _ => 0,
            };
            values.clear();
        } else {
            values.push(String::from_utf8_lossy(&value_b).parse::<i64>()?);
        }
    }
    println!("Part2 total: {total2}");

    Ok(())
}
// EOF
