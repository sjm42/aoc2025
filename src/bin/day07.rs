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
    let mut beams = vec![0; size_x as usize];

    rows[0] // initialize beams, find 'S'
        .iter_mut()
        .enumerate()
        .filter(|(_i, c)| **c == 'S')
        .for_each(|(i, c)| {
            *c = '|';
            beams[i] = 1;
        });

    debug!("Rows at start:");
    debug_print_rows(&rows);

    for row in rows.iter_mut() {
        // process beam positions from previous row
        for pos in 0..beams.len() {
            if beams[pos] == 0 {
                continue;
            }

            if row[pos] == '^' {
                n_splits += 1;

                for d in [-1, 1] {
                    let new_pos = pos as i32 + d;
                    let new_pos = if (0..size_x).contains(&new_pos) {
                        new_pos as usize // potential split
                    } else {
                        continue; // out of bounds
                    };

                    beams[new_pos] += beams[pos];
                    row[new_pos] = '|';
                }
                beams[pos] = 0;
            } else {
                row[pos] = '|'; // no splitter, continue beam
            }
        }
    }
    debug!("Rows after processing:");
    debug_print_rows(&rows);

    println!("part 1: splits {n_splits}");
    println!("part 2: beams {}", beams.iter().sum::<u64>());

    Ok(())
}

fn debug_print_rows(rows: &[Vec<char>]) {
    rows.iter()
        .for_each(|row| debug!("{}", row.iter().collect::<String>()));
}
// EOF
