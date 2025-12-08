// bin/day08.rs

use std::collections::HashMap;

use aoc2025::*;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct JBox {
    x: i64,
    y: i64,
    z: i64,
}
impl JBox {
    fn distance_sq(&self, b: &JBox) -> i64 {
        (self.x - b.x).pow(2) + (self.y - b.y).pow(2) + (self.z - b.z).pow(2)
    }
}

#[derive(Clone, Debug)]
struct JBoxPair<'a> {
    jb1: &'a JBox,
    jb2: &'a JBox,
}

#[derive(Clone, Debug, Default)]
struct Circuit<'a> {
    idx: usize,
    elems: Vec<&'a JBox>,
}

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.start_pgm(env!("CARGO_BIN_NAME"))?;

    let mut jbox_vec = Vec::new();
    for (n, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let coords = line.split(',').collect::<Vec<_>>();
        if coords.len() != 3 {
            bail!("Invalid line #{n}: {line:?}");
        }
        let jbox = JBox {
            x: coords[0].parse()?,
            y: coords[1].parse()?,
            z: coords[2].parse()?,
        };
        jbox_vec.push(jbox.clone());
    }

    let mut pair_vec = Vec::new();
    let n_jbox = jbox_vec.len();
    for i in 0..n_jbox {
        for j in i + 1..n_jbox {
            pair_vec.push(JBoxPair {
                jb1: &jbox_vec[i],
                jb2: &jbox_vec[j],
            });
        }
    }
    debug!("We have {} unique jbox pairs", pair_vec.len(),);

    // sort pairs by distance
    pair_vec.sort_by_key(|jb| jb.jb1.distance_sq(jb.jb2));
    debug!("JBox pairs sorted, first 10 elems:\n{:?}", &pair_vec[0..10]);

    // connect circuits, starting with the shortest connections
    let mut circuits: HashMap<usize, Circuit> = HashMap::new();
    let mut circuits_rev: HashMap<&JBox, usize> = HashMap::new();

    for (idx, jbox) in jbox_vec.iter().enumerate() {
        circuits.insert(
            idx,
            Circuit {
                idx,
                elems: vec![jbox],
            },
        );
        circuits_rev.insert(jbox, idx);
    }

    let part1_conns = if jbox_vec.len() > 100 { 1000 } else { 10 };
    for (i, pair) in pair_vec.iter().enumerate() {
        if i == part1_conns {
            let mut circuits_vec = circuits.values().collect::<Vec<_>>();
            circuits_vec.sort_by(|a, b| b.elems.len().cmp(&a.elems.len()));
            let part1 = circuits_vec
                .iter()
                .take(3)
                .map(|circ| circ.elems.len() as u64)
                .product::<u64>();
            println!("Part 1: {part1}");
        }

        // where are the jboxes connected to?
        let id1 = circuits_rev.get(&pair.jb1).unwrap();
        let id2 = circuits_rev.get(&pair.jb2).unwrap();
        if id1 == id2 {
            continue;
        }

        // connect circuits
        let circ2 = circuits.remove(id2).unwrap();
        let circ1 = circuits.get_mut(id1).unwrap();
        for elem in circ2.elems {
            circ1.elems.push(elem);
            circuits_rev.insert(elem, circ1.idx);
        }

        if circuits.len() == 1 {
            println!("Part 2: {}", pair.jb1.x * pair.jb2.x);
        }
    }

    Ok(())
}
// EOF
