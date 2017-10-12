use std::env;
use std::fs::File;
use std::io::{Read, Result};

#[derive(Default, Clone, Copy)]
struct Counter {
    a: u32,
    c: u32,
    g: u32,
    t: u32,
}

impl ToString for Counter {
    fn to_string(&self) -> String {
        format!("{} {} {} {}",
            self.a,
            self.c,
            self.g,
            self.t,
        )
    }
}

fn count_dna(dna: &[u8]) -> Counter {
    let mut counter = Counter::default();
    for nuc in dna {
        match *nuc {
            b'A' => counter.a += 1,
            b'C' => counter.c += 1,
            b'G' => counter.g += 1,
            b'T' => counter.t += 1,
            _ => panic!("Only carbon-based life is supported"),
        }
    }
    counter
}

fn main_io() -> Result<()> {
    let file_path = env::args().nth(1).expect("Input file required");
    let mut dna_file = File::open(&file_path)?;
    let mut dna = Vec::with_capacity(1000);
    dna_file.read_to_end(&mut dna)?;
    dna.pop(); // removes newline

    let cnt = count_dna(&dna); // removes newline
    println!("{}", cnt.to_string());
    Ok(())
}

fn main() {
    main_io().unwrap();
}
