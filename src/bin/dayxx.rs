// bin/dayXX.rs

use aoc2025::*;

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.start_pgm(env!("CARGO_BIN_NAME"))?;

    #[allow(unused_variables)]
    for (n, line) in io::stdin().lock().lines().enumerate() {}

    Ok(())
}
// EOF
