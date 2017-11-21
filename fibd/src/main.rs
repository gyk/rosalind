use std::collections::VecDeque;
use std::io::{stdin, BufRead, Result};

struct Rabbits {
    generations: VecDeque<usize>,
    n_sexually_active: usize,
}

impl Rabbits {
    pub fn invade(lifespan: usize) -> Rabbits {
        assert!(lifespan > 1);
        let mut g = VecDeque::from(vec![0; lifespan]);
        g[0] = 1;

        Rabbits {
            generations: g,
            n_sexually_active: 0,
        }
    }
}

impl Iterator for Rabbits {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let n_newborn = *self.generations.front().unwrap_or(&0);
        let n_population = n_newborn + self.n_sexually_active;

        let n_mating = self.n_sexually_active;
        self.n_sexually_active += n_newborn;
        let death = self.generations.pop_back().expect("Unexpected Extinction");
        self.n_sexually_active -= death;
        self.generations.push_front(n_mating);

        Some(n_population)
    }
}

fn main_io() -> Result<()> {
    let stdin = stdin();
    let line = stdin.lock()
                    .lines()
                    .nth(0)
                    .unwrap()?;
    let inputs = line.split(' ')
                     .map(|x| x.parse::<usize>().unwrap())
                     .collect::<Vec<_>>();
    let n = inputs[0];
    let m = inputs[1];

    let mut rabbits = Rabbits::invade(m);
    println!("{}", rabbits.nth(n - 1).unwrap());

    Ok(())
}

fn main() {
    main_io().unwrap();
}
