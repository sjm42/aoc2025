// bin/day01.rs

use aoc2025::*;

const N_POS: i64 = 100;

struct Dial {
    pub pos: i64,
    pub zeros: i64,
    pub zero_xings: i64,
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            pos: 50,
            zeros: 0,
            zero_xings: 0,
        }
    }
}
impl Dial {
    fn rotate(&mut self, input: &str) -> anyhow::Result<()> {
        if input.len() < 2 {
            bail!("invalid input: {}", input);
        }
        let dir = match input.chars().nth(0).unwrap() {
            'L' => -1,
            'R' => 1,
            _ => bail!("invalid input: {}", input),
        };

        let steps = input.chars().skip(1).collect::<String>().parse::<i64>()?;
        let delta = dir * steps;
        let tmp_pos = self.pos + delta;
        let mut xings = if dir > 0 {
            (self.pos + steps - 1) / N_POS
        } else if tmp_pos >= 0 {
            0
        } else if self.pos == 0 || tmp_pos % N_POS == 0 {
            tmp_pos.abs() / N_POS
        } else {
            1 + (tmp_pos.abs() / N_POS)
        };

        let mut new_pos = tmp_pos % N_POS;
        if new_pos < 0 {
            new_pos += 100;
        }

        let zero = if new_pos == 0 {
            xings += 1;
            1
        } else {
            0
        };
        debug!(
            "Old pos {} move {} ({}) new pos {} ({}) - zero {} crossings {}",
            self.pos, input, delta, new_pos, tmp_pos, zero, xings
        );

        self.zero_xings += xings;
        self.zeros += zero;
        self.pos = new_pos;
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let mut opts = OptsCommon::parse();
    opts.start_pgm(env!("CARGO_BIN_NAME"))?;

    let mut dial = Dial::default();
    for (n, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        dial.rotate(&line)?;
        info!("Dial pos@{n}: {}", dial.pos);
    }
    println!("zeros: {}", dial.zeros);
    println!("zero cross: {}", dial.zero_xings);

    Ok(())
}
// EOF
