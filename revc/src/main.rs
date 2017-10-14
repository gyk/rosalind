use std::io::{stdin, BufRead, Result};

fn complement(dna: &[u8]) -> Vec<u8> {
    dna.iter()
        .rev()
        .map(|nuc| {
            match *nuc {
                b'A' => b'T',
                b'C' => b'G',
                b'G' => b'C',
                b'T' => b'A',
                _ => panic!("Welcome to Earth"),
            }
        })
        .collect()
}

fn main_io() -> Result<()> {
    let mut original_dna = String::with_capacity(1000);
    {
        let stdin = stdin();
        stdin.lock().read_line(&mut original_dna)?;
        original_dna.pop(); // removes newline
    }

    let gmo = complement(original_dna.as_bytes());
    println!("{}", &String::from_utf8_lossy(&gmo));
    Ok(())
}

fn main() {
    main_io().unwrap();
}
