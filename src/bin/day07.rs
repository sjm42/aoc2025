// bin/day07.rs

use aoc2025::*;

const VALID_INPUT_CHARS: &str = ".^S";

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.start_pgm(env!("CARGO_BIN_NAME"))?;

    let mut rows = io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (size_x, _size_y) = (rows[0].len() as i32, rows.len() as i32);

    debug!("Rows read from input:");
    debug_print_rows(&rows);

    debug!("Sanity check for input...");
    for (row_n, row) in rows.iter().enumerate() {
        if row.len() as i32 != size_x
            || row.iter().any(|&c| !VALID_INPUT_CHARS.contains(c))
        {
            bail!("Illegal row #{row_n}: {row:?}");
        }
    }
    debug!("Sanity check passed.");

    let mut n_splits = 0;
    let mut beams_pos: Vec<i32> = Vec::new();
    for (row_n, row) in rows.iter_mut().enumerate() {
        // convert 'S' to '|'
        row.iter_mut().filter(|c| **c == 'S').for_each(|c| *c = '|');
        let row_sz = row.len() as i32;

        // process beam positions from previous row
        let mut new_splits = 0;
        for &pos in beams_pos.iter() {
            if row[pos as usize] == '.' {
                row[pos as usize] = '|'; // no splitter, continue beam
            } else if row[pos as usize] == '^' {
                // we have a splitter
                new_splits += 1;
                for d in [-1, 1] {
                    let new_pos = pos + d;
                    let new_pos = if new_pos < 0 || new_pos >= row_sz {
                        continue; // out of bounds
                    } else {
                        new_pos as usize // potential split
                    };
                    if row[new_pos] == '.' {
                        row[new_pos] = '|';
                    }
                }
            }
        }
        debug!("Row #{row_n} New splits: {new_splits}");
        n_splits += new_splits;

        // store beam positions from this line for next round
        beams_pos = row
            .iter()
            .enumerate()
            .filter_map(
                |(pos, c)| if *c == '|' { Some(pos as i32) } else { None },
            )
            .collect();
    }
    debug!("Rows after processing:");
    debug_print_rows(&rows);
    println!("part 1: splits {n_splits}");

    Ok(())
}

fn debug_print_rows(rows: &[Vec<char>]) {
    rows.iter()
        .for_each(|row| debug!("{}", row.iter().collect::<String>()));
}
// EOF
