use std::env;
use std::fs::File;
use std::io::{Read, Result};

struct Dna(Vec<u8>);
struct Rna(Vec<u8>);

fn transcribe(dna: Dna) -> Rna {
    let mut extracted_dna = dna.0;
    for nuc in extracted_dna.iter_mut() {
        if *nuc == b'T' {
            *nuc = b'U';
        }
    }

    let rna = extracted_dna;
    Rna(rna)
}

fn main_io() -> Result<()> {
    let file_path = env::args().nth(1).expect("Input file required");
    let mut dna_file = File::open(&file_path)?;
    let mut dna = Vec::with_capacity(1000);
    dna_file.read_to_end(&mut dna)?;
    dna.pop(); // removes newline

    let dna = Dna(dna);
    let rna = transcribe(dna);
    println!("{}", &String::from_utf8_lossy(&rna.0));
    Ok(())
}

fn main() {
    main_io().unwrap();
}
