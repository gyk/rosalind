use std::io::{stdin, BufRead, Result};

fn hamming(a: &[u8], b: &[u8]) -> usize {
    assert_eq!(a.len(), b.len());
    a.iter()
     .zip(b.iter())
     .filter(|&(aa ,bb)| aa != bb)
     .count()
}

fn main_io() -> Result<()> {
    let mut dna1 = String::with_capacity(1000);
    let mut dna2 = String::with_capacity(1000);
    {
        let stdin = stdin();
        stdin.lock().read_line(&mut dna1)?;
        stdin.lock().read_line(&mut dna2)?;
    }

    println!("{}", hamming(&dna1.as_bytes(),
                           &dna2.as_bytes()));
    Ok(())
}

fn main() {
    main_io().unwrap();
}
