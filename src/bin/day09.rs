// bin/day09.rs

use aoc2025::*;

fn area(p1: (i64, i64), p2: (i64, i64)) -> i64 {
    (1 + (p1.0 - p2.0).abs()) * (1 + (p1.1 - p2.1).abs())
}

fn p_min(p1: (i64, i64), p2: (i64, i64)) -> (i64, i64) {
    (p1.0.min(p2.0), p1.1.min(p2.1))
}
fn p_max(p1: (i64, i64), p2: (i64, i64)) -> (i64, i64) {
    (p1.0.max(p2.0), p1.1.max(p2.1))
}

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.start_pgm(env!("CARGO_BIN_NAME"))?;

    let mut pos_vec = Vec::new();
    for (n, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let coords = line.split(',').collect::<Vec<_>>();
        if coords.len() != 2 {
            bail!("Invalid line #{n}: {line:?}");
        }
        pos_vec.push((coords[0].parse::<i64>()?, coords[1].parse::<i64>()?));
    }
    let n_pos = pos_vec.len();
    info!("Read tiles positions: {n_pos}");
    debug!("{pos_vec:?}");

    let mut pairs = Vec::with_capacity(n_pos * n_pos);
    let mut areas = Vec::with_capacity(n_pos * n_pos);
    for i in 0..n_pos {
        for j in i + 1..n_pos {
            pairs.push((pos_vec[i], pos_vec[j]));
            areas.push((
                p_min(pos_vec[i], pos_vec[j]),
                p_max(pos_vec[i], pos_vec[j]),
            ));
        }
    }
    info!("Combinations: {}", pairs.len());
    debug!("{:?}", pairs);

    // sort by area
    pairs.sort_by(|p1, p2| area(p2.0, p2.1).cmp(&area(p1.0, p1.1)));
    let winner = pairs[0];
    println!("Part 1: {}", area(winner.0, winner.1));

    areas.sort_by(|p1, p2| area(p2.0, p2.1).cmp(&area(p1.0, p1.1)));
    let mut edges = Vec::with_capacity(n_pos);
    pos_vec.iter().skip(1).for_each(|p1| {
        edges.push(((pos_vec[0].0, pos_vec[0].1), (p1.0, p1.1)))
    });
    edges.push((
        (pos_vec[n_pos - 1].0, pos_vec[n_pos - 1].1),
        (pos_vec[0].0, pos_vec[0].1),
    ));
    Ok(())
}
// EOF
