use std::io::{stdin, BufRead, Result};

struct Rabbits {
    n_newborn: u64,
    n_sexually_active: u64,
    litter_size: u64,
}

impl Rabbits {
    pub fn invade(litter_size: u64) -> Rabbits {
        Rabbits {
            n_newborn: 1,
            n_sexually_active: 0,
            litter_size: litter_size,
        }
    }
}

impl Iterator for Rabbits {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let n_mating = self.n_sexually_active;
        self.n_sexually_active += self.n_newborn;
        self.n_newborn = n_mating * self.litter_size;
        Some(n_mating)
    }
}

fn main_io() -> Result<()> {
    let stdin = stdin();
    let line = stdin.lock()
                    .lines()
                    .nth(0)
                    .unwrap()?;
    let inputs = line.split(' ')
                     .map(|x| x.parse::<u64>().unwrap())
                     .collect::<Vec<_>>();
    let n = inputs[0];
    let k = inputs[1];

    let mut rabbits = Rabbits::invade(k);
    println!("{}", rabbits.nth(n as usize).unwrap());
    Ok(())
}

fn main() {
    main_io().unwrap();
}
