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
    let mut rows1 = rows.clone();

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
                    debug!("Accessible: ({x}, {y})");
                    rows1[y as usize][x as usize] = 'X';
                    accessible_count += 1;
                }
            })
    });

    info!("Original:");
    print_map(&rows);

    info!("Accessible marked:");
    print_map(&rows1);
    println!("Part 1: {}", accessible_count);

    // part 2

    let mut removed_count = 0;
    let mut rows2 = rows.clone();
    let mut rounds = 0;
    loop {
        rounds += 1;
        let mut to_remove: Vec<(i32, i32)> = Vec::new();
        (0..size_y).for_each(|y| {
            (0..size_x)
                .filter(|&x| rows2[y as usize][x as usize] == '@')
                .for_each(|x| {
                    let n_neighbors = ADJACENT
                        .iter()
                        .filter(|[dx, dy]| {
                            let (check_x, check_y) = (x + dx, y + dy);
                            (0..size_x).contains(&check_x)
                                && (0..size_y).contains(&check_y)
                                && rows2[check_y as usize][check_x as usize]
                                    == '@'
                        })
                        .count();
                    if n_neighbors < NEIGHBOR_LIMIT {
                        debug!("Accessible: ({x}, {y})");
                        to_remove.push((x, y));
                    }
                })
        });

        to_remove
            .iter()
            .for_each(|(x, y)| rows2[*y as usize][*x as usize] = 'X');
        let removed = to_remove.len();
        if removed == 0 {
            break;
        }
        removed_count += removed;
    }

    info!("Removed marked:");
    print_map(&rows2);
    println!("Part 2: {removed_count} took {rounds} rounds");

    Ok(())
}

fn print_map(map: &[Vec<char>]) {
    map.iter().for_each(|row| {
        info!(
            "{}",
            row.iter()
                .map(|&c| [c, ' '].iter().collect::<String>())
                .collect::<String>()
        );
    });
}
// EOF
