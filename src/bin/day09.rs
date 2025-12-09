// bin/day09.rs

use aoc2025::*;

fn area(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    (1 + (p1.0 - p2.0).abs()) * (1 + (p1.1 - p2.1).abs())
}

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.start_pgm(env!("CARGO_BIN_NAME"))?;

    let mut tiles_vec = Vec::new();
    for (n, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let coords = line.split(',').collect::<Vec<_>>();
        if coords.len() != 2 {
            bail!("Invalid line #{n}: {line:?}");
        }
        tiles_vec.push((coords[0].parse::<i64>()?, coords[1].parse::<i64>()?));
    }
    let n_tiles = tiles_vec.len();
    debug!("Read tiles positions ({n_tiles}): {tiles_vec:?}");

    let mut pair_vec = Vec::with_capacity(n_tiles * n_tiles); // a bit extra
    for i in 0..n_tiles {
        for j in i + 1..n_tiles {
            pair_vec.push((&tiles_vec[i], &tiles_vec[j]));
        }
    }
    debug!("Combinations ({}):\n{:?}", pair_vec.len(), pair_vec);

    // sort by area
    pair_vec.sort_by(|p1, p2| area(*p2.0, *p2.1).cmp(&area(*p1.0, *p1.1)));
    let winner = pair_vec[0];
    println!("Part 1: {}", area(*winner.0, *winner.1));
    Ok(())
}
// EOF
