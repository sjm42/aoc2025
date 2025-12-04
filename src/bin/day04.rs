// bin/day04.rs

use aoc2025::*;

const ADJACENT: [[i32; 2]; 8] = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [-1, 0],
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
];
const NEIGHBOR_LIMIT: usize = 4;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.start_pgm(env!("CARGO_BIN_NAME"))?;

    let rows = io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (size_x, size_y) = (rows[0].len() as i32, rows.len() as i32);

    // part 1

    let mut accessible_count = 0;
    let mut rows_1 = rows.clone();

    (0..size_y).for_each(|y| {
        (0..size_x)
            .filter(|&x| rows[y as usize][x as usize] == '@')
            .for_each(|x| {
                let n_neighbors = ADJACENT
                    .iter()
                    .filter(|[dx, dy]| {
                        let (check_x, check_y) = (x + dx, y + dy);
                        (0..size_x).contains(&check_x)
                            && (0..size_y).contains(&check_y)
                            && rows[check_y as usize][check_x as usize] == '@'
                    })
                    .count();
                if n_neighbors < NEIGHBOR_LIMIT {
                    info!("Accessible: ({x}, {y})");
                    rows_1[y as usize][x as usize] = 'X';
                    accessible_count += 1;
                }
            })
    });

    info!("Original:");
    rows.iter().for_each(|row| {
        info!("{}", row.iter().collect::<String>());
    });

    info!("Accessible marked:");
    rows_1.iter().for_each(|row| {
        info!("{}", row.iter().collect::<String>());
    });
    println!("Part 1: {}", accessible_count);

    Ok(())
}
// EOF
